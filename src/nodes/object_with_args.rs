use crate::schema_set::{Sql, SqlIdent, SqlList};
use postgres_parser::nodes::ObjectWithArgs;

impl Sql for ObjectWithArgs {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.objname.sql_ident());
        if !self.args_unspecified {
            sql.push('(');
            sql.push_str(&self.objargs.sql(", "));
            sql.push(')');
        }

        sql
    }
}
