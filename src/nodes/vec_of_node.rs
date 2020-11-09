use crate::schema_set::{Sql, SqlCollect, SqlList};

use postgres_parser::Node;

impl Sql for Vec<Node> {
    fn sql(&self) -> String {
        self.iter().sql()
    }
}

impl SqlList for Option<Vec<Node>> {
    fn sql_prefix(&self, pre: &str, sep: &str) -> String {
        match self {
            None => String::new(),
            Some(_) => format!("{}{}", pre, self.sql(sep)),
        }
    }

    fn sql(&self, sep: &str) -> String {
        match self {
            None => String::new(),
            Some(v) => {
                let mut sql = String::new();
                for (i, node) in v.iter().enumerate() {
                    if i > 0 {
                        sql.push_str(sep);
                    }
                    sql.push_str(&node.sql());
                }

                sql
            }
        }
    }

    fn sql_wrap_each(&self, pre: Option<&str>, post: Option<&str>) -> String {
        match self {
            None => String::new(),
            Some(v) => {
                let mut sql = String::new();
                for node in v.iter() {
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
        }
    }

    fn sql_wrap(&self, pre: Option<&str>, post: Option<&str>) -> String {
        match self {
            None => String::new(),
            Some(v) => {
                if pre.is_some() && post.is_some() {
                    let mut sql = String::new();

                    if let Some(pre) = pre {
                        sql.push_str(pre);
                    }
                    sql.push_str(&v.sql());
                    if let Some(post) = post {
                        sql.push_str(post);
                    }

                    sql
                } else {
                    v.sql()
                }
            }
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
