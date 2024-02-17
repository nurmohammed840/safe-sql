use super::{column::Column, value::Value};
use crate::*;

#[derive(Debug)]
pub enum Term {
    Value(Value),
    Column(Column),
}

impl Parse for Term {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        if let Ok(v) = fork.parse::<Value>() {
            input.advance_to(&fork);
            return Ok(Term::Value(v));
        }
        if let Ok(v) = fork.parse::<Column>() {
            input.advance_to(&fork);
            return Ok(Term::Column(v));
        }
        Err(input.error("invalid `Term`"))
    }
}
