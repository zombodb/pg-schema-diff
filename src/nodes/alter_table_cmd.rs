use crate::schema_set::{Sql, SqlMaybeList};
use postgres_parser::nodes::AlterTableCmd;
use postgres_parser::sys::AlterTableType;

impl Sql for AlterTableCmd {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push(' ');
        match self.subtype {
            AlterTableType::AT_SetRelOptions => {
                sql.push_str("SET (");
                sql.push_str(&self.def.sql_maybe_list(", "));
                sql.push(')');
            }

            _ => unimplemented!("AlterTableCmd::behavior = {:?}", self.behavior),
        }

        sql
    }
}
