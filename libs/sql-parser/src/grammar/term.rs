use crate::*;
use super::{ast::OrExpr, value::Value, Column, Name};

#[derive(Debug)]
pub enum Term {
    Value(Value),
    Column(Column<Name>),
    OrExpr(Box<OrExpr>),
}

impl Parse for Term {
    fn parse(input: ParseStream) -> Result<Self> {
        let parse_group_expr = input.step(|c| {
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

        if let Ok(expr) = parse_group_expr {
            return Ok(expr);
        }
        if input
            .cursor()
            .literal()
            .is_some_and(|(v, _)| v.to_string().starts_with("c\""))
        {
            return Ok(Term::Column(input.parse()?));
        }
        if let Ok(v) = input.parse() {
            return Ok(Term::Value(v));
        }
        if let Ok(v) = input.parse() {
            return Ok(Term::Column(v));
        }
        Err(input.error("invalid `Term`"))
    }
}
