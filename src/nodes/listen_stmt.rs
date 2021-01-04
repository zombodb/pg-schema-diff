use crate::schema_set::{Diff, Sql};
use postgres_parser::nodes::ListenStmt;

impl Sql for ListenStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("LISTEN ");
        sql.push_str(&self.conditionname.as_ref().unwrap());

        sql
    }
}

impl Diff for ListenStmt {}
