use crate::schema_set::{Sql, SqlIdent};
use crate::EMPTY_NODE_VEC;
use postgres_parser::nodes::ColumnDef;
use postgres_parser::Node;

impl Sql for ColumnDef {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.colname.sql());
        sql.push(' ');
        sql.push_str(&self.typeName.as_ref().unwrap().sql());

        for constraint in self.constraints.as_ref().unwrap_or(&EMPTY_NODE_VEC) {
            sql.push(' ');
            sql.push_str(&constraint.sql());
        }

        sql
    }
}
