use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::LockingClause;
use postgres_parser::sys::{LockClauseStrength, LockWaitPolicy};

impl Sql for LockingClause {
    fn sql(&self) -> String {
        let mut sql = String::new();

        match self.strength {
            LockClauseStrength::LCS_NONE => {}
            LockClauseStrength::LCS_FORKEYSHARE => sql.push_str(" FOR KEY SHARE"),
            LockClauseStrength::LCS_FORSHARE => sql.push_str(" FOR SHARE"),
            LockClauseStrength::LCS_FORNOKEYUPDATE => sql.push_str(" FOR NO KEY UPDATE"),
            LockClauseStrength::LCS_FORUPDATE => sql.push_str(" FOR UPDATE"),
        }

        sql.push_str(&self.lockedRels.sql_prefix(" OF ", ", "));

        match self.waitPolicy {
            LockWaitPolicy::LockWaitBlock => {}
            LockWaitPolicy::LockWaitSkip => sql.push_str(" SKIP LOCKED"),
            LockWaitPolicy::LockWaitError => sql.push_str(" NOWAIT"),
        }

        sql
    }
}
