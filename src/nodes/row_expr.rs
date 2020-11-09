use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::RowExpr;

impl Sql for RowExpr {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("ROW ");
        sql.push_str(&self.args.sql_wrap(Some("("), Some(")")));

        sql
    }
}
