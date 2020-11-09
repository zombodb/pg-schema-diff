use crate::EMPTY_NODE_VEC;
use postgres_parser::{quote_identifier, Node, SqlStatementScanner};

use std::collections::HashSet;
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
    fn alter(&self, other: &Node) -> Option<String>;
    fn drop(&self) -> String;
    fn name(&self, sql: &str) -> String;
}

pub trait Sql {
    fn sql_prefix(&self, pre: &str) -> String {
        format!("{}{}", pre, self.sql())
    }

    fn sql_wrap(&self, pre: &str, post: &str) -> String {
        format!("{}{}{}", pre, self.sql(), post)
    }

    fn sql(&self) -> String;
}

pub trait SqlList {
    fn sql_prefix(&self, pre: &str, sep: &str) -> String;
    fn sql(&self, sep: &str) -> String;
    fn sql_wrap_each(&self, pre: Option<&str>, post: Option<&str>) -> String;
    fn sql_wrap(&self, pre: Option<&str>, post: Option<&str>) -> String;
}

pub trait SqlIdent {
    fn sql(&self) -> String;
    fn sql_suffix(&self, suf: &str) -> String;
}

pub trait SqlCollect {
    fn sql(self) -> String;
}

impl<T: Sql> Sql for Option<Box<T>> {
    fn sql_prefix(&self, pre: &str) -> String {
        match self {
            None => String::new(),
            Some(boxed_sql) => format!("{}{}", pre, boxed_sql.sql()),
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
    fn sql(&self) -> String {
        quote_identifier(self)
    }

    fn sql_suffix(&self, suf: &str) -> String {
        match self {
            None => String::new(),
            Some(_) => format!("{}{}", self.sql(), suf),
        }
    }
}

pub trait Len {
    fn len(&self) -> usize;
}

impl Len for Option<Vec<Node>> {
    fn len(&self) -> usize {
        self.as_ref().unwrap_or(&EMPTY_NODE_VEC).len()
    }
}

pub struct SchemaSet {
    nodes: HashSet<DiffableStatement>,
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

            _ => false,
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
            sql.push_str(&node.node.sql());
            sql.push('\n');
        }

        sql
    }

    #[allow(dead_code)]
    pub fn diff(self, other: &mut SchemaSet) -> String {
        let mut sql = String::new();

        for node in &self.nodes {
            match other.nodes.get(&node) {
                // it's in the other one too, so we need to diff it
                Some(other_node) => {
                    if node.node != other_node.node {
                        if let Some(alter) = node.differ.alter(&other_node.node) {
                            sql.push_str(&alter);
                        }

                        // we don't need it anymore
                        other.nodes.remove(&node);
                    }
                }

                // it's not in the other one, so we use it as is
                None => {
                    sql.push_str(&node.sql);
                }
            }
        }

        sql
    }
}
