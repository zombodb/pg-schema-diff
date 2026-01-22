// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::CreateEventTrigStmt;

impl Sql for CreateEventTrigStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE EVENT TRIGGER ");
        sql.push_str(&self.trigname.sql_ident());
        sql.push_str(" ON ");
        sql.push_str(&self.eventname.sql_ident());

        if let Some(when) = &self.whenclause {
            sql.push_str(" WHEN ");
            sql.push_str(&when.sql(" AND "));
        }

        sql.push_str("EXECUTE FUNCTION ");
        sql.push_str(&self.funcname.sql_ident());
        sql.push_str("()");
        sql
    }
}

impl Diff for CreateEventTrigStmt {
    fn object_name(&self) -> Option<String> {
        Some(self.trigname.sql_ident())
    }

    fn object_type(&self) -> String {
        "EVENT TRIGGER".into()
    }
}
