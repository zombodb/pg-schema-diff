use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::LockStmt;

impl Sql for LockStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("LOCK TABLE ");
        if self.relations.is_none() {
            sql.push('*');
        } else {
            sql.push_str(&self.relations.sql(", "));
        }

        sql.push_str(" IN ");

        /*
        /* NoLock is not a lock mode, but a flag value meaning "don't get a lock" */
        #define NoLock					0

        #define AccessShareLock			1	/* SELECT */
        #define RowShareLock			2	/* SELECT FOR UPDATE/FOR SHARE */
        #define RowExclusiveLock		3	/* INSERT, UPDATE, DELETE */
        #define ShareUpdateExclusiveLock 4	/* VACUUM (non-FULL),ANALYZE, CREATE INDEX
                                             * CONCURRENTLY */
        #define ShareLock				5	/* CREATE INDEX (WITHOUT CONCURRENTLY) */
        #define ShareRowExclusiveLock	6	/* like EXCLUSIVE MODE, but allows ROW
                                             * SHARE */
        #define ExclusiveLock			7	/* blocks ROW SHARE/SELECT...FOR UPDATE */
        #define AccessExclusiveLock		8	/* ALTER TABLE, DROP TABLE, VACUUM FULL,
                                             * and unqualified LOCK TABLE */

                 */
        match self.mode {
            1 => sql.push_str("ACCESS SHARE"),
            2 => sql.push_str("ROW SHARE"),
            3 => sql.push_str("ROW EXCLUSIVE"),
            4 => sql.push_str("SHARE UPDATE EXCLUSIVE"),
            5 => sql.push_str("SHARE"),
            6 => sql.push_str("SHARE ROW EXCLUSIVE"),
            7 => sql.push_str("EXCLUSIVE"),
            8 => sql.push_str("ACCESS EXCLUSIVE"),
            _ => panic!("unrecognized lock mode"),
        }

        sql.push_str(" MODE");
        if self.nowait {
            sql.push_str(" NOWAIT");
        }

        sql
    }
}

impl Diff for LockStmt {}
