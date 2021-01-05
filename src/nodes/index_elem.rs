use crate::schema_set::{Sql, SqlIdent};
use postgres_parser::nodes::IndexElem;

impl Sql for IndexElem {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.name.sql_ident());
        sql.push(' ');
        sql.push_str(&self.ordering.sql());

        sql
    }
}
