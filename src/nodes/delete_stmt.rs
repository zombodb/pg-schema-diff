use crate::nodes::res_target::res_target_returning;
use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::DeleteStmt;
use postgres_parser::Node;

impl Sql for DeleteStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.withClause.sql());
        sql.push_str("DELETE FROM ");
        sql.push_str(&self.relation.sql());
        sql.push_str(&self.usingClause.sql_prefix(" USING ", ", "));
        sql.push_str(&self.whereClause.sql_prefix(" WHERE "));
        sql.push_str(&res_target_returning(&self.returningList));

        sql
    }
}

impl Diff for DeleteStmt {}
