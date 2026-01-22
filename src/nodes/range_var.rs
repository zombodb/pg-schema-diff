// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlIdent};
use postgres_parser::nodes::RangeVar;

impl Sql for RangeVar {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if !self.inh {
            sql.push_str("ONLY ");
        }
        sql.push_str(&self.catalogname.sql_ident_suffix("."));
        sql.push_str(&self.schemaname.sql_ident_suffix("."));
        sql.push_str(&self.relname.sql_ident());
        sql.push_str(&self.alias.sql());

        sql
    }
}
