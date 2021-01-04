use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::DropStmt;

impl Sql for DropStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("DROP ");
        sql.push_str(&self.removeType.sql());
        sql.push(' ');
        if self.concurrent {
            sql.push_str("CONCURRENTLY ");
        }
        if self.missing_ok {
            sql.push_str("IF EXISTS ");
        }
        sql.push_str(&self.objects.sql(", "));
        sql.push_str(&self.behavior.sql_prefix(" "));

        sql
    }
}

impl Diff for DropStmt {}
