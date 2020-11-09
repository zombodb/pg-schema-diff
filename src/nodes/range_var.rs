use crate::schema_set::{Sql, SqlIdent};
use postgres_parser::nodes::RangeVar;
use postgres_parser::quote_identifier;

impl Sql for RangeVar {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if !self.inh {
            sql.push_str("ONLY ");
        }

        sql.push_str(&self.catalogname.sql_suffix("."));
        sql.push_str(&self.schemaname.sql_suffix("."));
        sql.push_str(&self.relname.sql());
        sql.push_str(&self.alias.sql());

        sql
    }
}
