use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::CreateRoleStmt;
use postgres_parser::sys::RoleStmtType;

impl Sql for CreateRoleStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE ");
        match self.stmt_type {
            RoleStmtType::ROLESTMT_ROLE => sql.push_str("ROLE "),
            RoleStmtType::ROLESTMT_USER => sql.push_str("USER "),
            RoleStmtType::ROLESTMT_GROUP => sql.push_str("GROUP "),
        }
        sql.push_str(&self.role.sql_ident());
        sql.push_str(&self.options.sql_prefix(" WITH ", " "));

        sql
    }
}

impl Diff for CreateRoleStmt {}
