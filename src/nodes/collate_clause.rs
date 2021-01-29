use crate::make_name;
use crate::schema_set::Sql;
use postgres_parser::nodes::CollateClause;

impl Sql for CollateClause {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push('(');
        sql.push_str(&self.arg.sql());
        sql.push_str(" COLLATE ");
        sql.push_str(&make_name(&self.collname).expect("unable to make CollateClause::collname"));
        sql.push(')');

        sql
    }
}
