use crate::schema_set::{Diff, Sql, SqlIdent};
use postgres_parser::nodes::CreateAmStmt;
use postgres_parser::Node;

impl Sql for CreateAmStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE ACCESS METHOD ");
        sql.push_str(&self.amname.sql_ident());
        sql.push_str(" TYPE ");
        match self.amtype {
            't' => sql.push_str("TABLE"),
            'i' => sql.push_str("INDEX"),
            _ => panic!("unexpected CreateAmStmt::amtype"),
        }
        sql.push_str(" HANDLER ");
        sql.push_str(&self.handler_name.sql_ident());

        sql
    }
}

impl Diff for CreateAmStmt {
    fn alter(&self, _other: &Node) -> Option<String> {
        unimplemented!()
    }

    fn drop(&self) -> String {
        unimplemented!()
    }

    fn object_name(&self) -> Option<String> {
        Some(self.amname.sql_ident())
    }
}
