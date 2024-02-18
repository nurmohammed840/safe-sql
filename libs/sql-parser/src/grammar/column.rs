use super::name::Name;
use crate::*;
use std::fmt::Write;

pub struct Column<T> {
    pub schema_name: Option<Name>,
    pub table_name: Option<Name>,
    pub alias: T,
}

impl<T: Parse> Parse for Column<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let get_name = || -> Result<Option<Name>> {
            if !input.peek2(Token![.]) {
                return Ok(None);
            }
            let name = input.parse()?;
            input.parse::<Token![.]>()?;
            Ok(Some(name))
        };
        let names = (get_name()?, get_name()?);
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

impl<T: fmt::Debug> fmt::Debug for Column<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.schema_name {
            name.fmt(f)?;
            f.write_char('.')?;
        }
        if let Some(name) = &self.table_name {
            name.fmt(f)?;
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
        let _: Column<Name> = syntex! { c"schema_name"."table_alias"."alias" }.unwrap();
        let _: Column<Name> = syntex! { field }.unwrap();
        let _: Column<Name> = syntex! { "document".field }.unwrap();
    }
}
