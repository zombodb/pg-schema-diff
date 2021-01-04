use crate::schema_set::Sql;
use postgres_parser::sys::DropBehavior;

impl Sql for DropBehavior {
    fn sql(&self) -> String {
        match self {
            DropBehavior::DROP_RESTRICT => "RESTRICT",
            DropBehavior::DROP_CASCADE => "CASCADE",
        }
        .into()
    }
}
