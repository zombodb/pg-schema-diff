use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};

use postgres_parser::nodes::CreateStmt;
use postgres_parser::Node;

impl Sql for CreateStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        let is_temp = self.relation.as_ref().unwrap().relpersistence == 't';

        sql.push_str("CREATE ");
        if is_temp {
            sql.push_str("TEMPORARY ");
        }
        sql.push_str("TABLE ");

        if self.if_not_exists {
            sql.push_str("IF NOT EXISTS ");
        }

        sql.push_str(&self.relation.sql());

        if self.partbound.is_some() {
            sql.push_str(&self.inhRelations.sql_prefix(" PARTITION OF ", ", "));
            sql.push_str(&self.partbound.sql_prefix(" "));
        } else {
            sql.push('(');
            sql.push_str(&self.tableElts.sql(", "));
            sql.push(')');
        }

        if self.partbound.is_none() {
            sql.push_str(
                &self
                    .inhRelations
                    .sql_prefix_and_wrap(" INHERITS ", "(", ")", ", "),
            );
        }
        sql.push_str(&self.partspec.sql_prefix(" PARTITION BY "));
        sql.push_str(&self.accessMethod.sql_ident_prefix(" USING "));
        sql.push_str(&self.options.sql_prefix_and_wrap(" WITH ", "(", ")", ", "));
        sql.push_str(&self.oncommit.sql_prefix(" "));

        sql
    }
}

impl Diff for CreateStmt {
    fn alter(&self, _other: &Node) -> Option<String> {
        unimplemented!()
    }

    fn drop(&self) -> String {
        unimplemented!()
    }

    fn name(&self, _: &str) -> String {
        self.relation.as_ref().unwrap().sql()
    }
}
