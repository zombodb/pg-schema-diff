use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::CompositeTypeStmt;

impl Sql for CompositeTypeStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE TYPE ");
        sql.push_str(&self.typevar.sql());
        sql.push_str(" AS ");
        sql.push_str(&self.coldeflist.sql_wrap(", ", "(", ")"));
        sql
    }
}

impl Diff for CompositeTypeStmt {}
