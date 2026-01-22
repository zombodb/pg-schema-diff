// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Len, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::RuleStmt;

impl Sql for RuleStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE ");
        if self.replace {
            sql.push_str("OR REPLACE ");
        }
        sql.push_str("RULE ");

        sql.push_str(&self.rulename.sql_ident());
        sql.push_str(" AS ON ");
        sql.push_str(&self.event.sql());
        sql.push_str(" TO ");
        sql.push_str(&self.relation.sql());

        sql.push_str(&self.whereClause.sql_prefix(" WHERE "));
        sql.push_str(" DO ");
        if self.instead {
            sql.push_str("INSTEAD ");
        } else {
            sql.push_str("ALSO ");
        }

        if self.actions.is_none() {
            sql.push_str("NOTHING");
        } else if self.actions.len() == 1 {
            sql.push_str(&self.actions.sql(""));
        } else {
            sql.push_str(&self.actions.sql_wrap("; ", "(", ")"))
        }

        sql
    }
}

impl Diff for RuleStmt {}
