// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Len, Sql};

use postgres_parser::nodes::A_Indirection;
use postgres_parser::Node;

impl Sql for A_Indirection {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push('(');
        sql.push_str(&self.arg.sql());
        sql.push_str(&indirection_list(&self.indirection));
        sql.push(')');
        sql
    }
}

pub fn indirection_list(list: &Option<Vec<Node>>) -> String {
    let mut sql = String::new();

    if list.len() > 0 {
        for indirection in list.as_ref().unwrap() {
            match indirection {
                Node::A_Indices(a_indieces) => sql.push_str(&format!("[{}]", a_indieces.sql())),
                Node::Value(value) => sql.push_str(&format!(".{}", value.sql())),
                _ => unimplemented!("unsupported A_Indirection::indirection node type"),
            }
        }
    }
    sql
}
