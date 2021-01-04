use crate::schema_set::{Len, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::InferClause;

impl Sql for InferClause {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if self.indexElems.len() > 0 {
            sql.push_str(&self.indexElems.sql_wrap(", ", "(", ")"));
        }

        if self.whereClause.is_some() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.whereClause.sql());
        }

        if self.conname.is_some() {
            sql.push_str(" ON CONSTRAINT ");
            sql.push_str(&self.conname.sql_ident());
        }

        sql
    }
}
