use crate::schema_set::{Sql, SqlIdent, SqlList};
use postgres_parser::nodes::VariableSetStmt;

impl Sql for VariableSetStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.name.sql_ident());
        sql.push_str(" TO ");
        sql.push_str(&self.args.sql(", "));

        sql
    }
}
