use crate::schema_set::Sql;
use crate::{get_bool_value, get_float_value, get_int_value, get_string_value};
use postgres_parser::nodes::DefElem;
use postgres_parser::Node;

impl Sql for DefElem {
    fn sql(&self) -> String {
        let mut sql = String::new();

        let defname = self.defname.as_ref().unwrap();

        match defname.as_str() {
            "language" => {
                sql.push_str(&format!(
                    "LANGUAGE {}",
                    get_string_value(&self.arg.as_ref().unwrap())
                        .as_ref()
                        .expect("no string value for 'language'")
                ));
            }
            "volatility" => {
                sql.push_str(&format!(
                    "{}",
                    get_string_value(&self.arg.as_ref().unwrap())
                        .as_ref()
                        .expect("no string value for 'volatility'")
                ));
            }
            "strict" => {
                if get_bool_value(&self.arg.as_ref().unwrap()) {
                    sql.push_str("STRICT");
                }
            }
            "set" => {
                sql.push_str(&format!(
                    "SET {}",
                    self.arg
                        .as_ref()
                        .expect("no 'arg' for DefElement SET")
                        .as_ref()
                        .sql()
                ));
            }
            "parallel" => sql.push_str(&format!(
                "PARALLEL {}",
                get_string_value(&self.arg.as_ref().unwrap())
                    .as_ref()
                    .expect("no value for 'parallel")
            )),
            "cost" => sql.push_str(&format!(
                "COST {}",
                get_float_value(&self.arg.as_ref().unwrap())
                    .as_ref()
                    .expect("no value for 'cost'")
            )),
            "rows" => sql.push_str(&format!(
                "ROWS {}",
                get_int_value(&self.arg.as_ref().unwrap())
                    .as_ref()
                    .expect("no value for 'rows'")
            )),
            "as" => {
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
