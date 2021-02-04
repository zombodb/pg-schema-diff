use crate::schema_set::Sql;
use postgres_parser::nodes::TableLikeClause;

impl Sql for TableLikeClause {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.relation.sql_prefix("LIKE "));

        sql
    }
}
