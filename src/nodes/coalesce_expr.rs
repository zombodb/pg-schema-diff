// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::CoalesceExpr;

impl Sql for CoalesceExpr {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("COALESCE (");
        sql.push_str(&self.args.sql(", "));
        sql.push(')');

        sql
    }
}
