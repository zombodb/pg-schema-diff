// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::Sql;
use postgres_parser::sys::DropBehavior;

impl Sql for DropBehavior {
    fn sql(&self) -> String {
        match self {
            DropBehavior::DROP_RESTRICT => "RESTRICT",
            DropBehavior::DROP_CASCADE => "CASCADE",
        }
        .into()
    }
}
