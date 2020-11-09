use crate::nodes::res_target::{res_target_insert, res_target_returning};
use crate::schema_set::{Diff, Sql};
use postgres_parser::nodes::InsertStmt;
use postgres_parser::Node;

impl Diff for InsertStmt {
    fn alter(&self, _other: &Node) -> Option<String> {
        unimplemented!()
    }

    fn drop(&self) -> String {
        unimplemented!()
    }

    fn name(&self, sql: &str) -> String {
        sql.into()
    }
}

impl Sql for InsertStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.withClause.sql());

        sql.push_str("INSERT INTO ");
        sql.push_str(&self.relation.sql());

        sql.push_str(&res_target_insert(&self.cols));

        sql.push(' ');

        if self.selectStmt.is_some() {
            sql.push_str(&self.selectStmt.sql());
        } else {
            sql.push_str("DEFAULT VALUES");
        }

        sql.push_str(&self.onConflictClause.sql());
        sql.push_str(&res_target_returning(&self.returningList));

        sql
    }
}
