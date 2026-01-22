// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Diff, Sql, SqlIdent};
use postgres_parser::nodes::DropStmt;
use postgres_parser::sys::ObjectType;
use postgres_parser::Node;

impl Sql for DropStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("DROP ");
        sql.push_str(&self.removeType.sql());
        sql.push(' ');
        if self.concurrent {
            sql.push_str("CONCURRENTLY ");
        }
        if self.missing_ok {
            sql.push_str("IF EXISTS ");
        }

        match self.removeType {
            ObjectType::OBJECT_RULE => {
                let objects = self.objects.as_ref().unwrap();
                if let Node::List(objects) = objects.get(0).as_ref().unwrap() {
                    let tablename = objects.get(0).unwrap();
                    let rulename = objects.get(1).unwrap();
                    sql.push_str(&rulename.sql_ident());
                    sql.push_str(" ON ");
                    sql.push_str(&tablename.sql_ident());
                }
            }
            _ => {
                for (i, node) in self.objects.as_ref().unwrap().iter().enumerate() {
                    if i > 0 {
                        sql.push_str(", ");
                    }
                    if let Node::List(names) = node {
                        sql.push_str(&names.sql_ident());
                    } else if let Node::Value(_) = node {
                        sql.push_str(&node.sql_ident());
                    } else {
                        sql.push_str(&node.sql());
                    }
                }
            }
        }

        sql.push_str(&self.behavior.sql_prefix(" "));

        sql
    }
}

impl Diff for DropStmt {}
