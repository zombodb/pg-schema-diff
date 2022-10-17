use std::cmp::Ordering;
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
                .map(|node| node.clone())
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
                    .map(|node| node.clone())
                    .sql_wrap("(", ")"),
            );
        } else {
            sql.push_str(&self.returnType.sql_prefix(" RETURNS "));
        }

        let mut orig_options = self.options.clone();
        if let Some(mut options) = orig_options {
            options.sort_by(|a, b| match a {
                Node::DefElem(a_defelem) => {
                    if let Node::DefElem(b_defelem) = b {
                        return a_defelem.sql().cmp(&b_defelem.sql());
                    }

                    Ordering::Equal
                },

                _ => Ordering::Equal
            });
            orig_options = Some(options);
        }

        sql.push_str(&orig_options.sql_prefix(" ", " "));

        sql
    }
}

impl Diff for CreateFunctionStmt {
    fn alter_stmt(&self, other: &Node) -> Option<String> {
        let mut alter = String::new();
        alter.push_str(&self.drop_stmt().unwrap());
        alter.push_str(";\n");
        alter.push_str(&other.sql());
        Some(alter)
    }

    fn drop_stmt(&self) -> Option<String> {
        let mut drop = String::new();
        drop.push_str("DROP FUNCTION IF EXISTS ");
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
                .map(|node| match node {
                    Node::FunctionParameter(fp) => {
                        let mut fp = fp.clone();
                        fp.defexpr = None;
                        Node::FunctionParameter(fp)
                    }
                    _ => panic!("unexpected function parameter node type"),
                })
                .sql(),
        );
        drop.push(')');
        Some(drop)
    }

    fn object_name(&self) -> Option<String> {
        let mut as_ = String::new();
        let mut is_c = false;
        for opt in self.options.iter().flatten() {
            if let Node::DefElem(defelem) = opt {
                if defelem.defname.as_ref().unwrap().eq_ignore_ascii_case("as") {
                    as_ = defelem.sql();
                    break;
                } else if defelem.defname.as_ref().unwrap().eq_ignore_ascii_case("language") {
                    is_c = true;
                }
            }
        }

        let name = make_name(&self.funcname).expect("unable to make name for CreateFunctionStatement");
        if is_c {
            Some(name + &as_)
        } else {
            Some(name)
        }
    }

    fn object_type(&self) -> String {
        "FUNCTION".into()
    }
}
