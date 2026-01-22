// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::Sql;
use postgres_parser::sys::CmdType;

impl Sql for CmdType {
    fn sql(&self) -> String {
        match self {
            CmdType::CMD_UNKNOWN => "UNKNOWN",
            CmdType::CMD_SELECT => "SELECT",
            CmdType::CMD_UPDATE => "UPDATE",
            CmdType::CMD_INSERT => "INSERT",
            CmdType::CMD_DELETE => "DELETE",
            CmdType::CMD_UTILITY => "UTILITY",
            CmdType::CMD_NOTHING => "NOTHING",
        }
        .into()
    }
}
