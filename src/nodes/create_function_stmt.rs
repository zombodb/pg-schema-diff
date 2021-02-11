use crate::schema_set::{Diff, Sql, SqlCollect, SqlIdent, SqlList};
use crate::{make_name, EMPTY_NODE_VEC};
use postgres_parser::nodes::CreateFunctionStmt;

use postgres_parser::sys::FunctionParameterMode::FUNC_PARAM_TABLE;
use postgres_parser::Node;

impl Sql for CreateFunctionStmt {
    fn sql(&self) -> String {
        let mut returns_table = false;
        let mut sql = String::new();

        if self.replace {
            sql.push_str("CREATE OR REPLACE ");
        } else {
            sql.push_str("CREATE ");
        }

        if self.is_procedure {
            sql.push_str("PROCEDURE ");
        } else {
            sql.push_str("FUNCTION ");
        }

        sql.push_str(&self.funcname.sql_ident());
        sql.push_str(
            &self
                .parameters
                .as_ref()
                .unwrap_or(&EMPTY_NODE_VEC)
                .iter()
                .filter(|p| match p {
                    Node::FunctionParameter(param) if param.mode != FUNC_PARAM_TABLE => true,
                    Node::FunctionParameter(param) if param.mode == FUNC_PARAM_TABLE => {
                        returns_table = true;
                        false
                    }
                    _ => false,
                })
                .sql_wrap("(", ")"),
        );

        if returns_table {
            sql.push_str(" RETURNS TABLE");

            sql.push_str(
                &self
                    .parameters
                    .as_ref()
                    .unwrap_or(&EMPTY_NODE_VEC)
                    .iter()
                    .filter(|p|
                        matches!(p, Node::FunctionParameter(param) if param.mode == FUNC_PARAM_TABLE)
                    )
                    .sql_wrap("(", ")"),
            );
        } else {
            sql.push_str(&self.returnType.sql_prefix(" RETURNS "));
        }

        sql.push_str(&self.options.sql_prefix(" ", " "));

        sql
    }
}

impl Diff for CreateFunctionStmt {
    fn alter(&self, other: &Node) -> Option<String> {
        if let Node::CreateFunctionStmt(other) = other {
            let mut alter = String::new();
            alter.push_str(&other.drop());
            alter.push('\n');
            alter.push_str(&self.sql());
            Some(alter)
        } else {
            None
        }
    }

    fn drop(&self) -> String {
        let mut drop = String::new();
        drop.push_str("DROP FUNCTION ");
        drop.push_str(&make_name(&self.funcname).expect("no 'funcname' for CreateFunctionStmt"));
        drop.push('(');
        drop.push_str(
            &self
                .parameters
                .as_ref()
                .unwrap_or(&EMPTY_NODE_VEC)
                .iter()
                .filter(|p| {
                    matches!(p,
                    Node::FunctionParameter(param) if param.mode != FUNC_PARAM_TABLE)
                })
                .sql(),
        );
        drop.push(')');

        drop
    }

    fn object_name(&self) -> Option<String> {
        Some(make_name(&self.funcname).expect("unable to make name for CreateFunctionStatement"))
    }
}
