use std::fmt::Display;

use super::*;

pub enum Name {
    /// Unquoted names are not case sensitive. There is no maximum name length
    Ident(Ident),
    /// Quoted names are case sensitive. and can contain spaces. There is no maximum name length.
    /// Two double quotes can be used to create a single double quote inside an identifier.
    String(Literal),
}

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Name::Ident(a) => f.write_str(&a.to_string()),
            Name::String(s) => f.write_str(&s.to_string()),
        }
    }
}

impl Parse for Name {
    fn parse(input: ParseStream) -> Result<Self> {
        let err_msg = "expected `Name`";
        input.step(|c| {
            let (tt, rest) = c.token_tree().ok_or(c.error(err_msg))?;
            let name = match tt {
                TokenTree::Ident(v) => Name::Ident(v),
                TokenTree::Literal(v) => {
                    if !v
                        .to_string()
                        .as_bytes()
                        .first()
                        .is_some_and(|ch| matches!(ch, b'c' | b'"'))
                    {
                        return Err(Error::new(v.span(), "invalid `Name`"));
                    }
                    Name::String(v)
                }
                tt => return Err(Error::new(tt.span(), err_msg)),
            };
            Ok((name, rest))
        })
    }
}

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
