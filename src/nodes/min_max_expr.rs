// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::MinMaxExpr;
use postgres_parser::sys::MinMaxOp;

impl Sql for MinMaxExpr {
    fn sql(&self) -> String {
        match self.op {
            MinMaxOp::IS_GREATEST => format!("greatest({})", self.args.sql(", ")),
            MinMaxOp::IS_LEAST => format!("least({})", self.args.sql(", ")),
        }
    }
}
