use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::CreateOpClassItem;

impl Sql for CreateOpClassItem {
    fn sql(&self) -> String {
        let mut sql = String::new();

        match self.itemtype {
            1 => {
                sql.push_str("OPERATOR ");
                sql.push_str(&self.number.to_string());
                sql.push(' ');
                sql.push_str(&self.name.sql());
                sql.push(' ');
                sql.push_str(&self.order_family.sql(" "))
            }
            2 => {
                sql.push_str("FUNCTION ");
                sql.push_str(&self.number.to_string());
                sql.push(' ');
                sql.push_str(&self.name.sql());
            }
            3 => {
                sql.push_str("STORAGE ");
                sql.push_str(&self.storedtype.sql());
            }
            _ => panic!("unknoown CreateOptClassItem::itemtype: {}", self.itemtype),
        }

        sql
    }
}
