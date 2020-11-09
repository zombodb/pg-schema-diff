use crate::schema_set::Sql;
use postgres_parser::nodes::FunctionParameter;
use postgres_parser::sys::FunctionParameterMode;

impl Sql for FunctionParameter {
    fn sql(&self) -> String {
        let mut sql = String::new();

        match self.mode {
            FunctionParameterMode::FUNC_PARAM_IN => { /* IN is the default */ }
            FunctionParameterMode::FUNC_PARAM_INOUT => sql.push_str("INOUT "),
            FunctionParameterMode::FUNC_PARAM_OUT => sql.push_str("OUT "),
            FunctionParameterMode::FUNC_PARAM_VARIADIC => sql.push_str("VARIADIC "),
            FunctionParameterMode::FUNC_PARAM_TABLE => { /* do nothing */ }
        }

        if let Some(name) = self.name.as_ref() {
            sql.push_str(&format!("\"{}\" ", name));
        }

        sql.push_str(&self.argType.as_ref().unwrap().sql());
        if let Some(defexpr) = self.defexpr.as_ref() {
            sql.push_str(" DEFAULT ");
            sql.push_str(&defexpr.sql());
        }

        sql
    }
}
