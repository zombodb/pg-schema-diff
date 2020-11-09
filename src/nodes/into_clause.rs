use crate::schema_set::Sql;
use postgres_parser::nodes::IntoClause;

impl Sql for IntoClause {
    fn sql(&self) -> String {
        unimplemented!("IntoClause")
    }
}
