use crate::nodes::res_target::res_target_select;
use crate::schema_set::{Diff, Sql, SqlList};

use postgres_parser::nodes::SelectStmt;
use postgres_parser::Node;

impl Diff for SelectStmt {
    fn alter(&self, _other: &Node) -> Option<String> {
        unimplemented!()
    }

    fn drop(&self) -> String {
        unimplemented!()
    }

    fn name(&self, sql: &str) -> String {
        sql.into()
    }
}

impl Sql for SelectStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.withClause.sql());
        if let Some(values_list) = self.valuesLists.as_ref() {
            sql.push_str("VALUES (");
            sql.push_str(&values_list.sql(", "));
            sql.push(')');
        } else {
            sql.push_str("SELECT ");
            sql.push_str(&self.distinctClause.sql_prefix(" DISTINCT ", ", "));
            sql.push_str(&res_target_select(&self.targetList));
            sql.push_str(&self.intoClause.sql_prefix(" INTO "));
            sql.push_str(&self.fromClause.sql_prefix(" FROM ", ", "));
            sql.push_str(&self.whereClause.sql_prefix(" WHERE "));
            sql.push_str(&self.groupClause.sql_prefix(" GROUP BY ", ", "));
            sql.push_str(&self.havingClause.sql_prefix(" HAVING "));
            sql.push_str(&self.windowClause.sql_prefix(" WINDOW ", ", "));
        }

        sql.push_str(&self.sortClause.sql_prefix(" ORDER BY ", ", "));
        sql.push_str(&self.limitCount.sql_prefix(" LIMIT "));
        sql.push_str(&self.limitOffset.sql_prefix(" OFFSET "));
        sql.push_str(&self.lockingClause.sql(""));

        //  not sure what to do about these?
        if self.larg.is_some() {
            unimplemented!("'larg' of SelectStmt")
        }
        if self.rarg.is_some() {
            unimplemented!("'rarg' of SelectStmt")
        }

        sql
    }
}
