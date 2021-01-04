use crate::schema_set::Sql;
use postgres_parser::nodes::NullTest;
use postgres_parser::sys::NullTestType;

impl Sql for NullTestType {
    fn sql(&self) -> String {
        match self {
            NullTestType::IS_NULL => "IS NULL",
            NullTestType::IS_NOT_NULL => "IS NOT NULL",
        }
        .into()
    }
}

impl Sql for NullTest {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push('(');
        sql.push_str(&self.arg.sql());
        sql.push(')');
        sql.push(' ');
        sql.push_str(&self.nulltesttype.sql());

        sql
    }
}
