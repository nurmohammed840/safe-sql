use super::*;

pub enum Name {
    /// Unquoted names are not case sensitive. There is no maximum name length
    Ident(Ident),
    /// Quoted names are case sensitive. and can contain spaces. There is no maximum name length.
    /// Two double quotes can be used to create a single double quote inside an identifier.
    String(LitStr),
}

impl Name {
    // pub fn to_string(&self) -> String {
    //     match self {
    //         Name::Ident(a) => a.to_string(),
    //         Name::String(s) => s.value(),
    //     }
    // }
}

impl Parse for Name {
    fn parse(input: ParseStream) -> Result<Self> {
        input.step(|c| {
            let Some((tt, rest)) = c.token_tree() else {
                return Err(c.error("expected name"));
            };
            let name = match tt {
                TokenTree::Ident(v) => Name::Ident(v),
                TokenTree::Literal(v) => {
                    let mut s = TokenStream::new();
                    s.append(v);
                    Name::String(syn::parse2::<LitStr>(s)?)
                }
                tt => return Err(Error::new(tt.span(), "expected name")),
            };
            Ok((name, rest))
        })
    }
}

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(arg0) => arg0.fmt(f),
            Self::String(arg0) => arg0.value().fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_num() {
        // it should not pass
        let _: Name = utils::test::syntex! {
            TRUE
        }
        .unwrap();
    }
}
