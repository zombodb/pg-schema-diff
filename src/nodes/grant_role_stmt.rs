// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::GrantRoleStmt;

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
