use crate::make_name;
use crate::schema_set::Sql;
use postgres_parser::nodes::TypeName;

impl Sql for TypeName {
    fn sql(&self) -> String {
        let mut sql = String::new();

        if self.setof {
            sql.push_str("SETOF ");
        }

        sql.push_str(&make_name(&self.names).expect("missing TypeName::names"));
        if let Some(array_bounds) = self.arrayBounds.as_ref() {
            for _ in 0..array_bounds.len() {
                sql.push_str("[]");
            }
        }

        sql
    }
}
