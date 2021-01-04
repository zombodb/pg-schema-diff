use crate::schema_set::{Diff, Sql};
use postgres_parser::nodes::CreateCastStmt;
use postgres_parser::Node;

impl Sql for CreateCastStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE CAST");
        sql.push('(');
        sql.push_str(&self.sourcetype.sql());
        sql.push_str(" AS ");
        sql.push_str(&self.targettype.sql());
        sql.push(')');

        if self.func.is_some() {
            sql.push_str(&self.func.sql_prefix(" WITH FUNCTION "));
        } else {
            if self.inout {
                sql.push_str(" WITH INOUT ");
            } else {
                sql.push_str(" WITHOUT FUNCTION ");
            }
        }

        sql.push_str(&self.context.sql());

        sql
    }
}

impl Diff for CreateCastStmt {
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
