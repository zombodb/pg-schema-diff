use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::RangeFunction;

impl Sql for RangeFunction {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if self.lateral {
            sql.push_str("LATERAL ");
        }

        sql.push_str(&self.functions.sql(", "));
        if self.ordinality {
            sql.push_str("WITH ORDINALITY ");
        }
        sql.push_str(&self.alias.sql());

        sql
    }
}
