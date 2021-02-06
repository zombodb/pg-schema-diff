use crate::schema_set::{Diff, Sql, SqlIdent};
use postgres_parser::nodes::FetchStmt;
use postgres_parser::sys::FetchDirection;

impl Sql for FetchStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("FETCH ");

        match self.direction {
            FetchDirection::FETCH_FORWARD => sql.push_str("FORWARD "),
            FetchDirection::FETCH_BACKWARD => sql.push_str("BACKWARD "),
            FetchDirection::FETCH_ABSOLUTE => sql.push_str("ABSOLUTE "),
            FetchDirection::FETCH_RELATIVE => sql.push_str("RELATIVE "),
        }

        if self.howMany == std::i64::MAX {
            sql.push_str("ALL");
        } else {
            sql.push_str(&self.howMany.to_string());
        }

        sql.push_str(" FROM ");
        sql.push_str(&self.portalname.sql_ident());

        sql
    }
}

impl Diff for FetchStmt {}
