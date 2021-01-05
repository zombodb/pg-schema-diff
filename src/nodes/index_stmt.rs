use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::IndexStmt;

impl Sql for IndexStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE ");
        if self.unique {
            sql.push_str("UNIQUE ");
        }
        sql.push_str("INDEX ");
        if self.concurrent {
            sql.push_str("CONCURRENTLY ");
        }
        if self.if_not_exists {
            sql.push_str("IF NOT EXISTS ");
        }
        sql.push_str(&self.idxname.sql_ident());
        sql.push_str(" ON ");
        if !self.relation.as_ref().unwrap().inh {
            sql.push_str("ONLY ");
        }
        sql.push_str(&self.relation.sql());
        sql.push_str(" USING ");
        sql.push_str(&self.accessMethod.sql_ident());
        sql.push_str(&self.indexParams.sql_wrap(", ", "(", ")"));
        sql.push_str(
            &self
                .indexIncludingParams
                .sql_prefix_and_wrap(" INCLUDE", "(", ")", ", "),
        );
        sql.push_str(&self.options.sql_prefix_and_wrap(" WITH ", "(", ")", ", "));
        sql.push_str(&self.tableSpace.sql_ident_prefix(" TABLESPACE "));
        sql.push_str(&self.whereClause.sql_prefix(" WHERE "));

        sql
    }
}

impl Diff for IndexStmt {}
