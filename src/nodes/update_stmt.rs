use crate::nodes::res_target::{res_target_returning, res_target_update};
use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::UpdateStmt;

impl Sql for UpdateStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.withClause.sql());

        sql.push_str("UPDATE ");
        sql.push_str(&self.relation.sql());
        sql.push_str(" SET ");
        sql.push_str(&res_target_update(&self.targetList));

        if self.fromClause.is_some() {
            sql.push_str(" FROM ");
            sql.push_str(&self.fromClause.sql(", "));
        }

        if self.whereClause.is_some() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.whereClause.sql());
        }

        sql.push_str(&res_target_returning(&self.returningList));

        sql
    }
}

impl Diff for UpdateStmt {}
