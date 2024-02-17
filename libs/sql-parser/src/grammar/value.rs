
use super::ast::OrExpr;
use crate::*;

pub enum Value {
    String(LitStr),
    Int(LitInt),
    Float(LitFloat),
    Boolean { span: Span, value: Option<bool> },
    ARRAY(Punctuated<OrExpr, Token![,]>),
    Null { span: Span },
}

impl Parse for Value {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            input.parse().map(Self::String)
        } else if lookahead.peek(LitInt) {
            input.parse().map(Self::Int)
        } else if lookahead.peek(LitFloat) {
            input.parse().map(Self::Float)
        } else if lookahead.peek(Ident::peek_any) {
            input.step(|s| {
                let (tt, rest) = s.ident().unwrap();
                let span = tt.span();
                let tt = tt.to_string();

                let val = if tt.eq_ignore_ascii_case("UNKNOWN") {
                    Self::Boolean { value: None, span }
                } else if tt.eq_ignore_ascii_case("TRUE") {
                    Self::Boolean {
                        value: Some(true),
                        span,
                    }
                } else if tt.eq_ignore_ascii_case("FALSE") {
                    Self::Boolean {
                        value: Some(false),
                        span,
                    }
                } else if tt.eq_ignore_ascii_case("ARRAY") {
                    let error = s.error("expected `[]`");
                    let (tt, rest) = rest.token_tree().ok_or(error.clone())?;
                    match tt {
                        TokenTree::Group(group) => {
                            if !matches!(group.delimiter(), Delimiter::Bracket) {
                                return Err(error);
                            }
                            let tokens = group.stream();
                            let punctuated = |a: ParseStream| a.call(Punctuated::parse_terminated);
                            return Ok((Self::ARRAY(punctuated.parse2(tokens)?), rest));
                        }
                        _ => return Err(error),
                    }
                } else if tt.eq_ignore_ascii_case("NULL") {
                    Self::Null { span }
                } else {
                    return Err(Error::new(span, "invalid value"));
                };
                Ok((val, rest))
            })
        } else {
            Err(lookahead.error())
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(arg0) => arg0.value().fmt(f),
            Self::Int(arg0) => arg0.to_string().fmt(f),
            Self::Float(arg0) => arg0.to_string().fmt(f),
            Self::Boolean { value, .. } => value.fmt(f),
            Self::ARRAY(v) => v.iter().collect::<Vec<_>>().fmt(f),
            Self::Null { .. } => "Null".fmt(f),
        }
    }
}
