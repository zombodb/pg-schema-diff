use crate::schema_set::{Sql, SqlIdent, SqlList};
use postgres_parser::nodes::IndexElem;

impl Sql for IndexElem {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.expr.sql_wrap("(", ")"));
        sql.push_str(&self.name.sql_ident());
        sql.push(' ');
        sql.push_str(
            &self
                .collation
                .sql_prefix_and_wrap(" COLLATE ", "\"", "\"", ""),
        );
        sql.push_str(&self.opclass.sql_prefix(" ", ""));
        sql.push_str(&self.ordering.sql_prefix(" "));
        sql.push_str(&self.nulls_ordering.sql_prefix(" "));

        sql
    }
}
