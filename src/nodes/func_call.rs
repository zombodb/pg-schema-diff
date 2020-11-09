use crate::make_name;
use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::FuncCall;

impl Sql for FuncCall {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&make_name(&self.funcname).expect("no name for FuncCall"));
        sql.push_str(&self.args.sql_wrap(Some("("), Some(")")));
        sql
    }
}
