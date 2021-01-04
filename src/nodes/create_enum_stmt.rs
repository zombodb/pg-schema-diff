use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::CreateEnumStmt;
use postgres_parser::Node;

impl Sql for CreateEnumStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE TYPE ");
        sql.push_str(&self.typeName.sql_ident());
        sql.push_str(" AS ENUM ");
        sql.push('(');
        sql.push_str(&self.vals.sql(", "));
        sql.push(')');

        sql
    }
}

impl Diff for CreateEnumStmt {
    fn alter(&self, _other: &Node) -> Option<String> {
        None
    }

    fn drop(&self) -> String {
        let mut sql = String::new();

        sql.push_str("DROP TYPE ");
        sql.push_str(&self.typeName.sql_ident());

        sql
    }

    fn name(&self, _sql: &str) -> String {
        self.typeName.sql_ident()
    }
}
