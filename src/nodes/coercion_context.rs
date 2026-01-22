// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::Sql;
use postgres_parser::sys::CoercionContext;

impl Sql for CoercionContext {
    fn sql(&self) -> String {
        match self {
            CoercionContext::COERCION_IMPLICIT => " AS IMPLICIT",
            CoercionContext::COERCION_ASSIGNMENT => " AS ASSIGNMENT",
            CoercionContext::COERCION_EXPLICIT => "",
        }
        .into()
    }
}
