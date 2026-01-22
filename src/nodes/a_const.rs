// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::Sql;
use postgres_parser::nodes::A_Const;

impl Sql for A_Const {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if let Some(i) = self.val.int.as_ref() {
            sql.push_str(&i.to_string());
        } else if let Some(s) = self.val.string.as_ref() {
            sql.push('\'');
            sql.push_str(&s.replace('\'', "''"));
            sql.push('\'');
        } else if let Some(_) = self.val.null.as_ref() {
            sql.push_str("NULL");
        } else if let Some(f) = self.val.float.as_ref() {
            sql.push_str(f);
        } else if let Some(b) = self.val.bit_string.as_ref() {
            let bitstring = &b[1..];
            sql.push('B');
            sql.push('\'');
            sql.push_str(bitstring);
            sql.push('\'');
        }

        sql
    }
}
