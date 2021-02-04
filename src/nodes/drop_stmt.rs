use crate::schema_set::{Diff, Sql, SqlIdent};
use postgres_parser::nodes::DropStmt;
use postgres_parser::Node;

impl Sql for DropStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("DROP ");
        sql.push_str(&self.removeType.sql());
        sql.push(' ');
        if self.concurrent {
            sql.push_str("CONCURRENTLY ");
        }
        if self.missing_ok {
            sql.push_str("IF EXISTS ");
        }

        for (i, node) in self.objects.as_ref().unwrap().iter().enumerate() {
            if i > 0 {
                sql.push_str(", ");
            }
            if let Node::List(names) = node {
                sql.push_str(&names.sql_ident());
            } else if let Node::Value(_) = node {
                sql.push_str(&node.sql_ident());
            } else {
                sql.push_str(&node.sql());
            }
        }

        sql.push_str(&self.behavior.sql_prefix(" "));

        sql
    }
}

impl Diff for DropStmt {}
