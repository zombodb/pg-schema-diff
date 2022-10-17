use crate::schema_set::{SchemaSet, Sql, SqlIdent};
use postgres_parser::*;

mod nodes;
mod schema_set;

static EMPTY_NODE_VEC: Vec<Node> = Vec::new();

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).expect("no command argument");

    match command.as_str() {
        "deparse" => {
            let filename = args.get(2).expect("noo filename argument");
            let mut set = SchemaSet::new();
            set.scan_file(&filename);
            let deparsed = set.deparse();
            println!("{}", deparsed);
        }

        "diff" => {
            let a = args.get(2).expect("no a filename");
            let b = args.get(3).expect("no b filename");

            let mut a_set = SchemaSet::new();
            let mut b_set = SchemaSet::new();

            a_set.scan_file(&a);
            b_set.scan_file(&b);

            let differences = a_set.diff(&b_set);
            println!("{}", differences);
        }

        unknown => panic!("unrecognized command argument: {}", unknown),
    }
}

pub fn make_individual_names(names: &Option<Vec<Node>>) -> Vec<String> {
    let mut result = Vec::new();

    if names.is_some() {
        for name in names.as_ref().unwrap() {
            result.push(
                make_name(&Some(vec![name.clone()])).expect("failed to make an individual name"),
            );
        }
    }

    result
}

pub fn make_name(names: &Option<Vec<Node>>) -> Result<String, PgParserError> {
    match names {
        Some(names) => {
            let mut result = String::new();
            for name in names {
                if !result.is_empty() {
                    result.push('.');
                }

                match name {
                    crate::Node::Value(value) if value.string.is_some() => {
                        let ident = value.string.sql_ident();
                        // if &ident != "pg_catalog" {
                            result.push_str(&value.string.sql_ident());
                        // }
                    }
                    crate::Node::A_Star(a_star) => {
                        result.push_str(&a_star.sql());
                    }
                    _ => return Err(PgParserError::NotAString),
                }
            }
            Ok(result)
        }

        // None => Err(PgParserError::InternalNull),
        None => Ok("".into()),
    }
}

pub fn make_operator_name(names: &Option<Vec<Node>>) -> Result<String, PgParserError> {
    match names {
        Some(names) => {
            if names.len() == 1 {
                return Ok(names[0].sql());
            }

            let mut result = String::new();
            result.push_str("OPERATOR(");
            let mut iter = names.iter().enumerate().peekable();
            while let Some((i, name)) = iter.next() {
                if i > 0 {
                    result.push('.');
                }

                match name {
                    crate::Node::Value(value) if value.string.is_some() => {
                        if iter.peek().is_none() {
                            // don't quote the operator itself
                            result.push_str(&value.string.as_ref().unwrap());
                        } else {
                            result.push_str(&value.string.sql_ident());
                        }
                    }
                    crate::Node::A_Star(a_star) => {
                        result.push_str(&a_star.sql());
                    }
                    _ => return Err(PgParserError::NotAString),
                }
            }
            result.push(')');
            Ok(result)
        }

        None => Err(PgParserError::InternalNull),
    }
}

pub fn get_string_value(node: &Node) -> &Option<String> {
    if let Node::Value(value) = node {
        return &value.string;
    }
    return &None;
}

pub fn get_float_value(node: &Node) -> &Option<String> {
    if let Node::Value(value) = node {
        return &value.float;
    }
    return &None;
}

pub fn get_int_value(node: &Node) -> &Option<i32> {
    if let Node::Value(value) = node {
        return &value.int;
    }
    return &None;
}

pub fn get_bool_value(node: &Node) -> bool {
    if let Node::Value(value) = node {
        return value.int == Some(1);
    }
    return false;
}
