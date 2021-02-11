use crate::schema_set::{Diff, Sql, SqlIdent};
use postgres_parser::nodes::VariableShowStmt;

impl Sql for VariableShowStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("SHOW ");
        sql.push_str(&self.name.sql_ident());

        sql
    }
}

impl Diff for VariableShowStmt {}
