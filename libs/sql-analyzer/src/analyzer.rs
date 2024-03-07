use crate::schema_info::SchemaInfo;
use crate::{err, utils, AnalyseError, DataType, Table};
use sql_parser::grammar::{ast::*, Term};
use sql_parser::{function::FunctionKind, grammar::Value};

pub trait Analyser {
    fn analyse_or_expr(&mut self, expr: &OrExpr) -> Result<DataType, AnalyseError> {
        let left = self.analyse_and_expr(&expr.left)?;
        if let Some((_, expr_right)) = &expr.right {
            err::expect_bool(&left, &expr.left)?;
            let right = &self.analyse_or_expr(expr_right)?;
            err::expect_bool(&right, &expr_right)?;
        }
        Ok(left)
    }

    fn analyse_and_expr(&mut self, expr: &AndExpr) -> Result<DataType, AnalyseError> {
        let left = self.analyse_condition(&expr.left)?;
        if let Some((_, expr_right)) = &expr.right {
            err::expect_bool(&left, &expr.left)?;
            let right = self.analyse_and_expr(expr_right)?;
            err::expect_bool(&right, &expr_right)?;
        }
        Ok(left)
    }

    fn analyse_condition(&mut self, cond: &Condition) -> Result<DataType, AnalyseError> {
        match cond {
            Condition::Not(cond) => {
                let ty = self.analyse_condition(cond)?;
                err::expect_bool(&ty, cond)?;
                Ok(ty)
            }
            Condition::Operand {
                left: expr_left,
                right,
            } => {
                let left = self.analyse_operand(expr_left)?;
                if let Some(rhs) = right {
                    match rhs {
                        RightHandSide::Comparison(_, expr_right) => {
                            err::expect_numeric(&left, expr_left)?;
                            let right = self.analyse_operand(expr_right)?;
                            if left.is_unknown() && right.is_unknown() {
                                return err::msg(expr_left, "unknown type");
                            }
                            err::expect_numeric(&right, expr_right)?;
                        }
                    }
                }
                Ok(left)
            }
        }
    }

    fn analyse_operand(&mut self, expr: &Operand) -> Result<DataType, AnalyseError> {
        let left = self.analyse_arithmetic(&expr.left)?;
        if let Some((_, expr_right)) = &expr.right {
            let right = self.analyse_operand(expr_right)?;
            if left.is_text() || right.is_text() {
                return Ok(DataType::Text);
            }
            if let (DataType::Array { ty }, DataType::Array { ty: _ty }) = (&left, &right) {
                if ty.is_numeric() && _ty.is_numeric() {
                    return Ok(DataType::Numeric);
                }
                // TODO
            }
            return err::msg(
                &expr.left,
                format!("mismatch type, left: `{left:?}`, right: `{right:?}`"),
            );
        }
        Ok(left)
    }

    fn analyse_arithmetic(&mut self, expr: &Arithmetic) -> Result<DataType, AnalyseError> {
        let left = self.analyse_factorial(&expr.left)?;
        if let Some((symbol, expr_right)) = &expr.right {
            err::expect_numeric(&left, &expr.left)?;

            let right = self.analyse_arithmetic(expr)?;
            if left.is_unknown() && right.is_unknown() {
                return err::msg(symbol, "unknown type");
            }
            err::expect_numeric(&right, expr_right)?;
        }
        Ok(left)
    }

    fn analyse_factorial(&mut self, expr: &Factorial) -> Result<DataType, AnalyseError> {
        let left = self.analyse_term(&expr.left)?;
        if let Some((symbol, expr_right)) = &expr.right {
            err::expect_numeric(&left, &expr.left)?;

            let right = self.analyse_factorial(expr_right)?;
            if left.is_unknown() && right.is_unknown() {
                return err::msg(symbol, "unknown type");
            }
            err::expect_numeric(&right, expr_right)?;
        }
        Ok(left)
    }

    fn analyse_term(&mut self, term: &Term) -> Result<DataType, AnalyseError>;
}

pub struct AnalyseExpr<'t> {
    pub _schema: &'t SchemaInfo,
    pub table: Option<&'t Table>,
}

impl Analyser for AnalyseExpr<'_> {
    fn analyse_term(&mut self, term: &Term) -> Result<DataType, AnalyseError> {
        Ok(match term {
            Term::Value(val) => match val {
                Value::String(_) => DataType::Text,
                Value::Int(int) => {
                    if let Ok(_) = int.base10_parse::<i32>() {
                        DataType::Integer
                    } else if let Ok(_) = int.base10_parse::<i64>() {
                        DataType::BigInt
                    } else {
                        DataType::Numeric
                    }
                }
                Value::Float(_) => DataType::Numeric,
                Value::Boolean { .. } => DataType::Boolean,
                Value::ARRAY(exprs) => match exprs.first() {
                    Some(expr) => {
                        let ty = self.analyse_or_expr(expr)?;
                        for expr in &exprs.value {
                            let rest = self.analyse_or_expr(expr)?;
                            if ty != rest {
                                return err::msg(
                                    expr,
                                    format!("expected `{ty:?}` type, found `{rest:?}`"),
                                );
                            }
                        }
                        DataType::Array { ty: Box::new(ty) }
                    }
                    None => return err::msg(exprs, "cannot determine type of empty array"),
                },
                Value::Null { .. } => DataType::Unknown,
            },
            Term::Column(name) => match self.table {
                Some(table) => match utils::get_column(table, &name.alias) {
                    Ok(column) => column.data_type.clone(),
                    Err(_) => DataType::Unknown,
                },
                None => DataType::Unknown,
            },
            Term::Func(func) => match &func.value {
                FunctionKind::ABS(expr) => {
                    let ty = self.analyse_arithmetic(expr)?;
                    err::expect_numeric(&ty, expr)?;
                    ty
                }
                FunctionKind::ACOS(_) => todo!(),
                FunctionKind::ASIN(_) => todo!(),
                FunctionKind::ATAN(_) => todo!(),
                FunctionKind::COS(e) => {
                    let ty = self.analyse_arithmetic(e)?;
                    err::expect_numeric(&ty, e)?;
                    ty
                }
                FunctionKind::COSH(_) => todo!(),
                FunctionKind::COT(_) => todo!(),
                FunctionKind::SIN(_) => todo!(),
                FunctionKind::SINH(_) => todo!(),
                FunctionKind::TAN(_) => todo!(),
                FunctionKind::TANH(_) => todo!(),
                FunctionKind::ATAN2(_, _) => todo!(),
                FunctionKind::BITAND(_, _) => todo!(),
                FunctionKind::BITOR(_, _) => todo!(),
                FunctionKind::BITXOR(_, _) => todo!(),
                FunctionKind::BITNOT(_) => todo!(),
                FunctionKind::BITNAND(_, _) => todo!(),
                FunctionKind::BITNOR(_, _) => todo!(),
                FunctionKind::BITXNOR(_, _) => todo!(),
                FunctionKind::BITGET(_, _) => todo!(),
                FunctionKind::BITCOUNT(_) => todo!(),
                FunctionKind::LSHIFT(_, _) => todo!(),
                FunctionKind::RSHIFT(_, _) => todo!(),
                FunctionKind::ULSHIFT(_, _) => todo!(),
                FunctionKind::URSHIFT(_, _) => todo!(),
                FunctionKind::ROTATELEFT(_, _) => todo!(),
                FunctionKind::ROTATERIGHT(_, _) => todo!(),
                FunctionKind::MOD(_, _) => todo!(),
                FunctionKind::CEIL(_) => todo!(),
                FunctionKind::DEGREES(_) => todo!(),
                FunctionKind::EXP(_) => todo!(),
                FunctionKind::FLOOR(_) => todo!(),
                FunctionKind::LN(_) => todo!(),
                FunctionKind::LOG(_, _) => todo!(),
                FunctionKind::LOG10(_) => todo!(),
                FunctionKind::ORA_HASH(_) => todo!(),
                FunctionKind::RADIANS(_) => todo!(),
                FunctionKind::SQRT(_) => todo!(),
                FunctionKind::PI() => todo!(),
                FunctionKind::POWER(_, _) => todo!(),
                FunctionKind::RAND(_) => todo!(),
                FunctionKind::RANDOM_UUID() => todo!(),
                FunctionKind::ROUND(_) => todo!(),
                FunctionKind::SECURE_RAND(_) => todo!(),
                FunctionKind::SIGN(_) => todo!(),

                FunctionKind::ASCII(_) => todo!(),
                FunctionKind::CHAR_LENGTH(_) => todo!(),
                FunctionKind::CHAR(_) => todo!(),
                FunctionKind::CONCAT(_) => todo!(),
                FunctionKind::DIFFERENCE(_, _) => todo!(),
                FunctionKind::HEXTORAW(_) => todo!(),
                FunctionKind::LOWER(_) => todo!(),
                FunctionKind::UPPER(_) => todo!(),
                FunctionKind::LEFT(_, _) => todo!(),
                FunctionKind::RIGHT(_, _) => todo!(),
                FunctionKind::REPEAT(_, _) => todo!(),
                FunctionKind::SOUNDEX(_) => todo!(),
                FunctionKind::SPACE(_) => todo!(),
            },
            Term::OrExpr(expr) => self.analyse_or_expr(expr)?,
        })
    }
}
