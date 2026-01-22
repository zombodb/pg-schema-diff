// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlIdent, SqlList};
use postgres_parser::nodes::TypeName;
use postgres_parser::Node;

impl Sql for TypeName {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if self.setof {
            sql.push_str("SETOF ");
        }

        sql.push_str(&self.names.sql_ident());
        sql.push_str(&self.typmods.sql_wrap(",", "(", ")"));

        if let Some(array_bounds) = self.arrayBounds.as_ref() {
            for bound in array_bounds {
                if let Node::Value(bound) = bound {
                    let bound = bound.int.unwrap();
                    if bound == -1 {
                        sql.push_str("[]");
                    } else {
                        sql.push_str(&format!("[{}]", bound));
                    }
                }
            }
        }

        sql
    }
}
