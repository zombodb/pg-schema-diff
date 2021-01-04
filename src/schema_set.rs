use crate::{make_name, EMPTY_NODE_VEC};
use postgres_parser::{parse_query, quote_identifier, Node, SqlStatementScanner};

use indexmap::set::IndexSet;
use std::hash::{Hash, Hasher};

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
    fn alter(&self, _other: &Node) -> Option<String>;
    fn drop(&self) -> String;
    fn name(&self, _sql: &str) -> String;
}

pub trait Sql {
    fn sql_prefix(&self, pre: &str) -> String {
        format!("{}{}", pre, self.sql())
    }
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

    fn sql_prefix_and_wrap(&self, pre: &str, start: &str, end: &str) -> String {
        match self {
            None => String::new(),
            Some(boxed_sql) => format!("{}{}{}{}", pre, start, boxed_sql.sql(), end),
        }
    }

    fn sql(&self) -> String {
        match self {
            None => String::new(),
            Some(boxed_sql) => boxed_sql.sql(),
        }
    }
}

impl SqlIdent for Option<String> {
    fn sql_ident(&self) -> String {
        match self {
            Some(ident) => {
                for _ in ident.chars().filter(|c| {
                    ![
                        '+', '-', '*', '/', '<', '>', '=', '~', '!', '@', '#', '%', '^', '&', '|',
                        '`', '?',
                    ]
                    .contains(c)
                }) {
                    return quote_identifier(self);
                }

                return ident.clone();
            }
            None => quote_identifier(self),
        }
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
        match node.clone() {
            Node::CreateFunctionStmt(stmt) => {
                self.nodes.insert(DiffableStatement::new(sql, node, stmt))
            }

            Node::CreateStmt(stmt) => self.nodes.insert(DiffableStatement::new(sql, node, stmt)),
            Node::DeleteStmt(stmt) => self.nodes.insert(DiffableStatement::new(sql, node, stmt)),
            Node::InsertStmt(stmt) => self.nodes.insert(DiffableStatement::new(sql, node, stmt)),
            Node::SelectStmt(stmt) => self.nodes.insert(DiffableStatement::new(sql, node, stmt)),
            Node::UpdateStmt(stmt) => self.nodes.insert(DiffableStatement::new(sql, node, stmt)),

            Node::DoStmt(stmt) => self.nodes.insert(DiffableStatement::new(sql, node, stmt)),
            Node::CreateSchemaStmt(stmt) => {
                self.nodes.insert(DiffableStatement::new(sql, node, stmt))
            }
            Node::GrantStmt(stmt) => self.nodes.insert(DiffableStatement::new(sql, node, stmt)),
            Node::GrantRoleStmt(stmt) => self.nodes.insert(DiffableStatement::new(sql, node, stmt)),
            Node::CreateDomainStmt(stmt) => {
                self.nodes.insert(DiffableStatement::new(sql, node, stmt))
            }
            Node::CreateEnumStmt(stmt) => {
                self.nodes.insert(DiffableStatement::new(sql, node, stmt))
            }
            Node::DefineStmt(stmt) => self.nodes.insert(DiffableStatement::new(sql, node, stmt)),
            Node::CreateCastStmt(stmt) => {
                self.nodes.insert(DiffableStatement::new(sql, node, stmt))
            }
            Node::CreateAmStmt(stmt) => self.nodes.insert(DiffableStatement::new(sql, node, stmt)),
            Node::CreateOpClassStmt(stmt) => {
                self.nodes.insert(DiffableStatement::new(sql, node, stmt))
            }
            Node::ViewStmt(stmt) => self.nodes.insert(DiffableStatement::new(sql, node, stmt)),

            _ => panic!("unknown node: {:?}", node),
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
                Err(e) => {
                    eprintln!("{}", stmt.sql);
                    eprintln!("-- ERROR:  {:?}", e);
                }
            };
        }
    }

    pub fn deparse(&self) -> String {
        let mut sql = String::new();

        for node in &self.nodes {
            let deparsed = node.node.sql();
            let reparsed = parse_query(&deparsed).unwrap_or_else(|e| {
                panic!(
                    "FAILED TO REPARSE:\n{:#?}\nORIG:\n   {}\nNEW:\n   {};",
                    e, node.sql, deparsed
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

            sql.push_str(&node.node.sql());
            sql.push_str(";\n");
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
