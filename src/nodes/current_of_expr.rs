// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlIdent};
use postgres_parser::nodes::CurrentOfExpr;

impl Sql for CurrentOfExpr {
    fn sql(&self) -> String {
        format!("CURRENT OF {}", self.cursor_name.sql_ident())
    }
}
