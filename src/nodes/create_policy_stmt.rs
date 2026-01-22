// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::CreatePolicyStmt;

impl Sql for CreatePolicyStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE POLICY ");
        sql.push_str(&self.policy_name.sql_ident());
        sql.push_str(" ON ");
        sql.push_str(&self.table.sql());
        if self.permissive {
            sql.push_str(" AS PERMISSIVE ");
        } else {
            sql.push_str(" AS RESTRICTIVE ");
        }

        if self.cmd_name.is_none() {
            sql.push_str("FOR ALL ");
        } else {
            sql.push_str("FOR ");
            sql.push_str(&self.cmd_name.as_ref().unwrap().to_uppercase());
        }

        sql.push_str(&self.roles.sql_prefix(" TO ", ", "));
        sql.push_str(&self.qual.sql_prefix_and_wrap(" USING ", "(", ")"));
        sql.push_str(
            &self
                .with_check
                .sql_prefix_and_wrap(" WITH CHECK ", "(", ")"),
        );

        sql
    }
}

impl Diff for CreatePolicyStmt {}
