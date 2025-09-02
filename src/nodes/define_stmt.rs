use crate::schema_set::{Diff, Len, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::DefineStmt;
use postgres_parser::sys::ObjectType;
use postgres_parser::Node;
use std::borrow::Cow;
use std::collections::HashMap;

impl Sql for DefineStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE ");
        if self.replace {
            sql.push_str("OR REPLACE ");
        }
        sql.push_str(&self.kind.sql());
        sql.push(' ');
        sql.push_str(&self.defnames.sql("."));

        match self.kind {
            ObjectType::OBJECT_COLLATION => {
                if self.definition.len() == 1 {
                    if let Node::DefElem(defelem) =
                        self.definition.as_ref().unwrap().get(0).unwrap()
                    {
                        if defelem.defname == Some("from".into()) {
                            sql.push(' ');
                            sql.push_str(&defelem.sql())
                        } else {
                            sql.push('(');
                            sql.push_str(&self.definition.sql_prefix(" ", ", "));
                            sql.push(')');
                        }
                    } else {
                        panic!(
                            "unexpected DefineStmt::definition node: {:#?}",
                            self.definition
                        );
                    }
                } else {
                    sql.push_str(&self.definition.sql_prefix_and_wrap(" ", "(", ")", ", "));
                }
            }

            ObjectType::OBJECT_OPERATOR | ObjectType::OBJECT_AGGREGATE => {
                if self.args.is_some() {
                    let args = self.args.as_ref().unwrap().get(0).unwrap();
                    if let Node::List(args) = args {
                        let marker = self.args.as_ref().unwrap().last().unwrap();

                        match marker {
                            Node::Value(value) => {
                                let ndirectargs = value.int.unwrap();
                                sql.push('(');

                                let mut iter = args.iter();
                                let mut i = 0;
                                while let Some(arg) = iter.next() {
                                    if i > 0 {
                                        sql.push_str(", ");
                                    }

                                    i += 1;
                                    if ndirectargs > -1 && i >= ndirectargs.max(0) as usize {
                                        let orderby = iter.next();

                                        if orderby.is_some() {
                                            sql.push_str(&arg.sql());
                                            sql.push_str(" ORDER BY ");
                                            sql.push_str(&orderby.unwrap().sql());
                                        } else {
                                            sql.push_str(&arg.sql());
                                            sql.push_str(" ORDER BY ");
                                            sql.push_str(&arg.sql());
                                        }
                                    } else {
                                        sql.push_str(&arg.sql());
                                    }
                                }

                                sql.push(')');
                            }

                            _ => sql.push_str(&self.args.sql_wrap(", ", "(", ")")),
                        }
                    } else {
                        sql.push('(');
                        for arg in self.args.as_ref().unwrap() {
                            let arg = arg.sql();
                            if arg == "-1" {
                                sql.push('*');
                            } else {
                                sql.push_str(&arg);
                            }
                        }
                        sql.push(')');
                    }
                }

                sql.push('(');
                for (i, node) in self.definition.as_ref().unwrap().iter().enumerate() {
                    if let Node::DefElem(defelem) = node {
                        if i > 0 {
                            sql.push_str(", ");
                        }
                        if self.oldstyle {
                            sql.push('"');
                            sql.push_str(defelem.defname.as_ref().unwrap());
                            sql.push('"');
                        } else {
                            sql.push_str(defelem.defname.as_ref().unwrap());
                        }
                        if defelem.arg.is_some() {
                            sql.push('=');

                            let defname = &defelem.defname.as_ref().unwrap().to_lowercase();
                            if "initcond" == defname || "initcond1" == defname {
                                sql.push('\'');
                                sql.push_str(&defelem.arg.sql().replace("'", "''"));
                                sql.push('\'');
                            } else if Some("basetype".into()) == defelem.defname {
                                let arg = defelem.arg.sql();

                                if arg == "ANY" {
                                    sql.push('\'');
                                    sql.push_str(&defelem.arg.sql().replace("'", "''"));
                                    sql.push('\'');
                                } else {
                                    sql.push_str(&defelem.arg.sql());
                                }
                            } else {
                                sql.push_str(&defelem.arg.sql());
                            }
                        }
                    } else {
                        panic!("unexpected definition node for DefineStatement::ObjectType::OBJECT_OPERATOR");
                    }
                }
                sql.push(')');
            }
            _ => {
                if self.args.is_some() {
                    if self.kind != ObjectType::OBJECT_AGGREGATE {
                        sql.push_str(" AS ");
                    }

                    sql.push('(');
                    sql.push_str(&self.args.sql(", "));
                    sql.push(')');
                }
            }
        }

        sql
    }
}

impl Diff for DefineStmt {
    fn alter_stmt(&self, other: &Node) -> Option<String> {
        if matches!(self.kind, ObjectType::OBJECT_OPERATOR) {
            let my_defelems = self
                .definition
                .as_ref()
                .unwrap()
                .into_iter()
                .filter_map(|node| {
                    if let Node::DefElem(defelem) = node {
                        let name = defelem.defname.as_ref().unwrap();
                        Some((name.as_str(), defelem))
                    } else {
                        None
                    }
                })
                .collect::<HashMap<_, _>>();
            if let Node::DefineStmt(other) = other {
                let other_defelms = other
                    .definition
                    .as_ref()
                    .unwrap()
                    .into_iter()
                    .filter_map(|node| {
                        if let Node::DefElem(defelem) = node {
                            let name = defelem.defname.as_ref().unwrap();
                            Some((name.as_str(), defelem))
                        } else {
                            None
                        }
                    })
                    .collect::<HashMap<_, _>>();

                let my_lefttype = my_defelems
                    .get("leftarg")
                    .expect("OPERATOR should have a LEFTARG");
                let my_righttype = my_defelems
                    .get("rightarg")
                    .expect("OPERATOR should have a RIGHTARG");

                let args = format!("({}, {})", my_lefttype.sql(), my_righttype.sql());

                let mut sql = String::new();

                match (my_defelems.get("restrict"), other_defelms.get("restrict")) {
                    (Some(mine), Some(theirs)) if mine != theirs => {
                        sql.push_str(&format!(
                            "ALTER OPERATOR {}{args} SET (RESTRICT = {});\n",
                            self.defnames.sql_ident(),
                            theirs.sql()
                        ));
                    }
                    (Some(_), None) => {
                        sql.push_str(&format!(
                            "ALTER OPERATOR {}{args} SET (RESTRICT = NONE);\n",
                            self.defnames.sql_ident()
                        ));
                    }
                    _ => {}
                }

                match (my_defelems.get("join"), other_defelms.get("join")) {
                    (Some(mine), Some(theirs)) if mine != theirs => {
                        sql.push_str(&format!(
                            "ALTER OPERATOR {}{args} SET (JOIN = {});\n",
                            self.defnames.sql_ident(),
                            theirs.sql()
                        ));
                    }
                    (Some(_), None) => {
                        sql.push_str(&format!(
                            "ALTER OPERATOR {}{args} SET (JOIN = NONE);\n",
                            self.defnames.sql_ident()
                        ));
                    }
                    _ => {}
                }

                let sql = sql.trim();
                return if sql.is_empty() {
                    None
                } else {
                    Some(sql.trim_end_matches(';').to_string())
                };
            }
            Diff::alter_stmt(self, other)
        } else {
            Diff::alter_stmt(self, other)
        }
    }

    fn drop_stmt(&self) -> Option<String> {
        Some(format!(
            "DROP {} IF EXISTS {}",
            self.kind.sql(),
            self.defnames.sql_ident()
        ))
    }

    fn object_name(&self) -> Option<String> {
        Some(self.defnames.sql_ident())
    }

    fn object_type(&self) -> String {
        self.kind.sql()
    }

    fn identifier<'a>(&self, tree_string: &'a str) -> Cow<'a, str> {
        match self.kind {
            ObjectType::OBJECT_TYPE => {
                let mut sql = self.sql();
                if self.definition.is_none() || self.definition.as_ref().unwrap().is_empty() {
                    sql += " (shell type)";
                }
                Cow::Owned(sql)
            }
            ObjectType::OBJECT_OPERATOR => Cow::Owned(self.sql()),
            _ => match self.object_name() {
                Some(name) => Cow::Owned(name + &self.object_type()),
                None => Cow::Borrowed(tree_string),
            },
        }
    }
}
