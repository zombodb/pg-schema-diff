// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlIdent};
use postgres_parser::nodes::CreateAmStmt;

impl Sql for CreateAmStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE ACCESS METHOD ");
        sql.push_str(&self.amname.sql_ident());
        sql.push_str(" TYPE ");
        match self.amtype {
            't' => sql.push_str("TABLE"),
            'i' => sql.push_str("INDEX"),
            _ => panic!("unexpected CreateAmStmt::amtype"),
        }
        sql.push_str(" HANDLER ");
        sql.push_str(&self.handler_name.sql_ident());

        sql
    }
}

impl Diff for CreateAmStmt {
    fn object_name(&self) -> Option<String> {
        Some(self.amname.sql_ident())
    }

    fn object_type(&self) -> String {
        "ACCESS METHOD".into()
    }
}
