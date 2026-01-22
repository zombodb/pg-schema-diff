// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::DoStmt;

impl Sql for DoStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("DO ");
        sql.push_str(&self.args.sql(" "));

        sql
    }
}

impl Diff for DoStmt {
    fn drop_stmt(&self) -> Option<String> {
        None
    }
}
