use crate::schema_set::{Diff, Sql, SqlMaybeList};
use postgres_parser::nodes::AlterOwnerStmt;

impl Sql for AlterOwnerStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("ALTER ");
        sql.push_str(&self.objectType.sql());
        sql.push(' ');
        sql.push_str(&self.object.sql_maybe_list("."));
        sql.push_str(" OWNER TO ");
        sql.push_str(&self.newowner.sql());

        sql
    }
}

impl Diff for AlterOwnerStmt {}
