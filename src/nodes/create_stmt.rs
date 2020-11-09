use crate::schema_set::{Diff, Sql, SqlList};
use crate::EMPTY_NODE_VEC;
use postgres_parser::nodes::CreateStmt;
use postgres_parser::Node;

impl Diff for CreateStmt {
    fn alter(&self, _other: &Node) -> Option<String> {
        unimplemented!()
    }

    fn drop(&self) -> String {
        unimplemented!()
    }

    fn name(&self, _: &str) -> String {
        self.relation.as_ref().unwrap().sql()
    }
}

impl Sql for CreateStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE TABLE ");
        sql.push_str(
            &self
                .relation
                .as_ref()
                .expect("no 'relation' for CreateStmt")
                .sql(),
        );
        sql.push_str(&self.tableElts.sql_wrap(Some("("), Some(")")));

        sql
    }
}
