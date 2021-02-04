use crate::schema_set::{Diff, Sql, SqlMaybeList};
use postgres_parser::nodes::CommentStmt;

impl Sql for CommentStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("COMMENT ON ");
        sql.push_str(&self.objtype.sql());
        sql.push(' ');
        sql.push_str(&self.object.sql_maybe_list("."));
        sql.push_str(" IS ");
        sql.push_str("'");
        sql.push_str(&self.comment.as_ref().unwrap().replace("'", "''"));
        sql.push_str("'");

        sql
    }
}

impl Diff for CommentStmt {}
