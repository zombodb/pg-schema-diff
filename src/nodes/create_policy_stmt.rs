use crate::schema_set::{Diff, Sql};
use postgres_parser::nodes::CreatePolicyStmt;

impl Sql for CreatePolicyStmt {
    fn sql(&self) -> String {
        unimplemented!()
    }
}

impl Diff for CreatePolicyStmt {}
