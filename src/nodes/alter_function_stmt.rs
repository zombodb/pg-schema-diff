// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::AlterFunctionStmt;

impl Sql for AlterFunctionStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("ALTER ");
        sql.push_str(&self.objtype.sql());
        sql.push_str(&self.func.sql_prefix(" "));
        sql.push_str(&self.actions.sql(", "));

        sql
    }
}

impl Diff for AlterFunctionStmt {
    fn drop_stmt(&self) -> Option<String> {
        None
    }
}
