// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlIdent, SqlList};
use postgres_parser::nodes::CommonTableExpr;

impl Sql for CommonTableExpr {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.ctename.sql_ident());
        sql.push_str(&self.aliascolnames.sql_wrap(", ", "(", ")"));
        sql.push_str(" AS (");
        sql.push_str(&self.ctequery.sql());
        sql.push_str(") ");

        sql
    }
}
