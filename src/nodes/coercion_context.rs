use crate::schema_set::Sql;
use postgres_parser::sys::CoercionContext;

impl Sql for CoercionContext {
    fn sql(&self) -> String {
        match self {
            CoercionContext::COERCION_IMPLICIT => " AS IMPLICIT",
            CoercionContext::COERCION_ASSIGNMENT => " AS ASSIGNMENT",
            CoercionContext::COERCION_EXPLICIT => "",
        }
        .into()
    }
}
