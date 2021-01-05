use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::AlterTableStmt;

impl Sql for AlterTableStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("ALTER ");
        sql.push_str(&self.relkind.sql());
        sql.push(' ');
        if self.missing_ok {
            sql.push_str("IF EXISTS ");
        }
        if !self.relation.as_ref().unwrap().inh {
            sql.push_str("ONLY ");
        }
        sql.push_str(&self.relation.sql());
        sql.push_str(&self.cmds.sql(", "));

        sql
    }
}

impl Diff for AlterTableStmt {}
