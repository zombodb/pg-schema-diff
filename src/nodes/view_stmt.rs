use crate::schema_set::{Diff, Len, Sql, SqlList};
use postgres_parser::nodes::ViewStmt;
use postgres_parser::Node;

impl Sql for ViewStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE ");
        if self.replace {
            sql.push_str("OR REPLACE ");
        }
        sql.push_str("VIEW ");
        sql.push_str(&self.view.sql());
        if self.options.len() > 0 {
            sql.push_str(" WITH (");
            sql.push_str(&self.options.sql(", "));
            sql.push(')');
        }
        sql.push_str(" AS ");
        sql.push_str(&self.query.sql());
        sql.push(' ');
        sql.push_str(&self.withCheckOption.sql());

        sql
    }
}

impl Diff for ViewStmt {
    fn alter(&self, _other: &Node) -> Option<String> {
        None
    }

    fn drop(&self) -> String {
        let mut sql = String::new();

        sql.push_str("DROP VIEW ");
        sql.push_str(&self.view.sql());

        sql
    }

    fn name(&self, _sql: &str) -> String {
        self.view.sql()
    }
}
