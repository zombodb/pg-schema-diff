use crate::schema_set::{Sql, SqlIdent};
use postgres_parser::nodes::RoleSpec;
use postgres_parser::sys::RoleSpecType;

impl Sql for RoleSpec {
    fn sql(&self) -> String {
        let mut sql = String::new();
        match self.roletype {
            RoleSpecType::ROLESPEC_CSTRING => sql.push_str(&self.rolename.sql_ident()),
            RoleSpecType::ROLESPEC_CURRENT_USER => sql.push_str("CURRENT_USER"),
            RoleSpecType::ROLESPEC_SESSION_USER => sql.push_str("SESSION_USER"),
            RoleSpecType::ROLESPEC_PUBLIC => sql.push_str("public"),
        }

        sql
    }
}
