use crate::schema_set::{Sql, SqlIdent, SqlList};
use postgres_parser::nodes::TypeName;

impl Sql for TypeName {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if self.setof {
            sql.push_str("SETOF ");
        }

        sql.push_str(&self.names.sql_ident());
        if let Some(array_bounds) = self.arrayBounds.as_ref() {
            for _ in 0..array_bounds.len() {
                sql.push_str("[]");
            }
        }
        sql.push_str(&self.typmods.sql_wrap(",", "(", ")"));

        sql
    }
}
