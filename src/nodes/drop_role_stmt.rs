use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::DropRoleStmt;

impl Sql for DropRoleStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("DROP ROLE ");
        if self.missing_ok {
            sql.push_str("IF EXISTS ");
        }
        sql.push_str(&self.roles.sql(", "));

        sql
    }
}

impl Diff for DropRoleStmt {}
