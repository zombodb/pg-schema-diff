use crate::make_name;
use crate::schema_set::SqlIdent;
use postgres_parser::nodes::ColumnRef;

impl SqlIdent for ColumnRef {
    fn sql(&self) -> String {
        make_name(&self.fields).expect("unable to make 'fields' for ColumnRef")
    }

    fn sql_prefix(&self, pre: &str) -> String {
        format!("{}{}", pre, self.sql())
    }

    fn sql_suffix(&self, suf: &str) -> String {
        format!("{}{}", self.sql(), suf)
    }
}
