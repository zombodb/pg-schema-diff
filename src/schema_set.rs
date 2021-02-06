use crate::{make_name, EMPTY_NODE_VEC};
use postgres_parser::{parse_query, quote_identifier, Node, SqlStatementScanner};

use colored::Colorize;
use indexmap::set::IndexSet;
use std::hash::{Hash, Hasher};
use std::panic::{catch_unwind, RefUnwindSafe};

pub struct DiffableStatement {
    sql: String,
    node: Node,
    differ: Box<dyn Diff>,
}

impl Hash for DiffableStatement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.differ.name(&self.sql).hash(state)
    }
}

impl Eq for DiffableStatement {}
impl PartialEq for DiffableStatement {
    fn eq(&self, other: &Self) -> bool {
        self.differ
            .name(&self.sql)
            .eq(&other.differ.name(&other.sql))
    }
}

impl RefUnwindSafe for DiffableStatement {}

impl DiffableStatement {
    fn new(sql: &str, node: Node, differ: impl Diff + 'static) -> Self {
        DiffableStatement {
            sql: sql.into(),
            node,
            differ: Box::new(differ),
        }
    }
}

pub trait Diff: Sql {
    fn alter(&self, _other: &Node) -> Option<String> {
        None
    }
    fn drop(&self) -> String {
        unimplemented!()
    }
    fn name(&self, _sql: &str) -> String {
        _sql.into()
    }
}

pub trait SqlMaybeList {
    fn sql_maybe_list(&self, sep: &str) -> String;
}

impl SqlMaybeList for Option<Box<Node>> {
    fn sql_maybe_list(&self, sep: &str) -> String {
        match self {
            None => String::new(),
            Some(boxed_sql) => boxed_sql.sql_maybe_list(sep),
        }
    }
}

impl SqlMaybeList for Node {
    fn sql_maybe_list(&self, sep: &str) -> String {
        if let Node::List(v) = self {
            v.sql(sep)
        } else {
            self.sql()
        }
    }
}

pub trait Sql {
    fn sql_prefix(&self, pre: &str) -> String {
        format!("{}{}", pre, self.sql())
    }
    #[track_caller]
    fn sql_wrap(&self, pre: &str, post: &str) -> String {
        format!("{}{}{}", pre, self.sql(), post)
    }
    fn sql_prefix_and_wrap(&self, pre: &str, start: &str, end: &str) -> String {
        format!("{}{}{}{}", pre, start, self.sql(), end)
    }
    fn sql(&self) -> String;
}

pub trait SqlList {
    fn sql(&self, sep: &str) -> String;
    fn sql_prefix(&self, pre: &str, sep: &str) -> String;
    fn sql_prefix_and_wrap(&self, pre: &str, start: &str, end: &str, sep: &str) -> String;
    fn sql_wrap_each(&self, pre: Option<&str>, post: Option<&str>) -> String;
    fn sql_wrap_each_and_separate(&self, sep: &str, pre: &str, post: &str) -> String;
    fn sql_wrap(&self, sep: &str, pre: &str, post: &str) -> String;
}

pub trait SqlIdent {
    fn sql_ident(&self) -> String;
    fn sql_ident_prefix(&self, pre: &str) -> String;
    fn sql_ident_suffix(&self, suf: &str) -> String;
}

pub trait SqlCollect {
    fn sql_wrap(self, pre: &str, post: &str) -> String;
    fn sql(self) -> String;
}

impl<T: Sql> Sql for Option<Box<T>> {
    fn sql_prefix(&self, pre: &str) -> String {
        match self {
            None => String::new(),
            Some(boxed_sql) => format!("{}{}", pre, boxed_sql.sql()),
        }
    }

    fn sql_wrap(&self, pre: &str, post: &str) -> String {
        match self {
            None => String::new(),
            Some(boxed_sql) => boxed_sql.sql_wrap(pre, post),
        }
    }

    fn sql_prefix_and_wrap(&self, pre: &str, start: &str, end: &str) -> String {
        match self {
            None => String::new(),
            Some(boxed_sql) => format!("{}{}{}{}", pre, start, boxed_sql.sql(), end),
        }
    }

    #[track_caller]
    fn sql(&self) -> String {
        match self {
            None => String::new(),
            Some(boxed_sql) => boxed_sql.sql(),
        }
    }
}

impl SqlIdent for Option<String> {
    fn sql_ident(&self) -> String {
        quote_identifier(self)
    }

    fn sql_ident_prefix(&self, pre: &str) -> String {
        match self {
            None => String::new(),
            Some(_) => format!("{}{}", pre, self.sql_ident()),
        }
    }

    fn sql_ident_suffix(&self, suf: &str) -> String {
        match self {
            None => String::new(),
            Some(_) => format!("{}{}", self.sql_ident(), suf),
        }
    }
}

impl SqlIdent for Option<Vec<Node>> {
    #[track_caller]
    fn sql_ident(&self) -> String {
        make_name(self).expect("unable to make SqlIdent")
    }

    #[track_caller]
    fn sql_ident_prefix(&self, pre: &str) -> String {
        match self {
            None => String::new(),
            Some(_) => format!("{}{}", pre, self.sql_ident()),
        }
    }

    #[track_caller]
    fn sql_ident_suffix(&self, suf: &str) -> String {
        match self {
            None => String::new(),
            Some(_) => format!("{}{}", self.sql_ident(), suf),
        }
    }
}

impl SqlIdent for Vec<Node> {
    #[track_caller]
    fn sql_ident(&self) -> String {
        make_name(&Some(self.clone())).expect("unable to make SqlIdent")
    }

    #[track_caller]
    fn sql_ident_prefix(&self, pre: &str) -> String {
        format!("{}{}", pre, self.sql_ident())
    }

    #[track_caller]
    fn sql_ident_suffix(&self, suf: &str) -> String {
        format!("{}{}", self.sql_ident(), suf)
    }
}

impl SqlIdent for Option<Box<Node>> {
    #[track_caller]
    fn sql_ident(&self) -> String {
        match self {
            None => String::new(),
            Some(node) => {
                make_name(&Some(vec![node.as_ref().clone()])).expect("unable to make SqlIdent")
            }
        }
    }

    #[track_caller]
    fn sql_ident_prefix(&self, pre: &str) -> String {
        match self {
            None => String::new(),
            Some(_) => format!("{}{}", pre, self.sql_ident()),
        }
    }

    #[track_caller]
    fn sql_ident_suffix(&self, suf: &str) -> String {
        match self {
            None => String::new(),
            Some(_) => format!("{}{}", self.sql_ident(), suf),
        }
    }
}

impl SqlIdent for Node {
    #[track_caller]
    fn sql_ident(&self) -> String {
        make_name(&Some(vec![self.clone()])).expect("unable to make SqlIdent")
    }

    #[track_caller]
    fn sql_ident_prefix(&self, pre: &str) -> String {
        format!("{}{}", pre, self.sql_ident())
    }

    #[track_caller]
    fn sql_ident_suffix(&self, suf: &str) -> String {
        format!("{}{}", suf, self.sql_ident())
    }
}

pub trait Len {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Len for Option<Vec<Node>> {
    fn len(&self) -> usize {
        self.as_ref().unwrap_or(&EMPTY_NODE_VEC).len()
    }
}

pub struct SchemaSet {
    nodes: IndexSet<DiffableStatement>,
}

impl Default for SchemaSet {
    fn default() -> Self {
        SchemaSet {
            nodes: Default::default(),
        }
    }
}

impl SchemaSet {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, sql: &str, node: Node) {
        #[inline]
        fn push(
            nodes: &mut IndexSet<DiffableStatement>,
            sql: &str,
            node: Node,
            differ: impl Diff + 'static,
        ) {
            nodes.insert(DiffableStatement::new(sql, node, differ));
        }

        match node.clone() {
            Node::AlterCollationStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::AlterFunctionStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::AlterObjectSchemaStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::AlterOwnerStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::AlterTableStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::ClusterStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CommentStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CompositeTypeStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CopyStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateAmStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateCastStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateConversionStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateDomainStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateEnumStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateFunctionStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateOpClassStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreatePolicyStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateRangeStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateRoleStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateSchemaStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateTableAsStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateTrigStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::DeclareCursorStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::DefineStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::DeleteStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::DiscardStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::DoStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::DropRoleStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::DropStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::ExplainStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::FetchStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::GrantRoleStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::GrantStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::IndexStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::InsertStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::ListenStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::NotifyStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::PrepareStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::RenameStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::SelectStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::TransactionStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::TruncateStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::UnlistenStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::UpdateStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::VacuumStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::VariableSetStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::VariableShowStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::ViewStmt(stmt) => push(&mut self.nodes, sql, node, stmt),

            _ => panic!("unknown node: {:?}\n\n{}", node, sql),
        };
    }

    pub fn scan_file(&mut self, filename: &str) {
        let mut sql =
            std::fs::read_to_string(filename).expect(&format!("failed to read file: {}", filename));
        sql = sql.replace("@extschema@", "\"@extschema@\"");
        let scanner = SqlStatementScanner::new(&sql);
        for stmt in scanner.into_iter() {
            match stmt.parsetree {
                Ok(parsetree) => {
                    if let Some(node) = parsetree {
                        self.push(stmt.sql, node);
                    }
                }

                // it couldn't be parsed.  Just display the underlying parse error
                Err(_e) => {
                    // eprintln!("---\nINPUT PARSE ERROR: {:?}\n{}\n/---", e, stmt.sql.trim());
                }
            };
        }
    }

    pub fn deparse(&self) -> String {
        let mut sql = String::new();

        for node in &self.nodes {
            if let Node::AlterTableStmt(_) = &node.node {
                println!("skipping AlterTableStmt");
                continue;
            }
            // println!("{}", node.sql);
            let deparsed = match catch_unwind(|| node.node.sql()) {
                Ok(deparsed) => deparsed,
                Err(e) => {
                    panic!(
                        "{:?}\n\n\nnode=\n{:#?}\nsql={}",
                        e,
                        node.node,
                        node.sql.trim(),
                    )
                }
            };
            let reparsed = parse_query(&deparsed).unwrap_or_else(|e| {
                panic!(
                    "FAILED TO REPARSE:\n{:#?}\n{:#?}\nORIG:\n   {}\nNEW:\n   {};",
                    e, node.node, node.sql, deparsed,
                )
            });
            if &node.node != reparsed.get(0).expect("didn't parse anything") {
                panic!(
                    "TREES NOT EQUAL:{:#?};\n---------\n{:#?};\n\n\nORIG:\n   {}\nNEW:\n   {};\n",
                    node.node,
                    reparsed.get(0).unwrap(),
                    node.sql,
                    deparsed
                );
            }

            sql.push_str("==================\n");
            sql.push_str(&format!("{}:\n{}\n", "BEFORE".yellow(), node.sql.trim()));
            sql.push_str(&format!("{}:\n{};\n", "AFTER".green(), deparsed.trim()));
            sql.push_str("/=================\n");
        }

        sql
    }

    #[allow(dead_code)]
    pub fn diff(self, that: &SchemaSet) -> String {
        let mut sql = String::new();

        for this_node in &self.nodes {
            match that.nodes.get(this_node) {
                // it's in 'that' one too, so we need to diff it
                Some(that_node) => {
                    if this_node.node != that_node.node {
                        // try to turn that_node into this_node
                        if let Some(alter) = that_node.differ.alter(&this_node.node) {
                            sql.push_str(&alter);
                        }
                    }
                }

                // it's not in the other one, so we use it as is
                None => {
                    sql.push_str(&this_node.sql);
                }
            }
        }

        sql
    }
}
