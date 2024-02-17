use super::{ast::OrExpr, column::Column, value::Value, Name};
use crate::*;

#[derive(Debug)]
pub enum Term {
    Value(Value),
    Column(Column<Name>),
    OrExpr(Box<OrExpr>),
}

impl Parse for Term {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed_expr = input.step(|c| {
            let (tt, rest) = c.token_tree().ok_or(c.error("expected tokens"))?;
            if let TokenTree::Group(g) = tt {
                if let Delimiter::Parenthesis = g.delimiter() {
                    let tokens = g.stream();
                    let expr = |input: ParseStream| input.parse();
                    return Ok((Self::OrExpr(expr.parse2(tokens)?), rest));
                }
            }
            Err(c.error("continue parsing..."))
        });
        if let Ok(expr) = parsed_expr {
            return Ok(expr);
        }
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
