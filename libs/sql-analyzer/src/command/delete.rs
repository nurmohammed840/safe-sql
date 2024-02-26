use super::*;
use expr_analyzer::ExprAnalyser;
use sql_parser::{command::delete::*, function::Function, grammar::Term};

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
            Term::Func(_func) => match _func.as_ref() {
                Function::ABS(_) => {},
                Function::ACOS(_) => {},
                Function::ASIN(_) => {},
                Function::ATAN(_) => {},
                Function::COS(_) => {},
                Function::COSH(_) => {},
                Function::COT(_) => {},
                Function::SIN(_) => {},
                Function::SINH(_) => {},
                Function::TAN(_) => {},
                Function::TANH(_) => {},
                Function::ATAN2(_, _) => {},
                Function::BITAND(_, _) => {},
                Function::BITOR(_, _) => {},
                Function::BITXOR(_, _) => {},
                Function::BITNOT(_) => {},
                Function::BITNAND(_, _) => {},
                Function::BITNOR(_, _) => {},
                Function::BITXNOR(_, _) => {},
                Function::BITGET(_, _) => {},
                Function::BITCOUNT(_) => {},
                Function::LSHIFT(_, _) => {},
                Function::RSHIFT(_, _) => {},
                Function::ULSHIFT(_, _) => {},
                Function::URSHIFT(_, _) => {},
                Function::ROTATELEFT(_, _) => {},
                Function::ROTATERIGHT(_, _) => {},
                Function::MOD(_, _) => {},
                Function::CEIL(_) => {},
                Function::DEGREES(_) => {},
                Function::EXP(_) => {},
                Function::FLOOR(_) => {},
                Function::LN(_) => {},
                Function::LOG(_, _) => {},
                Function::LOG10(_) => {},
                Function::ORA_HASH(_) => {},
                Function::RADIANS(_) => {},
                Function::SQRT(_) => {},
                Function::PI() => {},
                Function::POWER(_, _) => {},
                Function::RAND(_) => {},
                Function::RANDOM_UUID() => {},
                Function::ROUND(_) => {},
                Function::SECURE_RAND(_) => {},
                Function::SIGN(_) => {},
                // # String Function
                Function::ASCII(_) => {},
                Function::CHAR_LENGTH(_) => {},
                Function::CHAR(_) => {},
                Function::CONCAT(_) => {},
                Function::DIFFERENCE(_, _) => {},
                Function::HEXTORAW(_) => {},
                Function::LOWER(_) => {},
                Function::UPPER(_) => {},
                Function::LEFT(_, _) => {},
                Function::RIGHT(_, _) => {},
                Function::REPEAT(_, _) => {},
                Function::SOUNDEX(_) => {},
                Function::SPACE(_) => {},
            },
            Term::OrExpr(expr) => self.analyse_or_expr(expr),
        }
    }
}
