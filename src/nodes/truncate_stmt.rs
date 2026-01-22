// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::TruncateStmt;

impl Sql for TruncateStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("TRUNCATE TABLE ");
        sql.push_str(&self.relations.sql(", "));
        if self.restart_seqs {
            sql.push_str("RESTART IDENTITY");
        }
        sql.push(' ');
        sql.push_str(&self.behavior.sql());
        sql
    }
}

impl Diff for TruncateStmt {}
