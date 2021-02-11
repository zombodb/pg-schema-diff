use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::CreateOpClassStmt;
use postgres_parser::Node;

impl Sql for CreateOpClassStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE OPERATOR CLASS ");
        sql.push_str(&self.opclassname.sql_ident());
        sql.push(' ');
        if self.isDefault {
            sql.push_str("DEFAULT ");
        }
        sql.push_str("FOR TYPE ");
        sql.push_str(&self.datatype.sql());
        sql.push_str(" USING ");
        sql.push_str(&self.amname.sql_ident());
        sql.push_str(&self.opfamilyname.sql_ident_prefix(" FAMILY "));
        sql.push_str(" AS ");
        sql.push_str(&self.items.sql(", "));

        sql
    }
}

impl Diff for CreateOpClassStmt {
    fn object_name(&self) -> Option<String> {
        Some(self.opclassname.sql_ident())
    }
}
