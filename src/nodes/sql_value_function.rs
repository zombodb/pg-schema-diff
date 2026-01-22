// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::Sql;
use postgres_parser::nodes::SQLValueFunction;
use postgres_parser::sys::SQLValueFunctionOp;

impl Sql for SQLValueFunction {
    fn sql(&self) -> String {
        match self.op {
            SQLValueFunctionOp::SVFOP_CURRENT_DATE => "current_date".into(),
            SQLValueFunctionOp::SVFOP_CURRENT_TIME | SQLValueFunctionOp::SVFOP_CURRENT_TIME_N => {
                "current_time".into()
            }
            SQLValueFunctionOp::SVFOP_CURRENT_TIMESTAMP
            | SQLValueFunctionOp::SVFOP_CURRENT_TIMESTAMP_N => "current_timestamp".into(),
            SQLValueFunctionOp::SVFOP_LOCALTIME | SQLValueFunctionOp::SVFOP_LOCALTIME_N => {
                "localtime".into()
            }
            SQLValueFunctionOp::SVFOP_LOCALTIMESTAMP
            | SQLValueFunctionOp::SVFOP_LOCALTIMESTAMP_N => "localtimestamp".into(),
            SQLValueFunctionOp::SVFOP_CURRENT_ROLE => "current_role".into(),
            SQLValueFunctionOp::SVFOP_CURRENT_USER => "current_user".into(),
            SQLValueFunctionOp::SVFOP_USER => "user".into(),
            SQLValueFunctionOp::SVFOP_SESSION_USER => "session_user".into(),
            SQLValueFunctionOp::SVFOP_CURRENT_CATALOG => "current_catalog".into(),
            SQLValueFunctionOp::SVFOP_CURRENT_SCHEMA => "current_schema".into(),
        }
    }
}
