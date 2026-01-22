// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::CreateSeqStmt;

impl Sql for CreateSeqStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        let is_temp = self.sequence.as_ref().unwrap().relpersistence == 't';
        sql.push_str("CREATE ");
        if is_temp {
            sql.push_str("TEMPORARY ");
        }
        sql.push_str("SEQUENCE ");
        if self.if_not_exists {
            sql.push_str("IF NOT EXISTS ");
        }
        sql.push_str(&self.sequence.sql());
        sql.push_str(&self.options.sql(" "));
        sql
    }
}

impl Diff for CreateSeqStmt {}
