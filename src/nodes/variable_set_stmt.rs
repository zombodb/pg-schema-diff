use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::VariableSetStmt;
use postgres_parser::sys::VariableSetKind;
use postgres_parser::Node;

impl Sql for VariableSetStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        match self.kind {
            VariableSetKind::VAR_SET_VALUE => {
                sql.push_str("SET ");
                if self.is_local {
                    sql.push_str("LOCAL ");
                }
                sql.push_str(&self.name.sql_ident());
                sql.push_str(" TO ");
                sql.push_str(&self.args.sql(", "));
            }
            VariableSetKind::VAR_SET_DEFAULT => {
                sql.push_str("SET ");
                if self.is_local {
                    sql.push_str("LOCAL ");
                }
                sql.push_str(&self.name.sql_ident());
                sql.push_str(" TO DEFAULT");
            }
            VariableSetKind::VAR_SET_CURRENT => {
                sql.push_str("SET ");
                if self.is_local {
                    sql.push_str("LOCAL ");
                }
                sql.push_str(&self.name.sql_ident());
                sql.push_str(" FROM CURRENT ");
            }
            VariableSetKind::VAR_SET_MULTI => unimplemented!("VariableSetKind::VAR_SET_MULTI"),
            VariableSetKind::VAR_RESET => {
                sql.push_str("RESET ");
                if self.is_local {
                    sql.push_str("LOCAL ");
                }
                sql.push_str(&self.name.sql_ident());
            }
            VariableSetKind::VAR_RESET_ALL => {
                sql.push_str("RESET ALL");
            }
        }

        sql
    }
}

impl Diff for VariableSetStmt {}
