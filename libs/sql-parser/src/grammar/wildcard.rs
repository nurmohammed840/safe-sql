use super::*;

pub struct WildCardExpr {
    pub scheman_name: Option<Name>,
    pub table_alias: Option<Name>,
    pub symbol: Token![*],
}

impl Parse for WildCardExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let parse_name = || -> Result<Option<Name>> {
            if input.peek(Token![*]) {
                Ok(None)
            } else {
                let name = input.parse::<Name>()?;
                let _dot = input.parse::<Token![.]>()?;
                Ok(Some(name))
            }
        };
        let scheman_name = parse_name()?;
        let table_alias = parse_name()?;
        let mut wild_card_expr = Self {
            scheman_name: None,
            table_alias: None,
            symbol: input.parse()?,
        };
        match (scheman_name, table_alias) {
            (Some(scheman_name), Some(table_alias)) => {
                wild_card_expr.scheman_name = Some(scheman_name);
                wild_card_expr.table_alias = Some(table_alias);
            }
            (Some(table_alias), None) => {
                wild_card_expr.table_alias = Some(table_alias);
            }
            _ => {}
        }
        Ok(wild_card_expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    macro_rules! syntex {
        ($($t:tt)*) => { syn::parse2(TokenStream::from_str(stringify!($($t)*)).unwrap()) };
    }

    #[test]
    fn parse_wild_card_expr() {
        let _: WildCardExpr = syntex! {
            scheman_name."table_alias".*
        }
        .unwrap();
        let _: WildCardExpr = syntex! { * }.unwrap();
        let _: WildCardExpr = syntex! { table_alias.* }.unwrap();
    }
}
