// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlIdent, SqlList};
use postgres_parser::nodes::OnConflictClause;
use postgres_parser::sys::OnConflictAction;
use postgres_parser::Node;

impl Sql for OnConflictClause {
    fn sql(&self) -> String {
        match self.action {
            OnConflictAction::ONCONFLICT_NONE => "".into(),
            OnConflictAction::ONCONFLICT_NOTHING | OnConflictAction::ONCONFLICT_UPDATE => {
                let mut sql = String::new();

                sql.push_str(" ON CONFLICT ");
                sql.push_str(&self.infer.sql());

                if self.whereClause.is_some() {
                    unimplemented!("don't know how to handle 'whereClause' for OnConflictClause");
                }

                match self.action {
                    OnConflictAction::ONCONFLICT_NOTHING => sql.push_str(" DO NOTHING"),
                    OnConflictAction::ONCONFLICT_UPDATE => sql.push_str(" DO UPDATE"),
                    _ => {}
                }

                if let Some(target_list) = &self.targetList {
                    sql.push_str(" SET ");
                    let mut i = 0;
                    for node in target_list {
                        match node {
                            Node::ResTarget(res_target) => {
                                if i > 0 {
                                    sql.push_str(", ");
                                }
                                sql.push_str(&res_target.name.sql_ident());
                                sql.push_str(
                                    &res_target.indirection.sql_wrap_each(Some("["), Some("]")),
                                );
                                sql.push_str(" = ");
                                sql.push_str(&res_target.val.sql());
                                i += 1;
                            }

                            _ => panic!("unexpected node in 'targetList' of OnConflictClause"),
                        }
                    }
                }

                if self.whereClause.is_some() {
                    sql.push_str(" WHERE ");
                    sql.push_str(&self.whereClause.sql());
                }

                sql
            }
        }
    }
}
