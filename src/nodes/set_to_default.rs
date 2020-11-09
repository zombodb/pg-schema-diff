use crate::schema_set::Sql;
use postgres_parser::nodes::SetToDefault;

impl Sql for SetToDefault {
    fn sql(&self) -> String {
        "DEFAULT".into()
    }
}
