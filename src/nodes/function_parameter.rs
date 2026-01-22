// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlIdent};
use postgres_parser::nodes::FunctionParameter;
use postgres_parser::sys::FunctionParameterMode;

impl Sql for FunctionParameter {
    fn sql(&self) -> String {
        let mut sql = String::new();

        match self.mode {
            FunctionParameterMode::FUNC_PARAM_IN => { /* IN is the default */ }
            FunctionParameterMode::FUNC_PARAM_INOUT => sql.push_str("INOUT "),
            FunctionParameterMode::FUNC_PARAM_OUT => sql.push_str("OUT "),
            FunctionParameterMode::FUNC_PARAM_VARIADIC => sql.push_str("VARIADIC "),
            FunctionParameterMode::FUNC_PARAM_TABLE => { /* do nothing */ }
        }

        sql.push_str(&self.name.sql_ident_suffix(" "));
        sql.push_str(&self.argType.sql());
        if let Some(default) = &self.defexpr {
            let default_value = default.sql();
            if default_value == "(('t')::bool)" || default_value == "(('t')::pg_catalog.bool)" {
                sql.push_str(" DEFAULT 'true'");
            } else if default_value == "(('f')::bool)"
                || default_value == "(('f')::pg_catalog.bool)"
            {
                sql.push_str(" DEFAULT 'false'");
            } else if default_value == "NULL" {
                sql.push_str(" DEFAULT NULL");
            } else if default_value.starts_with("'") && default_value.ends_with("'") {
                sql.push_str(&self.defexpr.sql_prefix(" DEFAULT "));
            } else {
                sql.push_str(&format!(" DEFAULT '{}'", default_value));
            }
        }

        sql
    }
}
