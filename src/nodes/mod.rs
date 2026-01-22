// Copyright 2020-2026 Eric B. Ridge <eebbrr@gmail.com>. All rights reserved. Use
// of this source code is governed by the Postgres license that can be found in
// the LICENSE file.
use crate::schema_set::{Sql, SqlIdent};
use postgres_parser::Node;

mod a_arrayexpr;
mod a_const;
mod a_expr;
mod a_indicies;
mod a_indirection;
mod a_star;
mod access_priv;
mod alias;
mod alter_collation_stmt;
mod alter_function_stmt;
mod alter_object_schema_stmt;
mod alter_owner_stmt;
mod alter_table_cmd;
mod alter_table_stmt;
mod alter_type_stmt;
mod bool_expr;
mod boolean_test;
mod case_expr;
mod case_when;
mod cluster_stmt;
mod cmd_type;
mod coalesce_expr;
mod coercion_context;
mod collate_clause;
mod column_def;
mod column_ref;
mod comment_stmt;
mod common_table_expr;
mod composite_type_stmt;
mod constraint;
mod copy_stmt;
mod create_am_stmt;
mod create_cast_stmt;
mod create_conversion_stmt;
mod create_domain_stmt;
mod create_enum_stmt;
mod create_event_trig_stmt;
mod create_fdw_stmt;
mod create_foreign_server_stmt;
mod create_foreign_table_stmt;
mod create_function_stmt;
mod create_op_class_item;
mod create_op_class_stmt;
mod create_policy_stmt;
mod create_range_stmt;
mod create_role_stmt;
mod create_schema_stmt;
mod create_seq_stmt;
mod create_stmt;
mod create_table_as_stmt;
mod create_trig_stmt;
mod current_of_expr;
mod declare_cursor_stmt;
mod def_elem;
mod define_stmt;
mod delete_stmt;
mod discard_stmt;
mod do_stmt;
mod drop_behavior;
mod drop_role_stmt;
mod drop_stmt;
mod explain_stmt;
mod fetch_stmt;
mod func_call;
mod function_parameter;
mod grant_role_stmt;
mod grant_stmt;
mod index_elem;
mod index_stmt;
mod infer_clause;
mod insert_stmt;
mod into_clause;
mod join_expr;
mod listen_stmt;
mod lock_stmt;
mod locking_clause;
mod min_max_expr;
mod multi_assign_ref;
mod notify_stmt;
mod null_test;
mod object_type;
mod object_with_args;
mod on_commit_action;
mod on_conflict_clause;
mod partition_bound_spec;
mod partition_elem;
mod partition_spec;
mod prepare_stmt;
mod range_function;
mod range_subselect;
mod range_var;
mod rename_stmt;
mod res_target;
mod role_spec;
mod row_expr;
mod rule_stmt;
mod select_stmt;
mod set_to_default;
mod sort_by;
mod sort_by_dir;
mod sort_by_nulls;
mod sql_value_function;
mod sub_link;
mod table_like_clause;
mod transaction_stmt;
mod truncate_stmt;
mod type_cast;
mod type_name;
mod unlisten_stmt;
mod update_stmt;
mod vacuum_relation;
mod vacuum_stmt;
mod value;
mod variable_set_stmt;
mod variable_show_stmt;
mod vec_of_node;
mod view_check_option;
mod view_stmt;
mod window_def;
mod with_clause;

impl Sql for Node {
    #[track_caller]
    fn sql(&self) -> String {
        match self {
            Node::A_ArrayExpr(stmt) => stmt.sql(),
            Node::A_Const(stmt) => stmt.sql(),
            Node::A_Expr(stmt) => stmt.sql(),
            Node::A_Indices(stmt) => stmt.sql(),
            Node::A_Indirection(stmt) => stmt.sql(),
            Node::A_Star(stmt) => stmt.sql(),
            Node::AccessPriv(stmt) => stmt.sql(),
            Node::Alias(stmt) => stmt.sql(),
            Node::AlterCollationStmt(stmt) => stmt.sql(),
            Node::AlterFunctionStmt(stmt) => stmt.sql(),
            Node::AlterObjectSchemaStmt(stmt) => stmt.sql(),
            Node::AlterOwnerStmt(stmt) => stmt.sql(),
            Node::AlterTableCmd(stmt) => stmt.sql(),
            Node::AlterTableStmt(stmt) => stmt.sql(),
            Node::AlterTypeStmt(stmt) => stmt.sql(),
            Node::BoolExpr(stmt) => stmt.sql(),
            Node::BooleanTest(stmt) => stmt.sql(),
            Node::CaseExpr(stmt) => stmt.sql(),
            Node::CaseWhen(stmt) => stmt.sql(),
            Node::ClusterStmt(stmt) => stmt.sql(),
            Node::CoalesceExpr(stmt) => stmt.sql(),
            Node::CollateClause(stmt) => stmt.sql(),
            Node::ColumnDef(stmt) => stmt.sql(),
            Node::ColumnRef(stmt) => stmt.sql_ident(),
            Node::CommentStmt(stmt) => stmt.sql(),
            Node::CommonTableExpr(stmt) => stmt.sql(),
            Node::CompositeTypeStmt(stmt) => stmt.sql(),
            Node::Constraint(stmt) => stmt.sql(),
            Node::CopyStmt(stmt) => stmt.sql(),
            Node::CreateAmStmt(stmt) => stmt.sql(),
            Node::CreateCastStmt(stmt) => stmt.sql(),
            Node::CreateConversionStmt(stmt) => stmt.sql(),
            Node::CreateDomainStmt(stmt) => stmt.sql(),
            Node::CreateEnumStmt(stmt) => stmt.sql(),
            Node::CreateFdwStmt(stmt) => stmt.sql(),
            Node::CreateForeignServerStmt(stmt) => stmt.sql(),
            Node::CreateForeignTableStmt(stmt) => stmt.sql(),
            Node::CreateFunctionStmt(stmt) => {
                let mut stmt = stmt.clone();
                stmt.replace = true;
                stmt.sql()
            }
            Node::CreateOpClassItem(stmt) => stmt.sql(),
            Node::CreateOpClassStmt(stmt) => stmt.sql(),
            Node::CreatePolicyStmt(stmt) => stmt.sql(),
            Node::CreateRangeStmt(stmt) => stmt.sql(),
            Node::CreateRoleStmt(stmt) => stmt.sql(),
            Node::CreateSeqStmt(stmt) => stmt.sql(),
            Node::CreateTrigStmt(stmt) => stmt.sql(),
            Node::CreateSchemaStmt(stmt) => stmt.sql(),
            Node::CreateStmt(stmt) => stmt.sql(),
            Node::CreateTableAsStmt(stmt) => stmt.sql(),
            Node::CurrentOfExpr(stmt) => stmt.sql(),
            Node::DeclareCursorStmt(stmt) => stmt.sql(),
            Node::DefElem(stmt) => stmt.sql(),
            Node::DefineStmt(stmt) => stmt.sql(),
            Node::DeleteStmt(stmt) => stmt.sql(),
            Node::DiscardStmt(stmt) => stmt.sql(),
            Node::DoStmt(stmt) => stmt.sql(),
            Node::DropRoleStmt(stmt) => stmt.sql(),
            Node::DropStmt(stmt) => stmt.sql(),
            Node::ExplainStmt(node) => node.sql(),
            Node::Expr(node) => node.sql(),
            Node::FetchStmt(stmt) => stmt.sql(),
            Node::FuncCall(stmt) => stmt.sql(),
            Node::FunctionParameter(stmt) => stmt.sql(),
            Node::GrantRoleStmt(stmt) => stmt.sql(),
            Node::GrantStmt(stmt) => stmt.sql(),
            Node::IndexElem(stmt) => stmt.sql(),
            Node::IndexStmt(stmt) => stmt.sql(),
            Node::InferClause(stmt) => stmt.sql(),
            Node::InsertStmt(stmt) => stmt.sql(),
            Node::IntoClause(stmt) => stmt.sql(),
            Node::JoinExpr(stmt) => stmt.sql(),
            Node::List(_) => String::new(),
            Node::ListenStmt(stmt) => stmt.sql(),
            Node::LockStmt(stmt) => stmt.sql(),
            Node::LockingClause(stmt) => stmt.sql(),
            Node::MultiAssignRef(stmt) => stmt.sql(),
            Node::NotifyStmt(stmt) => stmt.sql(),
            Node::NullTest(stmt) => stmt.sql(),
            Node::MinMaxExpr(stmt) => stmt.sql(),
            Node::ObjectWithArgs(stmt) => stmt.sql(),
            Node::OnConflictClause(stmt) => stmt.sql(),
            Node::PartitionBoundSpec(stmt) => stmt.sql(),
            Node::PartitionElem(stmt) => stmt.sql(),
            Node::PartitionSpec(stmt) => stmt.sql(),
            Node::PrepareStmt(stmt) => stmt.sql(),
            Node::RangeFunction(stmt) => stmt.sql(),
            Node::RangeSubselect(stmt) => stmt.sql(),
            Node::RangeVar(stmt) => stmt.sql(),
            Node::RenameStmt(stmt) => stmt.sql(),
            Node::ResTarget(_) => unreachable!("encountered a ResTarget node"),
            Node::RoleSpec(stmt) => stmt.sql(),
            Node::RuleStmt(stmt) => stmt.sql(),
            Node::RowExpr(stmt) => stmt.sql(),
            Node::SelectStmt(stmt) => stmt.sql(),
            Node::SetToDefault(stmt) => stmt.sql(),
            Node::SortBy(stmt) => stmt.sql(),
            Node::SQLValueFunction(stmt) => stmt.sql(),
            Node::SubLink(stmt) => stmt.sql(),
            Node::TableLikeClause(stmt) => stmt.sql(),
            Node::TransactionStmt(stmt) => stmt.sql(),
            Node::TruncateStmt(stmt) => stmt.sql(),
            Node::TypeCast(stmt) => stmt.sql(),
            Node::TypeName(stmt) => stmt.sql(),
            Node::UnlistenStmt(stmt) => stmt.sql(),
            Node::UpdateStmt(stmt) => stmt.sql(),
            Node::VacuumRelation(stmt) => stmt.sql(),
            Node::VacuumStmt(stmt) => stmt.sql(),
            Node::Value(stmt) => stmt.sql(),
            Node::VariableSetStmt(stmt) => stmt.sql(),
            Node::VariableShowStmt(stmt) => stmt.sql(),
            Node::ViewStmt(stmt) => stmt.sql(),
            Node::WindowDef(stmt) => stmt.sql(),
            Node::WithClause(stmt) => stmt.sql(),
            Node::CreateEventTrigStmt(stmt) => stmt.sql(),

            _ => unimplemented!("Node: {:?}", self),
        }
    }
}
