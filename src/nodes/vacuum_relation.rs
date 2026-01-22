// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::VacuumRelation;

impl Sql for VacuumRelation {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.relation.sql());
        sql.push_str(&self.va_cols.sql_wrap(", ", "(", ")"));

        sql
    }
}
