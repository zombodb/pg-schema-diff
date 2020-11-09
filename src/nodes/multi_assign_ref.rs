use crate::schema_set::{Len, Sql};
use postgres_parser::nodes::MultiAssignRef;
use postgres_parser::Node;

impl Sql for MultiAssignRef {
    fn sql(&self) -> String {
        let mut sql = String::new();

        match self.source.as_ref() {
            None => {}
            Some(source) => match source.as_ref() {
                Node::RowExpr(row_expr) if row_expr.args.len() > 0 => {
                    sql.push_str(&row_expr.args.as_ref().unwrap()[self.colno as usize - 1].sql())
                }
                Node::SubLink(sub_link) => sql.push_str(&sub_link.sql()),
                _ => panic!(
                    "unexpected 'source' node in MultiAssignRef: {:?}",
                    self.source
                ),
            },
        }
        if let Some(_vec) = self.source.as_ref() {
            // sql.push_str(&vec[self.colno as usize].sql());
        }

        sql
    }
}
