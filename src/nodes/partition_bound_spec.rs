use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::PartitionBoundSpec;

impl Sql for PartitionBoundSpec {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("FOR VALUES ");
        match self.strategy {
            'h' => {
                unimplemented!("TODO:  PartitionBoundSpec::strategy `h`")
            }

            'l' => {
                sql.push_str(&self.listdatums.sql_prefix_and_wrap("IN ", "(", ")", ", "));
            }

            'r' => {
                unimplemented!("TODO:  PartitionBoundSpec::strategy `r`")
            }

            _ => panic!(
                "unsupported PartitionBoundSpec::strategy: `{}`",
                self.strategy
            ),
        }

        sql
    }
}
