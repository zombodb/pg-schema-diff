use crate::schema_set::{Diff, Sql, SqlCollect};
use crate::{make_name, EMPTY_NODE_VEC};
use postgres_parser::nodes::CreateFunctionStmt;
use postgres_parser::sys::FunctionParameterMode;
use postgres_parser::sys::FunctionParameterMode::FUNC_PARAM_TABLE;
use postgres_parser::Node;

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
        drop.push_str(";");
        drop
    }

    fn name(&self, _: &str) -> String {
        make_name(&self.funcname).expect("unable to make name for CreateFunctionStatement")
    }
}

impl Sql for CreateFunctionStmt {
    fn sql(&self) -> String {
        let mut returns_table = false;
        let mut sql = String::new();

        sql.push_str("CREATE FUNCTION ");
        sql.push_str(&make_name(&self.funcname).expect("no 'funcname' for CreateFunctionStmt"));
        sql.push('(');
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
                .sql(),
        );
        sql.push(')');

        if returns_table {
            sql.push_str(" RETURNS TABLE");

            sql.push('(');
            sql.push_str(
                &self
                    .parameters
                    .as_ref()
                    .unwrap_or(&EMPTY_NODE_VEC)
                    .iter()
                    .filter(|p|
                        matches!(p, Node::FunctionParameter(param) if param.mode == FUNC_PARAM_TABLE)
                    )
                    .sql(),
            );
            sql.push(')');
        } else {
            sql.push_str(&format!(
                " RETURNS {}",
                self.returnType.as_ref().expect("no return type").sql()
            ));
        }

        for opt in self.options.as_ref().unwrap_or(&EMPTY_NODE_VEC) {
            sql.push(' ');
            sql.push_str(&opt.sql());
        }

        sql.push_str(";");

        sql
    }
}
