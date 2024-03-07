use super::*;
use analyzer::{AnalyseExpr, Analyser};
use sql_parser::command::delete::*;

impl SqlAnalyzer for Delete {
    fn analyse(&self, ctx: &mut Ctx) -> AnalyseResult {
        if let Some(table) = ctx.get_table(&self.table_name.alias) {
            if let Some(expr) = &self.where_expr {
                let result = AnalyseExpr {
                    _schema: ctx.info,
                    table: Some(table),
                }
                .analyse_or_expr(expr);
                ctx.add_err(result);
            }
        }
        Ok(())
    }
}
