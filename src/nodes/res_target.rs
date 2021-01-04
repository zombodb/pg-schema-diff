use crate::schema_set::{Sql, SqlIdent, SqlList};
use crate::EMPTY_NODE_VEC;

use postgres_parser::Node;

pub fn res_target_select(targets: &Option<Vec<Node>>) -> String {
    let mut sql = String::new();

    for (i, node) in targets
        .as_ref()
        .unwrap_or(&EMPTY_NODE_VEC)
        .iter()
        .enumerate()
    {
        if let Node::ResTarget(node) = node {
            if i > 0 {
                sql.push_str(", ");
            }

            sql.push_str(&node.val.sql());
            sql.push_str(&node.indirection.sql_wrap_each(Some("["), Some("]")));
            if node.name.is_some() {
                sql.push_str(" AS ");
                sql.push_str(&node.name.sql_ident());
            }
        } else {
            panic!("unexpected node: {:?}", node);
        }
    }

    sql
}

pub fn res_target_insert(targets: &Option<Vec<Node>>) -> String {
    match targets {
        None => String::new(),
        Some(targets) => {
            let mut sql = String::new();

            sql.push('(');
            for (i, node) in targets.iter().enumerate() {
                if let Node::ResTarget(node) = node {
                    if i > 0 {
                        sql.push_str(", ");
                    }

                    sql.push_str(&node.name.sql_ident());
                    sql.push_str(&node.indirection.sql_wrap_each(Some("["), Some("]")));
                } else {
                    panic!("unexpected node: {:?}", node);
                }
            }
            sql.push(')');

            sql
        }
    }
}

pub fn res_target_update(targets: &Option<Vec<Node>>) -> String {
    let mut sql = String::new();

    let mut iter = targets
        .as_ref()
        .unwrap_or(&EMPTY_NODE_VEC)
        .iter()
        .map(|n| match n {
            Node::ResTarget(ref res_target) => res_target,
            _ => panic!("unexpected node {:?} in update targetList"),
        });

    let mut i = 0;
    loop {
        let mut current = iter.next();

        if current.is_none() {
            break;
        }
        let mut mars = Vec::new();
        while let Node::MultiAssignRef(multi_assign_ref) =
            current.as_ref().unwrap().val.as_ref().unwrap().as_ref()
        {
            mars.push((multi_assign_ref, current.unwrap()));
            current = iter.next();
            if current.is_none() {
                break;
            }
        }

        if i > 0 {
            sql.push_str(", ");
        }
        let current_i = i;
        if !mars.is_empty() {
            sql.push('(');
            for (_, ref_target) in &mars {
                if i > 0 && i > current_i {
                    sql.push_str(", ");
                }
                sql.push_str(&ref_target.name.sql_ident());
                sql.push_str(&ref_target.indirection.sql_wrap_each(Some("["), Some("]")));
                i += 1;
            }
            sql.push_str(") = ");

            sql.push_str(&mars[0].0.source.sql());
        }

        if let Some(node) = current {
            if i > 0 && i > current_i {
                sql.push_str(", ");
            }
            sql.push_str(&node.name.sql_ident());
            sql.push_str(&node.indirection.sql_wrap_each(Some("["), Some("]")));
            sql.push_str(" = ");
            sql.push_str(&node.val.sql());

            i += 1;
        }
    }

    sql
}

pub fn res_target_returning(targets: &Option<Vec<Node>>) -> String {
    match targets {
        None => String::new(),
        Some(targets) => {
            let mut sql = String::new();

            sql.push_str(" RETURNING ");

            for (i, node) in targets.iter().enumerate() {
                if let Node::ResTarget(node) = node {
                    if i > 0 {
                        sql.push_str(", ");
                    }

                    sql.push_str(&node.val.sql());

                    if node.name.is_some() {
                        sql.push_str(" AS ");
                        sql.push_str(&node.name.sql_ident());
                    }
                } else {
                    panic!("unexpected node {:?}", node);
                }
            }

            sql
        }
    }
}
