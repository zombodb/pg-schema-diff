use crate::schema_set::{Len, Sql, SqlList};
use crate::{make_name, make_operator_name};
use postgres_parser::nodes::A_Expr;
use postgres_parser::sys::A_Expr_Kind;
use postgres_parser::Node;

impl Sql for A_Expr {
    fn sql(&self) -> String {
        let mut sql = String::new();

        match self.kind {
            A_Expr_Kind::AEXPR_OP => {
                sql.push_str(&self.lexpr.sql());
                sql.push(' ');
                sql.push_str(
                    &make_operator_name(&self.name).expect("failed to make AEXPR_OP for A_Expr"),
                );
                sql.push(' ');
                sql.push_str(&self.rexpr.sql());
            }
            A_Expr_Kind::AEXPR_OP_ANY => {
                sql.push_str(&self.lexpr.sql());
                sql.push(' ');
                sql.push_str(
                    &make_operator_name(&self.name).expect("failed to make AEXPR_ALL for A_Expr"),
                );
                sql.push_str(&self.rexpr.sql_wrap(" ANY (", ")"));
            }
            A_Expr_Kind::AEXPR_OP_ALL => {
                sql.push_str(&self.lexpr.sql());
                sql.push(' ');
                sql.push_str(
                    &make_operator_name(&self.name).expect("failed to make AEXPR_ALL for A_Expr"),
                );
                sql.push_str(&self.rexpr.sql_wrap(" ALL (", ")"));
            }
            A_Expr_Kind::AEXPR_DISTINCT => {
                sql.push_str(&self.lexpr.sql());
                sql.push_str(" IS DISTINCT FROM ");
                sql.push_str(&self.rexpr.sql());
            }
            A_Expr_Kind::AEXPR_NOT_DISTINCT => {
                sql.push_str(&self.lexpr.sql());
                sql.push_str(" IS NOT DISTINCT FROM ");
                sql.push_str(&self.rexpr.sql());
            }
            A_Expr_Kind::AEXPR_NULLIF => {
                sql.push_str(" NULLIF (");
                sql.push_str(&self.lexpr.sql());
                sql.push_str(", ");
                sql.push_str(&self.rexpr.sql());
                sql.push(')');
            }
            A_Expr_Kind::AEXPR_OF => panic!("what is AEXPR_OF?"),
            A_Expr_Kind::AEXPR_IN => {
                sql.push_str(&self.lexpr.sql());
                sql.push_str(&self.rexpr.sql_wrap(" IN (", ")"));
            }
            A_Expr_Kind::AEXPR_LIKE => {
                sql.push_str(&self.lexpr.sql());
                sql.push_str(" LIKE ");
                sql.push_str(&self.rexpr.sql());
            }
            A_Expr_Kind::AEXPR_ILIKE => {
                sql.push_str(&self.lexpr.sql());
                sql.push_str(" ILIKE ");
                sql.push_str(&self.rexpr.sql());
            }
            A_Expr_Kind::AEXPR_SIMILAR => {
                sql.push_str(&self.lexpr.sql());
                sql.push_str(" SIMILAR TO ");
                sql.push_str(&self.rexpr.sql());
            }
            A_Expr_Kind::AEXPR_BETWEEN => {
                sql.push_str(&self.lexpr.sql());
                sql.push_str(" BETWEEN ");

                if let Node::List(nodes) = self.rexpr.as_ref().unwrap().as_ref() {
                    sql.push_str(&nodes[0].sql());
                    sql.push_str(" AND ");
                    sql.push_str(&nodes[1].sql());
                }
            }
            A_Expr_Kind::AEXPR_NOT_BETWEEN => {
                sql.push_str(&self.lexpr.sql());
                sql.push_str(" NOT BETWEEN ");

                if let Node::List(nodes) = self.rexpr.as_ref().unwrap().as_ref() {
                    sql.push_str(&nodes[0].sql());
                    sql.push_str(" AND ");
                    sql.push_str(&nodes[1].sql());
                }
            }
            A_Expr_Kind::AEXPR_BETWEEN_SYM => {
                sql.push_str(" BETWEEN SYMMETRIC ");
                sql.push_str(&self.lexpr.sql());
                sql.push_str(" AND ");
                sql.push_str(&self.rexpr.sql());
            }
            A_Expr_Kind::AEXPR_NOT_BETWEEN_SYM => {
                sql.push_str(" NOT BETWEEN SYMMETRIC ");
                sql.push_str(&self.lexpr.sql());
                sql.push_str(" AND ");
                sql.push_str(&self.rexpr.sql());
            }
            A_Expr_Kind::AEXPR_PAREN => {
                sql.push_str(&self.lexpr.sql_wrap("(", ")"));
            }
        }

        sql
    }
}
