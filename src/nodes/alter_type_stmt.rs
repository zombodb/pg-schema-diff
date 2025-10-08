use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::AlterTypeStmt;

impl Sql for AlterTypeStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("ALTER TYPE ");
        sql.push_str(&self.typeName.sql("."));

        println!("/* what to do with these options?:  {:?} */;", self.options);
        sql
    }
}

impl Diff for AlterTypeStmt {}
