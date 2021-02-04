use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::PartitionBoundSpec;

impl Sql for PartitionBoundSpec {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("FOR VALUES ");
        match self.strategy {
            'h' => {
                sql.push_str("WITH (");
                sql.push_str("MODULUS ");
                sql.push_str(&self.modulus.to_string());
                sql.push_str(", ");
                sql.push_str("REMAINDER ");
                sql.push_str(&self.remainder.to_string());
                sql.push(')');
            }

            'l' => {
                sql.push_str(&self.listdatums.sql_prefix_and_wrap("IN ", "(", ")", ", "));
            }

            'r' => {
                sql.push_str("FROM (");
                sql.push_str(&self.lowerdatums.sql(", "));
                sql.push(')');
                sql.push_str(" TO (");
                sql.push_str(&self.upperdatums.sql(", "));
                sql.push(')');
            }

            _ => panic!(
                "unsupported PartitionBoundSpec::strategy: `{}`",
                self.strategy
            ),
        }

        sql
    }
}
