// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::Sql;
use postgres_parser::sys::ViewCheckOption;

impl Sql for ViewCheckOption {
    fn sql(&self) -> String {
        match self {
            ViewCheckOption::NO_CHECK_OPTION => "",
            ViewCheckOption::LOCAL_CHECK_OPTION => "LOCAL",
            ViewCheckOption::CASCADED_CHECK_OPTION => "CASCADED",
        }
        .into()
    }
}
