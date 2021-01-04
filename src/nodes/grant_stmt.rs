use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::GrantStmt;
use postgres_parser::sys::GrantTargetType;
use postgres_parser::Node;

impl Sql for GrantStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(if self.is_grant { "GRANT " } else { "REVOKE " });
        if self.privileges.is_none() {
            sql.push_str("ALL PRIVILEGES")
        } else {
            sql.push_str(&self.privileges.sql(", "));
        }

        sql.push_str(" ON ");
        match self.targtype {
            GrantTargetType::ACL_TARGET_OBJECT => sql.push_str(&self.objtype.sql()),
            GrantTargetType::ACL_TARGET_ALL_IN_SCHEMA => {
                sql.push_str(&format!("ALL {}S IN SCHEMA", self.objtype.sql()))
            }
            GrantTargetType::ACL_TARGET_DEFAULTS => {
                unimplemented!("GrantTargetType::ACL_TARGET_DEFAULTS")
            }
        }

        sql.push(' ');

        sql.push_str(&self.objects.sql(", "));
        sql.push_str(" TO ");
        sql.push_str(&self.grantees.sql(", "));

        if self.grant_option {
            sql.push_str(" WITH GRANT OPTION");
        }

        sql
    }
}

impl Diff for GrantStmt {
    fn alter(&self, _other: &Node) -> Option<String> {
        None
    }

    fn drop(&self) -> String {
        unimplemented!()
    }

    fn name(&self, sql: &str) -> String {
        sql.into()
    }
}
