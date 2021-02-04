use crate::schema_set::{Diff, Sql, SqlIdent, SqlMaybeList};
use postgres_parser::nodes::AlterObjectSchemaStmt;

impl Sql for AlterObjectSchemaStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("ALTER ");
        sql.push_str(&self.objectType.sql());
        sql.push(' ');
        sql.push_str(&self.object.sql_maybe_list("."));
        sql.push_str(" SET SCHEMA ");
        sql.push_str(&self.newschema.sql_ident());

        sql
    }
}

impl Diff for AlterObjectSchemaStmt {}
