use crate::make_name;
use crate::schema_set::Sql;
use postgres_parser::nodes::CollateClause;

impl Sql for CollateClause {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&make_name(&self.collname).expect("unable to make CollateCluse::collname"));

        if self.arg.is_some() {
            unimplemented!("TODO:  how to handle CollateClause::arg")
        }

        sql
    }
}
