use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::WindowDef;

impl Sql for WindowDef {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push('(');
        sql.push_str(&self.orderClause.sql_prefix("ORDER BY ", ","));
        sql.push(')');

        sql
    }
}
