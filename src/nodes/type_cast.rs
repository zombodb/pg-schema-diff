use crate::schema_set::Sql;
use postgres_parser::nodes::TypeCast;
use postgres_parser::Node;

impl Sql for TypeCast {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(
            &self
                .arg
                .as_ref()
                .expect("no 'arg' for TypeCast")
                .as_ref()
                .sql(),
        );
        sql.push_str("::");
        sql.push_str(&self.typeName.as_ref().unwrap().sql());

        sql
    }
}
