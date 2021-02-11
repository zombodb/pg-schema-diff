use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::GrantRoleStmt;
use postgres_parser::Node;

impl Sql for GrantRoleStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(if self.is_grant { "GRANT " } else { "REVOKE " });

        sql.push_str(&self.granted_roles.sql(", "));
        sql.push_str(" TO ");
        sql.push_str(&self.grantee_roles.sql(", "));

        if self.admin_opt {
            sql.push_str(" WITH ADMIN OPTION")
        }

        sql.push_str(&self.grantor.sql_prefix(" GRANTED BY "));

        sql
    }
}

impl Diff for GrantRoleStmt {}
