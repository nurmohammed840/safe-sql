use super::{ast::Expression, column::Column, value::Value, Name};
use crate::*;

#[derive(Debug)]
pub enum Term {
    Value(Value),
    Column(Column<Name>),
    // Expression(Box<Expression>),
}

impl Parse for Term {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        if let Ok(v) = fork.parse::<Value>() {
            input.advance_to(&fork);
            return Ok(Term::Value(v));
        }
        if let Ok(v) = fork.parse() {
            input.advance_to(&fork);
            return Ok(Term::Column(v));
        }
        Err(input.error("invalid `Term`"))
    }
}
