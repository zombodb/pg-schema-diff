use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::PartitionSpec;

impl Sql for PartitionSpec {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.strategy.as_ref().unwrap());
        sql.push_str(&self.partParams.sql_wrap(", ", "(", ")"));

        sql
    }
}
