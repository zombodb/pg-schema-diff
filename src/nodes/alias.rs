use crate::schema_set::{Sql, SqlIdent};
use postgres_parser::nodes::Alias;

impl Sql for Alias {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(" AS ");
        sql.push_str(&self.aliasname.sql());

        sql
    }
}
