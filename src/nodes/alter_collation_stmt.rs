use crate::schema_set::{Diff, Sql, SqlIdent};
use postgres_parser::nodes::AlterCollationStmt;

impl Sql for AlterCollationStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("ALTER COLLATION ");
        sql.push_str(&self.collname.sql_ident());
        sql.push_str(" REFRESH VERSION");

        sql
    }
}

impl Diff for AlterCollationStmt {}
