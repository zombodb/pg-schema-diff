use crate::schema_set::Sql;
use postgres_parser::nodes::RangeSubselect;

impl Sql for RangeSubselect {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if self.lateral {
            sql.push_str("LATERAL ");
        }
        sql.push_str(&self.subquery.sql_wrap("(", ")"));
        sql.push_str(&self.alias.sql());

        sql
    }
}
