// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::Sql;
use postgres_parser::sys::SortByDir;

impl Sql for SortByDir {
    fn sql(&self) -> String {
        match self {
            SortByDir::SORTBY_DEFAULT => "",
            SortByDir::SORTBY_ASC => "ASC",
            SortByDir::SORTBY_DESC => "DESC",
            SortByDir::SORTBY_USING => unimplemented!("SortByDir::SORTBY_USING"),
        }
        .into()
    }
}
