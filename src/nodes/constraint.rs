use crate::schema_set::Sql;
use postgres_parser::nodes::Constraint;
use postgres_parser::sys::ConstrType;
use postgres_parser::Node;

impl Sql for Constraint {
    fn sql(&self) -> String {
        let mut sql = String::new();

        match self.contype {
            ConstrType::CONSTR_NOTNULL => sql.push_str("NOT NULL"),
            ConstrType::CONSTR_NULL => sql.push_str("NULL"),
            ConstrType::CONSTR_PRIMARY => sql.push_str("PRIMARY KEY"),
            ConstrType::CONSTR_DEFAULT => {
                sql.push_str("DEFAULT ");
                sql.push_str(
                    &self
                        .raw_expr
                        .as_ref()
                        .expect("no raw_expr for Constraint")
                        .sql(),
                );
            }
            _ => unimplemented!("Constraint: {:?}", self),
        }

        sql
    }
}
