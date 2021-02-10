use crate::schema_set::{Diff, Sql};
use postgres_parser::nodes::CreateForeignTableStmt;

impl Sql for CreateForeignTableStmt {
    fn sql(&self) -> String {
        unimplemented!()
    }
}

impl Diff for CreateForeignTableStmt {}
