use super::name::Name;
use crate::*;

// type WildCard = Column<Token![*]>;

#[derive(Debug)]
pub struct Column<T> {
    pub schema_name: Option<Name>,
    pub table_name: Option<Name>,
    pub alias: T,
}

impl<T: Parse> Parse for Column<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let fetch_name = || -> Result<Option<Name>> {
            if input.peek2(Token![.]) {
                let name = input.parse()?;
                input.parse::<Token![.]>()?;
                return Ok(Some(name));
            }
            Ok(None)
        };
        let names = (fetch_name()?, fetch_name()?);
        let mut column = Self {
            schema_name: None,
            table_name: None,
            alias: input.parse()?,
        };
        match names {
            (Some(schema_name), Some(table_name)) => {
                column.schema_name = Some(schema_name);
                column.table_name = Some(table_name);
            }
            (Some(table_name), None) => column.table_name = Some(table_name),
            _ => {}
        };
        Ok(column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn parse_column() {
        let _: Column<Name> = syntex! {
            scheman_name."table_alias"."column"
        }
        .unwrap();
        let _: Column<Name> = syntex! { field }.unwrap();
        let _: Column<Name> = syntex! { "document".field }.unwrap();
    }
}
