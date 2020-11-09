use crate::schema_set::Sql;
use postgres_parser::nodes::A_Indices;

impl Sql for A_Indices {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if self.lidx.is_some() {
            sql.push_str(&self.lidx.sql());
            sql.push(':');
        }
        sql.push_str(&self.uidx.sql());

        sql
    }
}
