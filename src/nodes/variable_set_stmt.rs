use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::VariableSetStmt;
use postgres_parser::Node;

impl Sql for VariableSetStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.name.as_ref().unwrap());
        sql.push_str(" TO ");
        sql.push_str(&self.args.sql_wrap(None, None));

        sql
    }
}
