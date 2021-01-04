use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::ExplainStmt;

impl Sql for ExplainStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("EXPLAIN ");
        sql.push_str(&self.options.sql_wrap(", ", "(", ")"));
        sql.push_str(&self.query.sql_prefix(" "));

        sql
    }
}

impl Diff for ExplainStmt {}
