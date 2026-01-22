// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql};
use postgres_parser::nodes::CreateCastStmt;

impl Sql for CreateCastStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE CAST");
        sql.push('(');
        sql.push_str(&self.sourcetype.sql());
        sql.push_str(" AS ");
        sql.push_str(&self.targettype.sql());
        sql.push(')');

        if self.func.is_some() {
            sql.push_str(&self.func.sql_prefix(" WITH FUNCTION "));
        } else {
            if self.inout {
                sql.push_str(" WITH INOUT ");
            } else {
                sql.push_str(" WITHOUT FUNCTION ");
            }
        }

        sql.push_str(&self.context.sql());

        sql
    }
}

impl Diff for CreateCastStmt {
    fn drop_stmt(&self) -> Option<String> {
        Some(format!(
            "DROP CAST IF EXISTS ({} AS {})",
            self.sourcetype.sql(),
            self.targettype.sql()
        ))
    }
}
