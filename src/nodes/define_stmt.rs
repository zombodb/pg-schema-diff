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

        if self.args.len() > 0 {
            if self.kind != ObjectType::OBJECT_AGGREGATE {
                sql.push_str(" AS ");
            }

            match self.kind {
                ObjectType::OBJECT_AGGREGATE if !self.oldstyle => {
                    sql.push('(');
                    let args = self.args.as_ref().unwrap().get(0).cloned().unwrap();
                    sql.push_str(&args.sql_maybe_list(", "));
                    sql.push(')');
                }
                _ => {
                    sql.push('(');
                    sql.push_str(&self.args.sql(", "));
                    sql.push(')');
                }
            }
        }

        if self.definition.len() > 0 {
            sql.push('(');
            sql.push_str(&self.definition.sql(", "));
            sql.push(')');
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
