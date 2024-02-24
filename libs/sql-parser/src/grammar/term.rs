use function::Function;
use syn::token;

use super::{ast::OrExpr, value::Value, Column, Name};
use crate::*;

pub enum Term {
    Value(Value),
    Column(Column<Name>),
    Func(Box<Function>),
    OrExpr(Box<OrExpr>),
}

impl Parse for Term {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(token::Paren) {
            let content;
            parenthesized!(content in input);
            return Ok(Self::OrExpr(content.parse()?));
        }
        
        if input.peek(Ident::peek_any) && input.peek2(token::Paren) {
            return Ok(Self::Func(input.parse()?));
        }

        if input
            .cursor()
            .literal()
            .is_some_and(|(v, _)| v.to_string().starts_with("r\""))
        {
            return Ok(Term::Column(input.parse()?));
        }

        if let Ok(v) = input.parse() {
            return Ok(Term::Value(v));
        }

        Ok(Term::Column(input.parse()?))
        // Err(input.error("invalid `Term`"))
    }
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(arg0) => f.debug_tuple("Value").field(arg0).finish(),
            Self::Column(arg0) => f.debug_tuple("Column").field(arg0).finish(),
            Self::Func(_) => f.debug_tuple("Func").finish(),
            Self::OrExpr(arg0) => f.debug_tuple("OrExpr").field(arg0).finish(),
        }
    }
}
