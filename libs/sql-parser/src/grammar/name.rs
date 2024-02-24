use crate::*;
use std::fmt::Display;
use std::fmt::Write;

#[derive(Clone)]
pub enum Name {
    /// Unquoted names are not case sensitive. There is no maximum name length
    Ident(Ident),
    /// Quoted names are case sensitive. and can contain spaces. There is no maximum name length.
    /// Two double quotes can be used to create a single double quote inside an identifier.
    String(Literal),
}

impl Name {
    pub fn span(&self) -> Span {
        match self {
            Name::Ident(v) => v.span(),
            Name::String(v) => v.span(),
        }
    }
}

pub struct Column<T> {
    pub schema_name: Option<Name>,
    pub table_name: Option<Name>,
    pub alias: T,
}

#[derive(Debug, Clone)]
pub struct TableName {
    pub schema_name: Option<Name>,
    pub table_name: Name,
}

fn get_name(input: ParseStream) -> Result<Option<Name>> {
    if !input.peek2(Token![.]) {
        return Ok(None);
    }
    let name = input.parse()?;
    input.parse::<Token![.]>()?;
    Ok(Some(name))
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
                        .is_some_and(|ch| matches!(ch, b'r' | b'"'))
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

impl<T: Parse> Parse for Column<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let names = (get_name(input)?, get_name(input)?);
        let mut column = Self {
            schema_name: None,
            table_name: None,
            alias: input.parse()?,
        };
        match names {
            (schema_name @ Some(_), table_name @ Some(_)) => {
                column.schema_name = schema_name;
                column.table_name = table_name;
            }
            (table_name @ Some(_), None) => column.table_name = table_name,
            _ => {}
        };
        Ok(column)
    }
}

impl Parse for TableName {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            schema_name: get_name(input)?,
            table_name: input.parse()?,
        })
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Name::Ident(a) => f.write_str(&a.to_string()),
            Name::String(s) => f.write_str(&s.to_string()),
        }
    }
}

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl<T: fmt::Debug> fmt::Debug for Column<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.schema_name {
            f.write_str(&name.to_string())?;
            f.write_char('.')?;
        }
        if let Some(name) = &self.table_name {
            f.write_str(&name.to_string())?;
            f.write_char('.')?;
        }
        self.alias.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_column() {
        let _: Column<Name> = syntex! { r"schema_name"."table_alias"."alias" }.unwrap();
        let _: Column<Name> = syntex! { field }.unwrap();
        let _: Column<Name> = syntex! { "document".field }.unwrap();
    }
}
