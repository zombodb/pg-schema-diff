use crate::schema_set::Sql;
use postgres_parser::sys::SortByNulls;

impl Sql for SortByNulls {
    fn sql(&self) -> String {
        match self {
            SortByNulls::SORTBY_NULLS_DEFAULT => String::new(),
            SortByNulls::SORTBY_NULLS_FIRST => "NULLS FIRST".into(),
            SortByNulls::SORTBY_NULLS_LAST => "NULLS LAST".into(),
        }
    }
}
