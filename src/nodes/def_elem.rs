use crate::schema_set::Sql;
use crate::{get_bool_value, get_string_value};
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
                sql.push_str(&format!("SET {}", self.arg.sql()));
            }
            "parallel" => sql.push_str(&format!("PARALLEL {}", self.arg.sql())),
            "cost" => sql.push_str(&format!("COST {}", self.arg.sql())),
            "rows" => sql.push_str(&format!("ROWS {}", self.arg.sql())),
            "start" => sql.push_str(&format!("START WITH {}", &self.arg.sql())),
            "as" => {
                // sql.push_str(&format!("AS {}", self.arg.sql()));
                if let Node::List(list) = self.arg.as_ref().unwrap().as_ref() {
                    sql.push_str("AS ");
                    sql.push_str(
                        &list
                            .iter()
                            .map(|e| {
                                format!(
                                    "'{}'",
                                    get_string_value(e)
                                        .as_ref()
                                        .expect("no string value for 'as'")
                                        .replace('\'', "''")
                                )
                            })
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                }
            }
            _ => unimplemented!("defname: {}", defname),
        }

        sql
    }
}
