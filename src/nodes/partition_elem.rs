// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlIdent};
use postgres_parser::nodes::PartitionElem;

impl Sql for PartitionElem {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.name.sql_ident());
        sql.push_str(&self.expr.sql());
        sql.push_str(&self.collation.sql_ident());
        sql.push_str(&self.opclass.sql_ident());

        sql
    }
}
