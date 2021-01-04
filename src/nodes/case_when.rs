use crate::schema_set::Sql;
use postgres_parser::nodes::CaseWhen;

impl Sql for CaseWhen {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.expr.sql_prefix("WHEN "));
        sql.push_str(&self.result.sql_prefix(" THEN "));

        sql
    }
}
