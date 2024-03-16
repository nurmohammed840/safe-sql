use crate::schema_info::SchemaInfo;
use crate::{err, utils, AnalyseError, DataType, Table};
use sql_parser::grammar::{ast::*, Term};
use sql_parser::GetSpan;
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
                FunctionKind::PI() | FunctionKind::SIGN(_) => DataType::DoublePrecision,
                FunctionKind::ABS(e)
                | FunctionKind::CEIL(e)
                | FunctionKind::FLOOR(e)
                | FunctionKind::ROUND(e) => self.expect_numeric(e)?,

                FunctionKind::ACOS(e)
                | FunctionKind::ASIN(e)
                | FunctionKind::ATAN(e)
                | FunctionKind::COS(e)
                | FunctionKind::COSH(e)
                | FunctionKind::COT(e)
                | FunctionKind::SIN(e)
                | FunctionKind::SINH(e)
                | FunctionKind::TAN(e)
                | FunctionKind::TANH(e)
                | FunctionKind::DEGREES(e)
                | FunctionKind::EXP(e)
                | FunctionKind::LN(e)
                | FunctionKind::LOG(_, e)
                | FunctionKind::LOG10(e)
                | FunctionKind::RADIANS(e)
                | FunctionKind::SQRT(e)
                | FunctionKind::POWER(e, _) => {
                    self.expect_numeric(e)?;
                    DataType::DoublePrecision
                }
                FunctionKind::ATAN2(e1, e2) => {
                    self.expect_numeric(e1)?;
                    self.expect_numeric(e2)?;
                    DataType::DoublePrecision
                }

                FunctionKind::BITAND(e1, e2)
                | FunctionKind::BITOR(e1, e2)
                | FunctionKind::BITXOR(e1, e2)
                | FunctionKind::BITNAND(e1, e2)
                | FunctionKind::BITNOR(e1, e2)
                | FunctionKind::BITXNOR(e1, e2) => {
                    let lhs_ty = self.get_bitwise_ty(e1)?;
                    let rhs_ty = self.analyse_or_expr(e2)?;
                    check_same_type(e2, &lhs_ty, &rhs_ty)?;
                    lhs_ty
                }

                FunctionKind::BITNOT(e)
                | FunctionKind::BITCOUNT(e)
                | FunctionKind::BITGET(e, _)
                | FunctionKind::LSHIFT(e, _)
                | FunctionKind::RSHIFT(e, _)
                | FunctionKind::ULSHIFT(e, _)
                | FunctionKind::URSHIFT(e, _)
                | FunctionKind::ROTATELEFT(e, _)
                | FunctionKind::ROTATERIGHT(e, _) => self.get_bitwise_ty(e)?,

                FunctionKind::MOD(e1, e2) => {
                    let lhs_ty = self.analyse_arithmetic(e1)?;
                    let rhs_ty = self.analyse_arithmetic(e2)?;
                    check_same_type(e2, &lhs_ty, &rhs_ty)?;
                    lhs_ty
                }
                // String Functions
                FunctionKind::ASCII(_) | FunctionKind::CHAR_LENGTH(_) => DataType::Integer,
                FunctionKind::CONCAT(_)
                | FunctionKind::LOWER(_)
                | FunctionKind::UPPER(_)
                | FunctionKind::LEFT(_, _)
                | FunctionKind::RIGHT(_, _)
                | FunctionKind::REPEAT(_, _)
                | FunctionKind::SPACE(_) => DataType::Text,

                FunctionKind::AVG() => todo!(),
                FunctionKind::UnknownFunc(name, _) => return err::msg(name.span(), "unknown function"),
            },
            Term::OrExpr(expr) => self.analyse_or_expr(expr)?,
        })
    }
}

fn check_same_type(
    span: impl GetSpan,
    lhs_ty: &DataType,
    rhs_ty: &DataType,
) -> Result<(), AnalyseError> {
    if *lhs_ty != *rhs_ty {
        return err::msg(
            span,
            format!("expected data type: {lhs_ty:?}, got: {rhs_ty:?}"),
        );
    }
    Ok(())
}

impl AnalyseExpr<'_> {
    fn expect_numeric(&mut self, e: &Arithmetic) -> Result<DataType, AnalyseError> {
        let ty = self.analyse_arithmetic(e)?;
        err::expect_numeric(&ty, e)?;
        Ok(ty)
    }

    fn get_bitwise_ty(&mut self, e: &OrExpr) -> Result<DataType, AnalyseError> {
        let ty = self.analyse_or_expr(e)?;
        match ty {
            DataType::TINYINT | DataType::SmallInt | DataType::Integer | DataType::BigInt => Ok(ty),
            _ => err::msg(e,  format!("arguments should have TINYINT, SMALLINT, INTEGER, BIGINT, BINARY, or BINARY VARYING data type, but got: {ty:?}"))
        }
    }
}
