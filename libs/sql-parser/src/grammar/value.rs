use super::ast::OrExpr;
use crate::*;

pub enum Value {
    String(LitStr),
    Int(LitInt),
    Float(LitFloat),
    Boolean(WithSpan<Option<bool>>),
    ARRAY(WithSpan<Punctuated<OrExpr, Token![,]>>),
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
                    Self::Boolean(WithSpan::new(span, None))
                } else if tt.eq_ignore_ascii_case("TRUE") {
                    Self::Boolean(WithSpan::new(span, Some(true)))
                } else if tt.eq_ignore_ascii_case("FALSE") {
                    Self::Boolean(WithSpan::new(span, Some(false)))
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
                            return Ok((Self::ARRAY(WithSpan::new(group.span(), punctuated.parse2(tokens)?)), rest));
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

impl GetSpan for Value {
    fn get_span(&self) -> Span {
        match self {
            Value::String(v) => v.span(),
            Value::Int(v) => v.span(),
            Value::Float(v) => v.span(),
            Value::Boolean(v) => v.get_span(),
            Value::ARRAY(v) => v.get_span(),
            Value::Null { span } => *span,
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(arg0) => arg0.value().fmt(f),
            Self::Int(arg0) => arg0.to_string().fmt(f),
            Self::Float(arg0) => arg0.to_string().fmt(f),
            Self::Boolean(v) => v.fmt(f),
            Self::ARRAY(v) => v.iter().collect::<Vec<_>>().fmt(f),
            Self::Null { .. } => "Null".fmt(f),
        }
    }
}
