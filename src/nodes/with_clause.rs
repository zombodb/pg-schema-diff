use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::WithClause;

impl Sql for WithClause {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("WITH ");
        if self.recursive {
            sql.push_str("RECURSIVE ");
        }
        sql.push_str(&self.ctes.sql_wrap(None, None));

        sql
    }
}
