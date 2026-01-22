// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::CreateOpClassItem;

impl Sql for CreateOpClassItem {
    fn sql(&self) -> String {
        let mut sql = String::new();

        match self.itemtype {
            1 => {
                sql.push_str("OPERATOR ");
                sql.push_str(&self.number.to_string());
                sql.push(' ');
                sql.push_str(&self.name.sql().replace('"', ""));
                sql.push(' ');
                sql.push_str(&self.order_family.sql(" "))
            }
            2 => {
                sql.push_str("FUNCTION ");
                sql.push_str(&self.number.to_string());
                sql.push(' ');
                sql.push_str(&self.name.sql());
            }
            3 => {
                sql.push_str("STORAGE ");
                sql.push_str(&self.storedtype.sql());
            }
            _ => panic!("unknoown CreateOptClassItem::itemtype: {}", self.itemtype),
        }

        sql
    }
}
