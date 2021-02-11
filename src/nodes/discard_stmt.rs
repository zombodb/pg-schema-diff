use crate::schema_set::{Diff, Sql};
use postgres_parser::nodes::DiscardStmt;
use postgres_parser::sys::DiscardMode;
use postgres_parser::Node;

impl Sql for DiscardMode {
    fn sql(&self) -> String {
        match self {
            DiscardMode::DISCARD_ALL => "ALL",
            DiscardMode::DISCARD_PLANS => "PLANS",
            DiscardMode::DISCARD_SEQUENCES => "SEQUENCES",
            DiscardMode::DISCARD_TEMP => "TEMP",
        }
        .into()
    }
}

impl Sql for DiscardStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("DISCARD ");
        sql.push_str(&self.target.sql());

        sql
    }
}

impl Diff for DiscardStmt {}
