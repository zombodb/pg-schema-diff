use crate::schema_set::{Diff, Sql};
use postgres_parser::nodes::CreateTableAsStmt;

impl Sql for CreateTableAsStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE ");
        if self.into.is_some() {
            if self
                .into
                .as_ref()
                .unwrap()
                .rel
                .as_ref()
                .unwrap()
                .relpersistence
                == 't'
            {
                sql.push_str("TEMPORARY ");
            }
        }

        sql.push_str(&self.relkind.sql());
        sql.push(' ');

        if self.if_not_exists {
            sql.push_str("IF NOT EXISTS ");
        }
        sql.push_str(&self.into.sql());
        sql.push_str(" AS ");
        sql.push_str(&self.query.sql());

        if self.into.is_some() {
            if self.into.as_ref().unwrap().skipData {
                sql.push_str(" WITH NO DATA");
            } else {
                sql.push_str(" WITH DATA");
            }
        }

        sql
    }
}

impl Diff for CreateTableAsStmt {}
