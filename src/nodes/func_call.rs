use crate::make_name;
use crate::schema_set::{Sql, SqlList};
use postgres_parser::nodes::FuncCall;

impl Sql for FuncCall {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&make_name(&self.funcname).expect("no name for FuncCall"));
        sql.push('(');
        if self.agg_star {
            sql.push('*');
        } else if self.agg_distinct {
            sql.push_str("DISTINCT ");
        }
        if self.func_variadic {
            sql.push_str("VARIADIC ");
        }
        sql.push_str(&self.args.sql(", "));
        if self.agg_within_group {
            sql.push(')');
            sql.push_str(" WITHIN GROUP (")
        }
        sql.push_str(&self.agg_order.sql_prefix(" ORDER BY ", ", "));
        sql.push(')');
        sql.push_str(
            &self
                .agg_filter
                .sql_prefix_and_wrap(" FILTER", "(WHERE ", ")"),
        );
        sql
    }
}
