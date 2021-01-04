use crate::schema_set::{Sql, SqlIdent};
use postgres_parser::nodes::CurrentOfExpr;

impl Sql for CurrentOfExpr {
    fn sql(&self) -> String {
        format!("CURRENT OF {}", self.cursor_name.sql_ident())
    }
}
