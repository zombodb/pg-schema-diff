#![allow(dead_code)]
use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::CreateTrigStmt;

const TRIGGER_TYPE_AFTER: i16 = 0;
const TRIGGER_TYPE_ROW: i16 = 1 << 0;
const TRIGGER_TYPE_BEFORE: i16 = 1 << 1;
const TRIGGER_TYPE_INSERT: i16 = 1 << 2;
const TRIGGER_TYPE_DELETE: i16 = 1 << 3;
const TRIGGER_TYPE_UPDATE: i16 = 1 << 4;
const TRIGGER_TYPE_TRUNCATE: i16 = 1 << 5;
const TRIGGER_TYPE_INSTEAD: i16 = 1 << 6;

impl Sql for CreateTrigStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE ");
        if self.isconstraint {
            sql.push_str("CONSTRAINT ");
        }
        sql.push_str("TRIGGER ");
        sql.push_str(&self.trigname.sql_ident());

        if self.timing & TRIGGER_TYPE_BEFORE == TRIGGER_TYPE_BEFORE {
            sql.push_str(" BEFORE");
        } else if self.timing & TRIGGER_TYPE_INSTEAD == TRIGGER_TYPE_INSTEAD {
            sql.push_str(" INSTEAD OF");
        } else if self.timing & TRIGGER_TYPE_INSTEAD == TRIGGER_TYPE_AFTER {
            sql.push_str(" AFTER");
        }

        let mut have_type = false;
        if self.events & TRIGGER_TYPE_INSERT == TRIGGER_TYPE_INSERT {
            sql.push_str(" INSERT ");
            have_type = true;
        }

        if self.events & TRIGGER_TYPE_UPDATE == TRIGGER_TYPE_UPDATE {
            if have_type {
                sql.push_str("OR")
            }
            sql.push_str(" UPDATE ");
            have_type = true;
        }

        if self.events & TRIGGER_TYPE_DELETE == TRIGGER_TYPE_DELETE {
            if have_type {
                sql.push_str("OR")
            }
            sql.push_str(" DELETE ");
            have_type = true;
        }

        if self.events & TRIGGER_TYPE_TRUNCATE == TRIGGER_TYPE_TRUNCATE {
            if have_type {
                sql.push_str("OR")
            }
            sql.push_str(" TRUNCATE ");
        }

        sql.push_str("ON ");
        sql.push_str(&self.relation.sql());
        sql.push(' ');

        if self.isconstraint {
            if self.deferrable {
                sql.push_str("DEFERRABLE ");
            } else {
                sql.push_str("NOT DEFERRABLE ");
            }

            if self.initdeferred {
                sql.push_str("INITIALLY DEFERRED ");
            } else {
                sql.push_str("INITIALLY IMMEDIATE ");
            }
        }

        sql.push_str("FOR EACH ");
        if self.row {
            sql.push_str("ROW ");
        } else {
            sql.push_str("STATEMENT ");
        }

        sql.push_str("EXECUTE PROCEDURE ");
        sql.push_str(&self.funcname.sql_ident());
        if self.args.is_some() {
            sql.push_str(&self.args.sql_wrap(", ", "(", ")"));
        } else {
            sql.push_str("()");
        }

        sql
    }
}

impl Diff for CreateTrigStmt {}
