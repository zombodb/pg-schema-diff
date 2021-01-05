use crate::get_bool_value;
use crate::schema_set::Sql;
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
            "parallel" => sql.push_str(&format!("PARALLEL {}", self.arg.sql())),
            "cost" => sql.push_str(&format!("COST {}", self.arg.sql())),
            "rows" => sql.push_str(&format!("ROWS {}", self.arg.sql())),
            "start" => sql.push_str(&format!("START WITH {}", &self.arg.sql())),
            "procedure" => sql.push_str(&format!("PROCEDURE = {}", self.arg.sql())),
            "restrict" => sql.push_str(&format!("RESTRICT = {}", self.arg.sql())),
            "leftarg" => sql.push_str(&format!("LEFTARG = {}", self.arg.sql())),
            "rightarg" => sql.push_str(&format!("RIGHTARG = {}", self.arg.sql())),

            "null" => sql.push_str(&format!("NULL {}", scalar(self.arg.sql()))),

            "costs" => sql.push_str(&format!("COSTS {}", scalar(self.arg.sql()))),

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
            key => {
                if self.arg.is_some() {
                    sql.push_str(&format!("{} = {}", key, scalar(self.arg.sql())))
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
