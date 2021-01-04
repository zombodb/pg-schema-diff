use crate::schema_set::{Diff, Len, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::DefineStmt;
use postgres_parser::Node;

impl Sql for DefineStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE ");
        if self.replace {
            sql.push_str("OR REPLACE ");
        }
        sql.push_str(&self.kind.sql());
        sql.push(' ');
        sql.push_str(&self.defnames.sql_ident());

        if self.args.len() > 0 {
            sql.push_str(" AS (");
            sql.push_str(&self.args.sql(", "));
            sql.push(')');
        }

        if self.definition.len() > 0 {
            sql.push(')');
            sql.push_str(&self.definition.sql(", "));
            sql.push('(');
        }

        sql
    }
}

impl Diff for DefineStmt {
    fn alter(&self, _other: &Node) -> Option<String> {
        unimplemented!()
    }

    fn drop(&self) -> String {
        unimplemented!()
    }

    fn name(&self, _sql: &str) -> String {
        self.defnames.sql_ident()
    }
}
