use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::CaseExpr;

impl Sql for CaseExpr {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CASE");

        if self.arg.is_some() {
            sql.push(' ');
            sql.push_str(&self.arg.sql());
        }

        sql.push(' ');
        sql.push_str(&self.args.sql(" "));

        sql.push_str(" ELSE ");
        sql.push_str(&self.defresult.sql());
        sql.push_str(" END");

        sql
    }
}
