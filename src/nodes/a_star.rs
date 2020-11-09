use crate::schema_set::Sql;
use postgres_parser::nodes::A_Star;

impl Sql for A_Star {
    fn sql(&self) -> String {
        "*".into()
    }
}
