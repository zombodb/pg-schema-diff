use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::CoalesceExpr;

impl Sql for CoalesceExpr {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("COALESCE (");
        sql.push_str(&self.args.sql(", "));
        sql.push(')');

        sql
    }
}
