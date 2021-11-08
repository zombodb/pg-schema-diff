use crate::make_name;
use crate::schema_set::{Diff, Sql, SqlIdent, SqlList};
use postgres_parser::nodes::CreateDomainStmt;

impl Sql for CreateDomainStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CREATE DOMAIN ");
        sql.push_str(&self.domainname.sql_ident());
        sql.push_str(" AS ");
        sql.push_str(&self.typeName.sql());

        sql.push_str(&self.collClause.sql());
        sql.push_str(&self.constraints.sql_prefix(" ", " "));

        sql
    }
}

impl Diff for CreateDomainStmt {
    fn object_name(&self) -> Option<String> {
        Some(make_name(&self.domainname).expect("unable to make CreateDomainStmt::domainname"))
    }

    fn object_type(&self) -> String {
        "DOMAIN".into()
    }

}
