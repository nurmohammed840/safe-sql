use crate::*;

#[derive(Debug)]
pub enum Value<Span> {
    String(Literal<Span>),
    // Int(LitInt),
    // Float(LitFloat),
    Boolean(Ident<Span>),
    // ARRAY(WithSpan<Punctuated<OrExpr, Token![,]>>),
    // Null { span: Span },
}

impl<Span: Clone> Parse<Span> for Value<Span> {
    fn parse(cursor: &mut Cursor<Span>) -> Result<Self, Diagnostic<Span>> {
        let head = cursor.fork();
        if let Some(tt) = cursor.next() {
            match tt {
                TokenTree::Group(_) => todo!(),
                TokenTree::Ident(ident) => {
                    if ident.name.eq_ignore_ascii_case("TRUE")
                        || ident.name.eq_ignore_ascii_case("FALSE")
                        || ident.name.eq_ignore_ascii_case("UNKNOWN")
                    {
                        return Ok(Self::Boolean(ident.clone()));
                    }
                }
                TokenTree::Punct(_) => todo!(),
                TokenTree::Literal(lit) => {
                    if lit.value.starts_with("'") && lit.value.ends_with("'") {
                        return Ok(Self::String(lit.clone()));
                    }
                }
            }
        }
        return Err(head.error("invalid value"));
    }
}

#[test]
fn test_name() {
    let mut cursor = Cursor {
        scope: &857,
        tokens: &[
            TokenTree::Ident(Ident {
                span: 1,
                name: "true".into(),
            }),
            TokenTree::Ident(Ident {
                span: 2,
                name: "false".into(),
            }),
            TokenTree::Literal(Literal {
                span: 3,
                value: "false".into(),
            }),
        ],
    };
    println!("{:#?}", cursor.parse::<Value<u32>>());
    println!("{:#?}", cursor.parse::<Value<u32>>());
    println!("{:#?}", cursor.parse::<Value<u32>>());
}
