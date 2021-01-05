use crate::schema_set::{Sql, SqlCollect, SqlList, SqlMaybeList};

use postgres_parser::Node;

fn render_node(node: &Node, sql: &mut String, sep: &str) {
    if let Node::List(node) = node {
        sql.push_str(&node.sql(sep));
    } else {
        sql.push_str(&node.sql());
    }
}

impl SqlList for Vec<Node> {
    #[track_caller]
    fn sql(&self, sep: &str) -> String {
        let mut sql = String::new();
        for (i, node) in self.iter().enumerate() {
            if i > 0 {
                sql.push_str(sep);
            }
            render_node(node, &mut sql, sep);
        }

        sql
    }

    fn sql_prefix(&self, pre: &str, sep: &str) -> String {
        format!("{}{}", pre, self.sql(sep))
    }

    fn sql_prefix_and_wrap(&self, pre: &str, start: &str, end: &str, sep: &str) -> String {
        format!("{}{}{}{}", pre, start, self.sql(sep), end)
    }

    fn sql_wrap_each(&self, pre: Option<&str>, post: Option<&str>) -> String {
        let mut sql = String::new();
        for node in self.iter() {
            if let Some(pre) = pre.as_ref() {
                sql.push_str(pre);
            }
            sql.push_str(&node.sql());
            if let Some(post) = post.as_ref() {
                sql.push_str(post);
            }
        }

        sql
    }

    fn sql_wrap_each_and_separate(&self, sep: &str, pre: &str, post: &str) -> String {
        let mut sql = String::new();
        for (i, node) in self.iter().enumerate() {
            if i > 0 {
                sql.push_str(sep);
            }

            sql.push_str(pre);
            sql.push_str(&node.sql_maybe_list(sep));
            sql.push_str(post);
        }

        sql
    }

    fn sql_wrap(&self, sep: &str, pre: &str, post: &str) -> String {
        let mut sql = String::new();
        sql.push_str(pre);
        for (i, node) in self.iter().enumerate() {
            if i > 0 {
                sql.push_str(sep);
            }

            sql.push_str(&node.sql());
        }
        sql.push_str(post);

        sql
    }
}

impl SqlList for Option<Vec<Node>> {
    fn sql(&self, sep: &str) -> String {
        match self {
            None => String::new(),
            Some(v) => v.sql(sep),
        }
    }

    fn sql_prefix(&self, pre: &str, sep: &str) -> String {
        match self {
            None => String::new(),
            Some(v) => v.sql_prefix(pre, sep),
        }
    }

    fn sql_prefix_and_wrap(&self, pre: &str, start: &str, end: &str, sep: &str) -> String {
        match self {
            None => String::new(),
            Some(v) => v.sql_prefix_and_wrap(pre, start, end, sep),
        }
    }

    fn sql_wrap_each(&self, pre: Option<&str>, post: Option<&str>) -> String {
        match self {
            None => String::new(),
            Some(v) => v.sql_wrap_each(pre, post),
        }
    }

    fn sql_wrap_each_and_separate(&self, sep: &str, pre: &str, post: &str) -> String {
        match self {
            None => String::new(),
            Some(v) => v.sql_wrap_each_and_separate(sep, pre, post),
        }
    }

    fn sql_wrap(&self, sep: &str, pre: &str, post: &str) -> String {
        match self {
            None => String::new(),
            Some(v) => v.sql_wrap(sep, pre, post),
        }
    }
}

impl<'a, I: Iterator<Item = &'a Node>> SqlCollect for I {
    fn sql_wrap(self, pre: &str, post: &str) -> String {
        format!("{}{}{}", pre, self.sql(), post)
    }
    fn sql(self) -> String {
        let mut sql = String::new();

        for (i, n) in self.map(|n| n.sql()).enumerate() {
            if i > 0 {
                sql.push_str(", ");
            }
            sql.push_str(&n);
        }

        sql
    }
}
