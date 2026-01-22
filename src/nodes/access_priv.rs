// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Len, Sql, SqlIdent};
use postgres_parser::nodes::AccessPriv;

impl Sql for AccessPriv {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if self.priv_name.is_none() {
            sql.push_str("ALL PRIVILEGES");
        } else {
            sql.push_str(&self.priv_name.sql_ident());
        }

        if self.cols.len() > 0 {
            sql.push('(');
            for (i, col) in self.cols.as_ref().unwrap().iter().enumerate() {
                if i > 0 {
                    sql.push_str(", ");
                }
                sql.push_str(&col.sql());
            }
            sql.push(')');
        }

        sql
    }
}
