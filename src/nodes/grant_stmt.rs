// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::GrantStmt;
use postgres_parser::sys::GrantTargetType;

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

impl Diff for GrantStmt {}
