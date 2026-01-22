// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::AlterTypeStmt;

impl Sql for AlterTypeStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if let Some(options) = &self.options {
            sql.push_str("ALTER TYPE ");
            sql.push_str(&self.typeName.sql("."));
            sql.push_str(" ");
            sql.push_str("SET (");
            sql.push_str(&options.sql(","));
            sql.push_str(")");
        }

        sql
    }
}

impl Diff for AlterTypeStmt {
    fn drop_stmt(&self) -> Option<String> {
        None
    }
}
