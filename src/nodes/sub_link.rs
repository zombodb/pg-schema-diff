// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::make_operator_name;
use crate::schema_set::{Sql, SqlIdent};
use postgres_parser::nodes::SubLink;
use postgres_parser::sys::SubLinkType;

impl Sql for SubLink {
    fn sql(&self) -> String {
        let mut sql = String::new();

        match self.subLinkType {
            SubLinkType::EXISTS_SUBLINK => {
                sql.push_str(&self.subselect.sql_wrap(" EXISTS (", ")"));
            }
            SubLinkType::ALL_SUBLINK => {
                sql.push_str(&self.testexpr.sql());
                sql.push(' ');
                sql.push_str(
                    &make_operator_name(&self.operName)
                        .expect("failed to make operator name for SubLink"),
                );
                sql.push_str(&self.subselect.sql_wrap(" ALL (", ")"));
            }
            SubLinkType::ANY_SUBLINK => {
                sql.push_str(&self.testexpr.sql());
                sql.push(' ');

                if self.operName.is_some() {
                    sql.push_str(&self.operName.sql_ident());
                    sql.push(' ');
                } else {
                    sql.push_str("IN ");
                }
                sql.push_str(&self.subselect.sql_wrap("(", ")"));
            }
            SubLinkType::ROWCOMPARE_SUBLINK => {
                sql.push_str(&self.testexpr.sql());
                sql.push(' ');
                sql.push_str(
                    &make_operator_name(&self.operName)
                        .expect("failed to make operator name for SubLink"),
                );
                sql.push_str(&self.subselect.sql_wrap("(", ")"));
            }
            SubLinkType::EXPR_SUBLINK => {
                sql.push_str(&self.subselect.sql_wrap("(", ")"));
            }
            SubLinkType::MULTIEXPR_SUBLINK => unimplemented!("SubLinkType::MULTIEXPR_SUBLINK"),
            SubLinkType::ARRAY_SUBLINK => {
                sql.push_str("array");
                sql.push_str(&self.subselect.sql_wrap("(", ")"));
            }
            SubLinkType::CTE_SUBLINK => unimplemented!("SubLinkType::CTE_SUBLINK"),
        }

        sql
    }
}
