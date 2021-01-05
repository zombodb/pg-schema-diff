use crate::schema_set::{Diff, Len, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::PrepareStmt;

impl Sql for PrepareStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("PREPARE ");

        sql.push_str(&self.name.sql_ident());
        if self.argtypes.len() > 0 {
            sql.push_str(&self.argtypes.sql(", "))
        }

        sql.push_str(" AS ");
        sql.push_str(&self.query.sql());

        sql
    }
}

impl Diff for PrepareStmt {}
