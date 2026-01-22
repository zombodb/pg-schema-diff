// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlIdent};
use postgres_parser::nodes::CreateConversionStmt;

impl Sql for CreateConversionStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE ");
        if self.def {
            sql.push_str("DEFAULT ");
        }
        sql.push_str("CONVERSION ");
        sql.push_str(&self.conversion_name.sql_ident());
        sql.push_str(" FOR '");
        sql.push_str(&self.for_encoding_name.as_ref().unwrap());
        sql.push_str("' TO '");
        sql.push_str(&self.to_encoding_name.as_ref().unwrap());
        sql.push_str("' FROM ");
        sql.push_str(&self.func_name.sql_ident());

        sql
    }
}

impl Diff for CreateConversionStmt {}
