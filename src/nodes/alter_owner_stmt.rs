// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlMaybeList};
use postgres_parser::nodes::AlterOwnerStmt;

impl Sql for AlterOwnerStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("ALTER ");
        sql.push_str(&self.objectType.sql());
        sql.push(' ');
        sql.push_str(&self.object.sql_maybe_list("."));
        sql.push_str(" OWNER TO ");
        sql.push_str(&self.newowner.sql());

        sql
    }
}

impl Diff for AlterOwnerStmt {}
