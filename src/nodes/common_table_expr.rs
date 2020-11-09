use crate::schema_set::{Sql, SqlIdent};
use postgres_parser::nodes::CommonTableExpr;

impl Sql for CommonTableExpr {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.ctename.sql());
        sql.push_str(" AS (");
        sql.push_str(&self.ctequery.sql());
        sql.push_str(") ");

        sql
    }
}
