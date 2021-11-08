use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::CreateEnumStmt;

impl Sql for CreateEnumStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE TYPE ");
        sql.push_str(&self.typeName.sql_ident());
        sql.push_str(" AS ENUM ");
        sql.push('(');
        sql.push_str(&self.vals.sql_wrap_each_and_separate(", ", "'", "'"));
        sql.push(')');

        sql
    }
}

impl Diff for CreateEnumStmt {
    fn object_name(&self) -> Option<String> {
        Some(self.typeName.sql_ident())
    }

    fn object_type(&self) -> String {
        "ENUM".into()
    }
}
