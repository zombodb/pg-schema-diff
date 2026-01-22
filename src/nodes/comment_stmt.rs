// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlMaybeList};
use postgres_parser::nodes::CommentStmt;

impl Sql for CommentStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("COMMENT ON ");
        sql.push_str(&self.objtype.sql());
        sql.push(' ');
        sql.push_str(&self.object.sql_maybe_list("."));
        sql.push_str(" IS ");

        if self.comment.is_some() {
            sql.push_str("'");
            sql.push_str(&self.comment.as_ref().unwrap().replace("'", "''"));
            sql.push_str("'");
        } else {
            sql.push_str("NULL");
        }

        sql
    }
}

impl Diff for CommentStmt {}
