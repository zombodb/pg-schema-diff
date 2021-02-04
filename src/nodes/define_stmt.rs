use crate::schema_set::{Diff, Len, Sql, SqlIdent, SqlList, SqlMaybeList};
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
            ObjectType::OBJECT_AGGREGATE if !self.oldstyle => {
                sql.push('(');
                let args = self.args.as_ref().unwrap().get(0).cloned().unwrap();
                sql.push_str(&args.sql_maybe_list(", "));
                sql.push(')');
            }
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
            _ => {
                if self.kind != ObjectType::OBJECT_AGGREGATE {
                    sql.push_str(" AS ");
                }

                sql.push('(');
                sql.push_str(&self.args.sql(", "));
                sql.push(')');
            }
        }

        sql
    }
}

impl Diff for DefineStmt {
    fn alter(&self, _other: &Node) -> Option<String> {
        unimplemented!()
    }

    fn drop(&self) -> String {
        unimplemented!()
    }

    fn name(&self, _sql: &str) -> String {
        self.defnames.sql_ident()
    }
}
