use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::CreateFdwStmt;

impl Sql for CreateFdwStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE FOREIGN DATA WRAPPER ");
        sql.push_str(&self.fdwname.sql_ident());
        sql.push(' ');
        sql.push_str(&self.func_options.sql(" "));
        sql.push_str(&self.options.sql_prefix_and_wrap(" OPTIONS", "(", ")", ", "));

        sql
    }
}

impl Diff for CreateFdwStmt {}
