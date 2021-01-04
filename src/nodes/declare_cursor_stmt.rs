use crate::schema_set::{Diff, Sql};
use postgres_parser::nodes::DeclareCursorStmt;

impl Sql for DeclareCursorStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("DECLARE ");
        sql.push_str(&self.portalname.as_ref().unwrap());

        if self.options & 0x0001 != 0 {
            sql.push_str(" BINARY");
        }

        if self.options & 0x0008 != 0 {
            sql.push_str(" INSENSITIVE");
        }

        if self.options & 0x0002 != 0 {
            sql.push_str(" SCROLL");
        }

        if self.options & 0x0004 != 0 {
            sql.push_str(" NO SCROLL");
        }

        sql.push_str(" CURSOR");

        if self.options & 0x0010 != 0 {
            sql.push_str(" WITH HOLD");
        } else {
            sql.push_str(" WITHOUT HOLD");
        }

        sql.push_str(" FOR ");
        sql.push_str(&self.query.sql());

        sql
    }
}

impl Diff for DeclareCursorStmt {}
