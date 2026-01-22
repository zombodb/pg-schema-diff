// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::CompositeTypeStmt;

impl Sql for CompositeTypeStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE TYPE ");

        // we don't want .inh=false b/c that'll cause the RangeVar to output an "ONLY" and that's
        // not a thing here
        let mut typevar = self.typevar.clone();
        typevar.as_mut().unwrap().inh = true;
        sql.push_str(&typevar.sql());

        sql.push_str(" AS ");
        sql.push_str(&self.coldeflist.sql_wrap(", ", "(", ")"));
        sql
    }
}

impl Diff for CompositeTypeStmt {}
