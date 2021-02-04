use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::CreateRangeStmt;

impl Sql for CreateRangeStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE TYPE ");
        sql.push_str(&self.typeName.sql_ident());
        sql.push_str(" AS RANGE (");
        sql.push_str(&self.params.sql(", "));
        sql.push(')');

        sql
    }
}

impl Diff for CreateRangeStmt {}
