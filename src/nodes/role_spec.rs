// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlIdent};
use postgres_parser::nodes::RoleSpec;
use postgres_parser::sys::RoleSpecType;

impl Sql for RoleSpec {
    fn sql(&self) -> String {
        let mut sql = String::new();
        match self.roletype {
            RoleSpecType::ROLESPEC_CSTRING => sql.push_str(&self.rolename.sql_ident()),
            RoleSpecType::ROLESPEC_CURRENT_USER => sql.push_str("CURRENT_USER"),
            RoleSpecType::ROLESPEC_SESSION_USER => sql.push_str("SESSION_USER"),
            RoleSpecType::ROLESPEC_PUBLIC => sql.push_str("public"),
        }

        sql
    }
}
