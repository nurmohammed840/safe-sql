use sql_parser::grammar::{ast::*, Term};

pub trait ExprAnalyser {
    fn analyse_or_expr(&mut self, expr: &OrExpr) {
        self.analyse_and_expr(&expr.left);
        if let Some((_, expr)) = &expr.right {
            let _ = self.analyse_or_expr(expr);
        }
    }
    fn analyse_and_expr(&mut self, expr: &AndExpr) {
        self.analyse_condition(&expr.left);
        if let Some((_, expr)) = &expr.right {
            let _ = self.analyse_and_expr(expr);
        }
    }

    fn analyse_condition(&mut self, cond: &Condition) {
        match cond {
            Condition::Not(cond) => self.analyse_condition(cond),
            Condition::Operand { left, right } => {
                self.analyse_operand(left);
                if let Some(_cond) = right {}
            }
        }
    }

    fn analyse_operand(&mut self, expr: &Operand) {
        self.analyse_arithmetic(&expr.left);
        if let Some((_, expr)) = &expr.right {
            let _ = self.analyse_operand(expr);
        }
    }
    fn analyse_arithmetic(&mut self, expr: &Arithmetic) {
        self.analyse_factorial(&expr.left);
        if let Some((_, expr)) = &expr.right {
            let _ = self.analyse_arithmetic(expr);
        }
    }

    fn analyse_factorial(&mut self, expr: &Factorial) {
        self.analyse_term(&expr.left);
        if let Some((_, expr)) = &expr.right {
            let _ = self.analyse_factorial(expr);
        }
    }

    fn analyse_term(&mut self, term: &Term);
}
