use crate::schema_set::{Diff, Sql, SqlIdent};
use postgres_parser::nodes::NotifyStmt;

impl Sql for NotifyStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.conditionname.sql_ident_prefix("NOTIFY "));
        if self.payload.is_some() {
            sql.push_str(&format!(
                ", '{}'",
                self.payload.as_ref().unwrap().replace("'", "''")
            ));
        }

        sql
    }
}

impl Diff for NotifyStmt {}
