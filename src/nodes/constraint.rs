use crate::schema_set::{Sql, SqlIdent, SqlList};
use crate::{make_individual_names, EMPTY_NODE_VEC};
use postgres_parser::nodes::Constraint;
use postgres_parser::sys::ConstrType;
use postgres_parser::Node;

impl Sql for Constraint {
    fn sql(&self) -> String {
        eprintln!("{:#?}", self);
        let mut sql = String::new();

        match self.contype {
            ConstrType::CONSTR_NOTNULL => sql.push_str("NOT NULL"),
            ConstrType::CONSTR_NULL => sql.push_str("NULL"),
            ConstrType::CONSTR_PRIMARY => sql.push_str("PRIMARY KEY"),
            ConstrType::CONSTR_DEFAULT => {
                sql.push_str("DEFAULT ");
                sql.push_str(
                    &self
                        .raw_expr
                        .as_ref()
                        .expect("no raw_expr for Constraint")
                        .sql(),
                );
            }

            ConstrType::CONSTR_IDENTITY => {
                if self.generated_when == 'a' {
                    sql.push_str("GENERATED ALWAYS ");
                } else {
                    sql.push_str("AS DEFAULT ");
                }

                sql.push_str("AS IDENTITY ");
                sql.push('(');
                sql.push_str(&self.options.sql_ident());
                sql.push(')');
            }
            ConstrType::CONSTR_GENERATED => {
                sql.push_str("GENERATED ALWAYS AS ");
                sql.push_str(&self.raw_expr.sql_wrap("(", ")"));
                sql.push_str(" STORED");
            }
            ConstrType::CONSTR_UNIQUE => {
                sql.push_str("UNIQUE ");
                sql.push('(');
                sql.push_str(
                    &make_individual_names(&self.keys)
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                sql.push(')');
                sql.push_str(
                    &self
                        .including
                        .sql_prefix_and_wrap(" INCLUDE ", "(", ")", ", "),
                );
                if self.options.is_some() {
                    sql.push_str(" WITH (");
                    for opt in self.options.as_ref().unwrap_or(&EMPTY_NODE_VEC) {
                        if let Node::DefElem(def_elem) = opt {
                            sql.push_str(&def_elem.defname.sql_ident());
                            sql.push_str(&def_elem.arg.sql_prefix("="));
                        } else {
                            panic!("unexpected 'options' element in Constraint::CONSTR_UNIQUE")
                        }
                    }
                    sql.push(')');
                }
                sql.push_str(&self.indexspace.sql_ident_prefix(" USING INDEX TABLESPACE "))
            }
            ConstrType::CONSTR_CHECK => {
                sql.push_str(&self.raw_expr.sql_prefix_and_wrap("CHECK ", "(", ")"));
                if self.is_no_inherit {
                    sql.push_str(" NO INHERIT");
                }
            }
            ConstrType::CONSTR_FOREIGN => {
                sql.push_str("FOREIGN KEY ");
                sql.push('(');
                sql.push_str(
                    &make_individual_names(&self.fk_attrs)
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                sql.push(')');

                sql.push_str(" REFERENCES ");
                sql.push('(');
                sql.push_str(
                    &make_individual_names(&self.pk_attrs)
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                sql.push(')');

                match self.fk_matchtype {
                    'f' => sql.push_str(" MATCH FULL"),
                    'p' => sql.push_str(" MATCH PARTIAL"),
                    's' => sql.push_str(" MATCH SIMPLE"),
                    _ => {}
                }

                /*
                   #define FKCONSTR_ACTION_NOACTION	'a'
                   #define FKCONSTR_ACTION_RESTRICT	'r'
                   #define FKCONSTR_ACTION_CASCADE		'c'
                   #define FKCONSTR_ACTION_SETNULL		'n'
                   #define FKCONSTR_ACTION_SETDEFAULT	'd'
                */
                match self.fk_del_action {
                    'a' => sql.push_str(" ON DELETE NO ACTION"),
                    'r' => sql.push_str(" ON DELETE RESTRICT"),
                    'c' => sql.push_str(" ON DELETE CASCADE"),
                    'n' => sql.push_str(" ON DELETE SET NULL"),
                    'd' => sql.push_str(" ON DELETE SET DEFAULT"),
                    _ => {}
                }
                match self.fk_upd_action {
                    'a' => sql.push_str(" ON UPDATE NO ACTION"),
                    'r' => sql.push_str(" ON UPDATE RESTRICT"),
                    'c' => sql.push_str(" ON UPDATE CASCADE"),
                    'n' => sql.push_str(" ON UPDATE SET NULL"),
                    'd' => sql.push_str(" ON UPDATE SET DEFAULT"),
                    _ => {}
                }

                if self.deferrable {
                    sql.push_str(" DEFERRABLE");
                } else {
                    sql.push_str(" NOT DEFERRABLE");
                }

                if self.initdeferred {
                    sql.push_str(" INITIALLY DEFERRED");
                } else {
                    sql.push_str(" INITIALLY IMMEDIATE");
                }
            }
            _ => unimplemented!("{:?}", self.contype),
        }

        sql
    }
}
