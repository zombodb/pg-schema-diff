use crate::schema_set::{Diff, Sql, SqlList};

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

        let is_temp = self.relation.as_ref().unwrap().relpersistence == 't';

        sql.push_str("CREATE ");
        if is_temp {
            sql.push_str("TEMPORARY ");
        }
        sql.push_str("TABLE ");

        if self.if_not_exists {
            sql.push_str("IF NOT EXISTS ");
        }

        sql.push_str(&self.relation.sql());
        sql.push('(');
        sql.push_str(&self.tableElts.sql(", "));
        sql.push(')');
        sql.push_str(
            &self
                .inhRelations
                .sql_prefix_and_wrap(" INHERITS ", "(", ")", ", "),
        );
        sql.push_str(&self.oncommit.sql_prefix(" "));

        sql
    }
}
