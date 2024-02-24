use super::*;
use expr_analyzer::ExprAnalyser;
use sql_parser::{command::delete::*, grammar::Term};

impl SqlAnalyzer for Delete {
    fn analyse(&self, ctx: &mut Ctx) -> AnalyseResult {
        if let Some(table) = ctx.get_table(&self.table_name.table_name) {
            // table;
            if let Some(expr) = &self.where_expr {
                AnalyseDeleteExpr { ctx, table }.analyse_or_expr(expr);
            }
        }
        Ok(())
    }
}

struct AnalyseDeleteExpr<'c, 's> {
    ctx: &'c mut Ctx<'s>,
    table: &'s Table,
}

impl ExprAnalyser for AnalyseDeleteExpr<'_, '_> {
    fn analyse_term(&mut self, term: &Term) {
        match term {
            Term::Value(_) => {}
            Term::Column(name) => {
                self.ctx.get_column(self.table, &name.alias);
            }
            Term::Func(_) => {},
            Term::OrExpr(expr) => self.analyse_or_expr(expr),
            
        }
    }
}
