// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql};
use postgres_parser::nodes::CreateForeignTableStmt;

impl Sql for CreateForeignTableStmt {
    fn sql(&self) -> String {
        unimplemented!()
    }
}

impl Diff for CreateForeignTableStmt {}
