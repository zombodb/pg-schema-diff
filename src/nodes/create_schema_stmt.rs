use crate::schema_set::{Diff, Sql, SqlIdent};
use postgres_parser::nodes::CreateSchemaStmt;
use postgres_parser::Node;

impl Sql for CreateSchemaStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE SCHEMA ");
        if self.if_not_exists {
            sql.push_str("IF NOT EXISTS ");
        }

        if self.authrole.is_some() {
            sql.push_str("AUTHORIZATION ");
            sql.push_str(&self.authrole.sql());
        } else {
            sql.push_str(&self.schemaname.sql_ident())
        }

        sql
    }
}

impl Diff for CreateSchemaStmt {
    fn alter_stmt(&self, other: &Node) -> Option<String> {
        if let Node::CreateSchemaStmt(other) = other {
            if self.authrole != other.authrole {
                let mut sql = String::new();

                sql.push_str("ALTER SCHEMA ");
                sql.push_str(&self.schemaname.sql_ident());
                sql.push_str(" OWNER TO ");
                sql.push_str(&self.authrole.sql());

                return Some(sql);
            }
        }

        None
    }

    fn drop_stmt(&self) -> Option<String> {
        let mut sql = String::new();
        sql.push_str("DROP SCHEMA ");
        sql.push_str(&self.schemaname.clone().unwrap());
        Some(sql)
    }

    fn object_name(&self) -> Option<String> {
        Some(self.schemaname.sql_ident())
    }

    fn object_type(&self) -> String {
        "SCHEMA".into()
    }
}
