use crate::schema_set::Sql;
use crate::{get_bool_value, make_name};
use postgres_parser::nodes::DefElem;
use postgres_parser::Node;

impl Sql for DefElem {
    fn sql(&self) -> String {
        let mut sql = String::new();

        let defname = self.defname.as_ref().unwrap();

        match defname.as_str() {
            "cache" => sql.push_str(&format!("CACHE {}", self.arg.sql())),
            "cycle" => {
                if get_bool_value(&self.arg.as_ref().unwrap()) {
                    sql.push_str("CYCLE");
                } else {
                    sql.push_str("NO CYCLE");
                }
            }
            "language" => {
                sql.push_str(&format!("LANGUAGE {}", self.arg.sql()));
            }
            "volatility" => {
                sql.push_str(&self.arg.sql());
            }
            "strict" => {
                if get_bool_value(&self.arg.as_ref().unwrap()) {
                    sql.push_str("STRICT");
                }
            }
            "set" => {
                sql.push_str(&self.arg.sql());
            }
            "parallel" => sql.push_str(&format!(
                "PARALLEL{}{}",
                separator(&self.arg.as_ref().unwrap()),
                self.arg.sql()
            )),
            "cost" => sql.push_str(&format!("COST {}", self.arg.sql())),
            "rows" => sql.push_str(&format!("ROWS {}", self.arg.sql())),
            "start" => sql.push_str(&format!("START WITH {}", &self.arg.sql())),
            "procedure" => sql.push_str(&format!("PROCEDURE = {}", self.arg.sql())),
            "restrict" => sql.push_str(&format!("RESTRICT = {}", self.arg.sql())),
            "leftarg" => sql.push_str(&format!("LEFTARG = {}", self.arg.sql())),
            "rightarg" => sql.push_str(&format!("RIGHTARG = {}", self.arg.sql())),

            "null" => {
                if self.arg.is_some() {
                    sql.push_str(&format!("NULL {}", scalar(self.arg.sql())))
                } else {
                    sql.push_str("NULL ''");
                }
            }

            "costs" => sql.push_str(&format!("COSTS {}", scalar(self.arg.sql()))),

            "stype" => sql.push_str(&format!("STYPE = {}", self.arg.sql())),
            "sfunc" => sql.push_str(&format!("SFUNC = {}", self.arg.sql())),
            "finalfunc" => sql.push_str(&format!("FINALFUNC = {}", self.arg.sql())),
            "initcond" => sql.push_str(&format!("INITCOND = '{}'", self.arg.sql())),
            "transaction_isolation" => {
                sql.push_str("ISOLATION LEVEL ");
                match self.arg.as_ref().unwrap().as_ref() {
                    Node::A_Const(a_const) => sql.push_str(&a_const.val.sql().to_uppercase()),
                    _ => panic!("unsupported arg node type for transaction_isolation"),
                }
            }
            "transaction_deferrable" => match self.arg.as_ref().unwrap().as_ref() {
                Node::A_Const(a_const) => {
                    if a_const.val.int.unwrap_or(1) == 1 {
                        sql.push_str(" DEFERRABLE");
                    } else {
                        sql.push_str(" NOT DEFERRABLE");
                    }
                }
                _ => panic!("unsupported arg node type for transaction_deferrable"),
            },
            "autovacuum_enabled" => {
                sql.push_str(&format!("autovacuum_enabled = {}", self.arg.sql()))
            }
            "deduplicate_items" => sql.push_str(&format!("deduplicate_items = {}", self.arg.sql())),

            "as" if self.arg.is_some() => {
                let unboxed = self.arg.as_ref().unwrap();
                if let Node::Value(value) = unboxed.as_ref() {
                    sql.push_str(&quote(value.sql()));
                } else if let Node::List(list) = unboxed.as_ref() {
                    sql.push_str("AS ");
                    for (i, node) in list.iter().enumerate() {
                        if i > 0 {
                            sql.push_str(", ");
                        }
                        sql.push_str(&quote(node.sql()));
                    }
                } else {
                    sql.push_str(&self.arg.sql());
                }
            }

            "from" => {
                if let Node::List(list) = self.arg.as_ref().unwrap().as_ref() {
                    sql.push_str(&format!("FROM {}", make_name(&Some(list.clone())).unwrap()));
                } else {
                    panic!("unexpected node type for 'from' DefElem")
                }
            }

            "delimiter" => sql.push_str(&format!(
                "delimiter '{}'",
                self.arg.sql().replace("'", "''")
            )),

            "provider" => sql.push_str(&format!("provider = {}", self.arg.sql())),
            "collation" => sql.push_str(&format!("collation = {}", self.arg.sql())),
            "subtype" => sql.push_str(&format!("subtype = {}", self.arg.sql())),
            "locale" => sql.push_str(&format!("locale = '{}'", self.arg.sql())),
            "lc_ctype" => sql.push_str(&format!("lc_ctype = {}", self.arg.sql())),
            "lc_collate" => sql.push_str(&format!("lc_collate = {}", self.arg.sql())),

            "createdb" => {
                if let Node::Value(value) = self.arg.as_ref().unwrap().as_ref() {
                    if value.int.unwrap_or_default() == 0 {
                        sql.push_str("NOCREATEDB");
                    } else {
                        sql.push_str("CREATEDB");
                    }
                }
            }
            "createrole" => {
                if let Node::Value(value) = self.arg.as_ref().unwrap().as_ref() {
                    if value.int.unwrap_or_default() == 0 {
                        sql.push_str("NOCREATEROLE");
                    } else {
                        sql.push_str("CREATEROLE");
                    }
                }
            }

            key => {
                if self.arg.is_some() {
                    let arg_sql = self.arg.sql();

                    let uppercase_cnt = key.chars().filter(|c| c.is_uppercase()).count();

                    let key = if uppercase_cnt > 0 {
                        format!("\"{}\"", key)
                    } else {
                        key.into()
                    };

                    if arg_sql.starts_with('"') && arg_sql.ends_with('"') {
                        sql.push_str(&format!("{} = {}", key, arg_sql))
                    } else {
                        sql.push_str(&format!("{} = {}", key, scalar(arg_sql)))
                    }
                } else {
                    sql.push_str(key)
                }
            }
        }

        sql
    }
}

fn quote(input: String) -> String {
    if input.contains('\'') {
        if input.contains("$$") {
            if input.contains("$_pgsdq$") {
                // just something random
                let quote: [char; 15] = rand::random();
                let quote: String = quote.iter().collect();
                format!("${}${}${}$", quote, input, quote)
            } else {
                format!("$_pgsdq${}$_pgsdq$", input)
            }
        } else {
            format!("$${}$$", input)
        }
    } else {
        format!("'{}'", input)
    }
}

fn scalar(input: String) -> String {
    for c in input.chars() {
        if !['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-'].contains(&c) {
            return format!("'{}'", input.replace('\'', "''"));
        }
    }

    input
}

fn separator(input: &Node) -> &'static str {
    if let Node::Value(_) = input {
        " "
    } else if let Node::TypeName(_) = input {
        " = "
    } else {
        panic!("cannot determine separator")
    }
}
