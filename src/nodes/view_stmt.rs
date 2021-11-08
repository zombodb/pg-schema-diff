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
    fn alter_stmt(&self, other: &Node) -> Option<String> {
        Some(format!("{};\n{}", self.drop_stmt().unwrap(), other.sql()))
    }

    fn drop_stmt(&self) -> Option<String> {
        Some(format!("DROP VIEW {}", self.view.sql()))
    }

    fn object_name(&self) -> Option<String> {
        Some(self.view.sql())
    }

    fn object_type(&self) -> String {
        "VIEW".into()
    }
}
