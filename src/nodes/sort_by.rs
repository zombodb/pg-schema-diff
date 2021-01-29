use crate::make_operator_name;
use crate::schema_set::Sql;
use postgres_parser::nodes::SortBy;
use postgres_parser::sys::{SortByDir, SortByNulls};

impl Sql for SortBy {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.node.sql());

        match self.sortby_dir {
            SortByDir::SORTBY_DEFAULT => {}
            SortByDir::SORTBY_ASC => sql.push_str(" ASC"),
            SortByDir::SORTBY_DESC => sql.push_str(" DESC"),
            SortByDir::SORTBY_USING => {
                sql.push_str(" USING ");
                sql.push_str(
                    &make_operator_name(&self.useOp)
                        .expect("failed to make 'useOp' name for SortBy"),
                );
            }
        }

        match self.sortby_nulls {
            SortByNulls::SORTBY_NULLS_DEFAULT => { /* noop */ }
            SortByNulls::SORTBY_NULLS_FIRST => sql.push_str(" NULLS FIRST"),
            SortByNulls::SORTBY_NULLS_LAST => sql.push_str(" NULLS LAST"),
        }

        sql
    }
}
