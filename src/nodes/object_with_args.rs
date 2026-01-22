// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlIdent, SqlList};
use postgres_parser::nodes::ObjectWithArgs;

impl Sql for ObjectWithArgs {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.objname.sql_ident());

        if !self.args_unspecified && self.objargs.is_none() {
            // noop
        } else {
            if !self.args_unspecified {
                sql.push('(');
                if self.objargs.is_none() {
                    sql.push('*');
                } else {
                    sql.push_str(&self.objargs.sql(", "));
                }
                sql.push(')');
            }
        }

        sql
    }
}
