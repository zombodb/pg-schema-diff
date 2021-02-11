use crate::schema_set::{Diff, Len, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::DefineStmt;
use postgres_parser::sys::ObjectType;
use postgres_parser::Node;

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
    fn object_name(&self) -> Option<String> {
        Some(self.defnames.sql_ident())
    }
}
