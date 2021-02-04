use crate::schema_set::{Sql, SqlIdent, SqlList};
use postgres_parser::nodes::IntoClause;

impl Sql for IntoClause {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.rel.sql());
        sql.push_str(&self.colNames.sql_wrap(", ", "(", ") "));
        sql.push_str(&self.accessMethod.sql_ident_prefix("USING "));
        sql.push_str(&self.options.sql_prefix_and_wrap(" WITH", "(", ") ", ", "));
        sql.push_str(&self.onCommit.sql());
        sql.push_str(&self.tableSpaceName.sql_ident_prefix("TABLESPACE "));
        sql.push_str(&self.viewQuery.sql_prefix(" AS "));

        sql
    }
}
