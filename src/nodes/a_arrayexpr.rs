use crate::schema_set::{Sql, SqlList};

use postgres_parser::nodes::A_ArrayExpr;

impl Sql for A_ArrayExpr {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("ARRAY[");
        sql.push_str(&self.elements.sql_wrap(None, None));
        sql.push(']');

        sql
    }
}
