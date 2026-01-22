// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql};
use postgres_parser::nodes::CreateForeignServerStmt;

impl Sql for CreateForeignServerStmt {
    fn sql(&self) -> String {
        unimplemented!()
    }
}

impl Diff for CreateForeignServerStmt {}
