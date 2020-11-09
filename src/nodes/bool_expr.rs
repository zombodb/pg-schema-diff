use crate::schema_set::{Len, Sql, SqlList};
use postgres_parser::nodes::BoolExpr;
use postgres_parser::sys::BoolExprType;

impl Sql for BoolExpr {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if self.args.len() > 1 {
            sql.push('(');
        }
        sql.push_str(&match self.boolop {
            BoolExprType::AND_EXPR => self.args.sql(" AND "),
            BoolExprType::OR_EXPR => self.args.sql(" OR "),
            BoolExprType::NOT_EXPR => self.args.sql_wrap_each(Some("NOT "), None),
        });
        if self.args.len() > 1 {
            sql.push(')');
        }

        sql
    }
}
