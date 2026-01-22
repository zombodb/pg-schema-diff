// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlIdent};
use postgres_parser::nodes::ListenStmt;

impl Sql for ListenStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("LISTEN ");
        sql.push_str(&self.conditionname.sql_ident());

        sql
    }
}

impl Diff for ListenStmt {}
