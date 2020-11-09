use crate::schema_set::Sql;
use postgres_parser::nodes::Value;

impl Sql for Value {
    fn sql(&self) -> String {
        if self.string.is_some() {
            return self.string.as_ref().unwrap().to_string();
        } else if self.int.is_some() {
            return self.int.as_ref().unwrap().to_string();
        } else if self.float.is_some() {
            return self.float.as_ref().unwrap().to_string();
        } else if self.bit_string.is_some() {
            return self.bit_string.as_ref().unwrap().to_string();
        } else if self.null.is_some() {
            return "NULL".into();
        } else {
            panic!("unexpected Value");
        }
    }
}
