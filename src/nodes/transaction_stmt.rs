use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::TransactionStmt;
use postgres_parser::sys::TransactionStmtKind;

impl Sql for TransactionStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        match self.kind {
            TransactionStmtKind::TRANS_STMT_BEGIN | TransactionStmtKind::TRANS_STMT_START => {
                sql.push_str("BEGIN TRANSACTION ");
                sql.push_str(&self.options.sql(" "));
            }
            TransactionStmtKind::TRANS_STMT_COMMIT => {
                sql.push_str("COMMIT TRANSACTION");
                if self.chain {
                    sql.push_str(" AND CHAIN")
                }
            }
            TransactionStmtKind::TRANS_STMT_ROLLBACK => {
                sql.push_str("ROLLBACK TRANSACTION");
                if self.chain {
                    sql.push_str(" AND CHAIN")
                }
            }
            TransactionStmtKind::TRANS_STMT_SAVEPOINT => {
                sql.push_str("SAVEPOINT ");
                sql.push_str(&self.savepoint_name.sql_ident());
            }
            TransactionStmtKind::TRANS_STMT_RELEASE => {
                sql.push_str("RELEASE SAVEPOINT ");
                sql.push_str(&self.savepoint_name.sql_ident());
            }
            TransactionStmtKind::TRANS_STMT_ROLLBACK_TO => {
                sql.push_str("ROLLBACK TRANSACTION TO ");
                sql.push_str(&self.savepoint_name.sql_ident());
            }
            TransactionStmtKind::TRANS_STMT_PREPARE => {
                sql.push_str("PREPARE TRANSACTION ");
                sql.push_str(&self.gid.sql_ident());
            }
            TransactionStmtKind::TRANS_STMT_COMMIT_PREPARED => {
                sql.push_str("COMMIT PREPARED ");
                sql.push_str(&self.gid.sql_ident());
            }
            TransactionStmtKind::TRANS_STMT_ROLLBACK_PREPARED => {
                sql.push_str("ROLLBACK PREPARED ");
                sql.push_str(&self.gid.sql_ident());
            }
        }

        sql
    }
}

impl Diff for TransactionStmt {}
