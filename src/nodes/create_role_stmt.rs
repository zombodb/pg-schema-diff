use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::CreateRoleStmt;

impl Sql for CreateRoleStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE ROLE ");
        sql.push_str(&self.role.sql_ident());
        sql.push_str(&self.options.sql_prefix(" WITH ", " "));

        sql
    }
}

impl Diff for CreateRoleStmt {}
