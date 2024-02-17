use super::name::Name;
use crate::*;

#[derive(Debug)]
pub struct Column {
    pub schema_name: Option<Name>,
    pub table_name: Option<Name>,
    pub name: Name,
}

impl Parse for Column {
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
            name: input.parse()?,
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
        let _: Column = syntex! {
            scheman_name."table_alias"."awdd"
        }.unwrap();
        let _: Column = syntex! { adw }.unwrap();
        let _: Column = syntex! { "table_alias".dsd }.unwrap();
    }
}
