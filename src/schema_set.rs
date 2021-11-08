use crate::{make_name, EMPTY_NODE_VEC};
use postgres_parser::{parse_query, quote_identifier, Node, SqlStatementScanner};

use colored::Colorize;
use std::borrow::Cow;

use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::panic::{catch_unwind, RefUnwindSafe};

#[derive(Debug)]
pub struct DiffableStatement {
    tree_string: String,
    sql: String,
    node: Node,
    differ: Box<dyn Diff>,
}

impl Hash for DiffableStatement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.differ.identifier(&self.tree_string).hash(state)
    }
}

impl Eq for DiffableStatement {}
impl PartialEq for DiffableStatement {
    fn eq(&self, other: &Self) -> bool {
        self.differ
            .identifier(&self.tree_string)
            .eq(&other.differ.identifier(&other.tree_string))
    }
}

impl RefUnwindSafe for DiffableStatement {}

impl DiffableStatement {
    fn new(sql: &str, node: Node, differ: impl Diff + 'static) -> Self {
        let mut sql = sql.trim();
        if sql.ends_with(';') {
            sql = &sql[..sql.len() - 1];
        }
        DiffableStatement {
            tree_string: format!("{:?}", node),
            sql: sql.trim().into(),
            node,
            differ: Box::new(differ),
        }
    }
}

pub trait Diff: Sql + Debug {
    fn alter_stmt(&self, _other: &Node) -> Option<String> {
        unimplemented!("Don't know how to ALTER:\n{}\n{:?}\n{:?}", self.sql(), _other, self);
    }

    fn drop_stmt(&self) -> Option<String> {
        unimplemented!("Don't know how to drop: {:#?}", self)
    }

    fn object_name(&self) -> Option<String> {
        None
    }

    fn object_type(&self) -> String { String::new() }

    fn identifier<'a>(&self, tree_string: &'a str) -> Cow<'a, str> {
        match self.object_name() {
            Some(name) => Cow::Owned(name + &self.object_type()),
            None => Cow::Borrowed(tree_string),
        }
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
    nodes: indexmap::IndexSet<DiffableStatement>,
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
            nodes: &mut indexmap::IndexSet<DiffableStatement>,
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
            Node::CreateForeignServerStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateForeignTableStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateFdwStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateFunctionStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateOpClassStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreatePolicyStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateRangeStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateRoleStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::CreateSeqStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
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
            Node::LockStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::NotifyStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::PrepareStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::RenameStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
            Node::RuleStmt(stmt) => push(&mut self.nodes, sql, node, stmt),
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

                // it couldn't be parsed -- panic!
                Err(e) => {
                    panic!("INPUT PARSE ERROR: {:?}\n{}\n/---", e, stmt.sql.trim());
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

    pub fn diff(self, that: &SchemaSet) -> String {
        let mut sql = String::new();

        // Find objects in 'self' that don't exist in 'that' so we can DROP them
        for this_node in &self.nodes {
            if !that.nodes.contains(this_node) {
                match this_node.differ.drop_stmt() {
                    Some(drop) => {
                        sql.push_str(&drop);
                        sql.push_str(";\n");
                    }
                    None => {
                        // it's a statement that we don't know how to drop
                    }
                }
            }
        }

        // find objects that are either in both or only in 'that'
        for that_node in &that.nodes {
            // do we have that_node?
            match self.nodes.get(that_node) {
                // yes, we do have that node, so lets see if it's different
                Some(this_node) => {
                    if &this_node.node.sql() != &that_node.node.sql() {
                        // they are different, so we try to alter it
                        if let Some(alter) = this_node.differ.alter_stmt(&that_node.node) {
                            sql.push_str(&alter);
                            sql.push_str(";\n");
                        }
                    }
                }

                // no, we don't have that node, so we need to just its sql directly
                None => {
                    sql.push_str(&that_node.sql);
                    sql.push_str(";\n");
                }
            }
        }

        sql
    }
}
