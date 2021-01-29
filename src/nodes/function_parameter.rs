use crate::schema_set::{Sql, SqlIdent};
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

        sql.push_str(&self.name.sql_ident_suffix(" "));
        sql.push_str(&self.argType.sql());
        sql.push_str(&self.defexpr.sql_prefix(" DEFAULT "));

        sql
    }
}
