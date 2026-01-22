// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlIdent};
use postgres_parser::nodes::UnlistenStmt;

impl Sql for UnlistenStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if self.conditionname.is_none() {
            sql.push_str("UNLISTEN *");
        } else {
            sql.push_str(&self.conditionname.sql_ident_prefix("UNLISTEN "));
        }

        sql
    }
}

impl Diff for UnlistenStmt {}
