use crate::schema_set::{Diff, Sql};
use postgres_parser::nodes::CreateForeignServerStmt;

impl Sql for CreateForeignServerStmt {
    fn sql(&self) -> String {
        unimplemented!()
    }
}

impl Diff for CreateForeignServerStmt {}
