// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::VacuumStmt;

impl Sql for VacuumStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if self.is_vacuumcmd {
            sql.push_str("VACUUM ");
        } else {
            sql.push_str("ANALYZE ");
        }

        sql.push_str(&self.options.sql_wrap(", ", "(", ") "));
        sql.push_str(&self.rels.sql(", "));

        sql
    }
}

impl Diff for VacuumStmt {}
