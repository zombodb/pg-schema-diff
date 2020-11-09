use crate::schema_set::{Sql, SqlList};

use postgres_parser::nodes::A_Indirection;

impl Sql for A_Indirection {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.arg.sql());
        sql.push_str(&self.indirection.sql_wrap_each(Some("["), Some("]")));

        sql
    }
}
