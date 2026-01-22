// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlIdent};
use crate::EMPTY_NODE_VEC;
use postgres_parser::nodes::ColumnDef;

impl Sql for ColumnDef {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.colname.sql_ident());
        sql.push(' ');
        sql.push_str(&self.typeName.sql());

        sql.push_str(&self.collClause.sql());
        for constraint in self.constraints.as_ref().unwrap_or(&EMPTY_NODE_VEC) {
            sql.push(' ');
            sql.push_str(&constraint.sql());
        }

        sql
    }
}
