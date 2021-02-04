use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::RowExpr;
use postgres_parser::sys::CoercionForm;

impl Sql for RowExpr {
    fn sql(&self) -> String {
        let mut sql = String::new();

        match self.row_format {
            CoercionForm::COERCE_EXPLICIT_CALL => sql.push_str("ROW "),
            CoercionForm::COERCE_EXPLICIT_CAST => sql.push_str("ROW "),
            CoercionForm::COERCE_IMPLICIT_CAST => {}
        }

        sql.push_str(&self.args.sql_wrap(", ", "(", ")"));

        sql
    }
}
