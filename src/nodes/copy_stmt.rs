use crate::schema_set::{Diff, Len, Sql, SqlList};
use postgres_parser::nodes::CopyStmt;

impl Sql for CopyStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("COPY ");

        if self.query.is_some() {
            sql.push_str(&self.query.sql_wrap("(", ")"))
        } else {
            sql.push_str(&self.relation.sql());
            if !self.attlist.is_empty() {
                sql.push_str(&self.attlist.sql_wrap(", ", "(", ")"))
            }
        }

        if self.is_from {
            sql.push_str(" FROM ");
            if self.filename.is_some() {
                if self.is_program {
                    sql.push_str("PROGRAM ");
                }
                sql.push_str(&format!("'{}'", self.filename.as_ref().unwrap()));
            } else {
                sql.push_str("STDIN");
            }
        } else {
            sql.push_str(" TO ");
            if self.filename.is_some() {
                if self.is_program {
                    sql.push_str("PROGRAM ");
                }
                sql.push_str(&format!("'{}'", self.filename.as_ref().unwrap()));
            } else {
                sql.push_str("STDOUT");
            }
        }

        if self.options.is_some() {
            sql.push_str(" WITH (");
            sql.push_str(&self.options.sql(", "));
            sql.push(')');
        }

        if self.whereClause.is_some() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.whereClause.sql());
        }

        sql
    }
}

impl Diff for CopyStmt {}
