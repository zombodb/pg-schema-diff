// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::Sql;
use postgres_parser::sys::OnCommitAction;

impl Sql for OnCommitAction {
    fn sql(&self) -> String {
        match self {
            OnCommitAction::ONCOMMIT_NOOP => "",
            OnCommitAction::ONCOMMIT_PRESERVE_ROWS => "ON COMMIT PRESERVE ROWS ",
            OnCommitAction::ONCOMMIT_DELETE_ROWS => "ON COMMIT DELETE ROWS ",
            OnCommitAction::ONCOMMIT_DROP => "ON COMMIT DROP ",
        }
        .into()
    }
}
