// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::CaseExpr;

impl Sql for CaseExpr {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CASE");

        if self.arg.is_some() {
            sql.push(' ');
            sql.push_str(&self.arg.sql());
        }

        sql.push(' ');
        sql.push_str(&self.args.sql(" "));

        sql.push_str(&self.defresult.sql_prefix(" ELSE "));
        sql.push_str(" END");

        sql
    }
}
