// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::Sql;
use postgres_parser::nodes::BooleanTest;
use postgres_parser::sys::BoolTestType;

impl Sql for BooleanTest {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.arg.sql());
        match self.booltesttype {
            BoolTestType::IS_TRUE => sql.push_str(" IS TRUE"),
            BoolTestType::IS_NOT_TRUE => sql.push_str(" IS NOT TRUE"),
            BoolTestType::IS_FALSE => sql.push_str(" IS FALSE"),
            BoolTestType::IS_NOT_FALSE => sql.push_str(" IS NOT FALSE"),
            BoolTestType::IS_UNKNOWN => sql.push_str(" IS UNKNOWN"),
            BoolTestType::IS_NOT_UNKNOWN => sql.push_str(" IS NOT UNKNOWN"),
        }

        sql
    }
}
