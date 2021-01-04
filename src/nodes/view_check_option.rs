use crate::schema_set::Sql;
use postgres_parser::sys::ViewCheckOption;

impl Sql for ViewCheckOption {
    fn sql(&self) -> String {
        match self {
            ViewCheckOption::NO_CHECK_OPTION => "",
            ViewCheckOption::LOCAL_CHECK_OPTION => "LOCAL",
            ViewCheckOption::CASCADED_CHECK_OPTION => "CASCADED",
        }
        .into()
    }
}
