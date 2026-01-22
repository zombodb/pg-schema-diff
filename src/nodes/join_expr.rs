// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::JoinExpr;
use postgres_parser::sys::JoinType;

impl Sql for JoinExpr {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.larg.sql());
        match self.jointype {
            JoinType::JOIN_INNER => sql.push_str(" INNER JOIN "),
            JoinType::JOIN_LEFT => sql.push_str(" LEFT JOIN "),
            JoinType::JOIN_FULL => sql.push_str(" FULL JOIN "),
            JoinType::JOIN_RIGHT => sql.push_str(" RIGHT JOIN "),
            JoinType::JOIN_SEMI => {}
            JoinType::JOIN_ANTI => {}
            JoinType::JOIN_UNIQUE_OUTER => {}
            JoinType::JOIN_UNIQUE_INNER => {}
        }
        sql.push_str(&self.rarg.sql());
        sql.push_str(&self.alias.sql());
        sql.push_str(
            &self
                .usingClause
                .sql_prefix_and_wrap(" USING ", "(", ")", ""),
        );
        sql.push_str(&self.quals.sql_prefix(" ON "));

        sql
    }
}
