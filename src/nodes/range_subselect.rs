// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::Sql;
use postgres_parser::nodes::RangeSubselect;

impl Sql for RangeSubselect {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if self.lateral {
            sql.push_str("LATERAL ");
        }
        sql.push_str(&self.subquery.sql_wrap("(", ")"));
        sql.push_str(&self.alias.sql());

        sql
    }
}
