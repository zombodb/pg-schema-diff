use crate::schema_set::{Sql, SqlIdent};
use postgres_parser::Node;

mod a_arrayexpr;
mod a_const;
mod a_expr;
mod a_indicies;
mod a_indirection;
mod a_star;
mod alias;
mod bool_expr;
mod column_def;
mod column_ref;
mod common_table_expr;
mod constraint;
mod create_function_stmt;
mod create_stmt;
mod current_of_expr;
mod def_elem;
mod delete_stmt;
mod func_call;
mod function_parameter;
mod index_elem;
mod infer_clause;
mod insert_stmt;
mod into_clause;
mod join_expr;
mod locking_clause;
mod multi_assign_ref;
mod on_conflict_clause;
mod range_subselect;
mod range_var;
mod res_target;
mod row_expr;
mod select_stmt;
mod set_to_default;
mod sort_by;
mod sql_value_function;
mod sub_link;
mod type_cast;
mod type_name;
mod update_stmt;
mod value;
mod variable_set_stmt;
mod vec_of_node;
mod with_clause;

impl Sql for Node {
    fn sql(&self) -> String {
        match self {
            Node::A_ArrayExpr(stmt) => stmt.sql(),
            Node::A_Const(stmt) => stmt.sql(),
            Node::A_Expr(stmt) => stmt.sql(),
            Node::A_Indices(stmt) => stmt.sql(),
            Node::A_Indirection(stmt) => stmt.sql(),
            Node::A_Star(stmt) => stmt.sql(),
            Node::Alias(stmt) => stmt.sql(),
            Node::BoolExpr(stmt) => stmt.sql(),
            Node::ColumnDef(stmt) => stmt.sql(),
            Node::ColumnRef(stmt) => stmt.sql(),
            Node::CommonTableExpr(stmt) => stmt.sql(),
            Node::Constraint(stmt) => stmt.sql(),
            Node::CreateFunctionStmt(stmt) => stmt.sql(),
            Node::CreateStmt(stmt) => stmt.sql(),
            Node::CurrentOfExpr(stmt) => stmt.sql(),
            Node::DefElem(stmt) => stmt.sql(),
            Node::DeleteStmt(stmt) => stmt.sql(),
            Node::FuncCall(stmt) => stmt.sql(),
            Node::FunctionParameter(stmt) => stmt.sql(),
            Node::IndexElem(stmt) => stmt.sql(),
            Node::InferClause(stmt) => stmt.sql(),
            Node::InsertStmt(stmt) => stmt.sql(),
            Node::IntoClause(stmt) => stmt.sql(),
            Node::JoinExpr(stmt) => stmt.sql(),
            Node::List(stmt) => stmt.sql(),
            Node::LockingClause(stmt) => stmt.sql(),
            Node::MultiAssignRef(stmt) => stmt.sql(),
            Node::OnConflictClause(stmt) => stmt.sql(),
            Node::RangeSubselect(stmt) => stmt.sql(),
            Node::RangeVar(stmt) => stmt.sql(),
            Node::ResTarget(_stmt) => unreachable!("encountered a ResTarget node"),
            Node::RowExpr(stmt) => stmt.sql(),
            Node::SelectStmt(stmt) => stmt.sql(),
            Node::SetToDefault(stmt) => stmt.sql(),
            Node::SortBy(stmt) => stmt.sql(),
            Node::SQLValueFunction(stmt) => stmt.sql(),
            Node::SubLink(stmt) => stmt.sql(),
            Node::TypeCast(stmt) => stmt.sql(),
            Node::TypeName(stmt) => stmt.sql(),
            Node::UpdateStmt(stmt) => stmt.sql(),
            Node::Value(stmt) => stmt.sql(),
            Node::VariableSetStmt(stmt) => stmt.sql(),
            Node::WithClause(stmt) => stmt.sql(),

            _ => unimplemented!("Node: {:?}", self),
        }
    }
}
