use self::grammar::Name;
use crate::*;
use grammar::ast::OrExpr;
use grammar::Column;
use utils::parse_keyword_if_matched;

pub enum SelectExpr {
    WildCard {
        symbol: Column<Token![*]>,
        except: Punctuated<Column<Name>, Token![,]>,
    },
    Expr {
        expr: OrExpr,
        alias: Option<Name>,
    },
}

pub enum SelectFilter {
    All,
    Distinct {
        kw: Ident,
    },
    DistinctON {
        on_kw: Ident,
        paren_token: Paren,
        exprs: Punctuated<OrExpr, Token![,]>,
    },
}

#[derive(Debug)]
pub struct Select {
    pub select_kw: Ident,
    pub filter: SelectFilter,
    pub exprs: Vec<SelectExpr>,
    pub from_kw: Ident,
}

impl Parse for Select {
    fn parse(input: ParseStream) -> Result<Self> {
        let select_kw = parse_keyword_if_matched(input, "SELECT")?;
        let filter = input.parse()?;
        let mut exprs = vec![];
        while !input.cursor().eof() {
            exprs.push(input.parse()?);
            if !input.peek(Token![,]) {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        let from_kw = parse_keyword_if_matched(input, "FROM")?;
        Ok(Self {
            select_kw,
            filter,
            exprs,
            from_kw,
        })
    }
}

impl Parse for SelectExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        if let Ok(symbol) = fork.parse() {
            let except = match parse_keyword_if_matched(&fork, "EXCEPT").ok() {
                None => Punctuated::new(),
                Some(_) => {
                    let content;
                    parenthesized!(content in fork);
                    content.call(Punctuated::parse_terminated)?
                }
            };
            input.advance_to(&fork);
            return Ok(Self::WildCard { symbol, except });
        }

        let expr = input.parse()?;
        let mut alias = None;

        if parse_keyword_if_matched(input, "AS").is_ok() {
            alias = Some(input.parse()?)
        }
        Ok(Self::Expr { expr, alias })
    }
}

impl Parse for SelectFilter {
    fn parse(input: ParseStream) -> Result<Self> {
        match parse_keyword_if_matched(input, "DISTINCT").ok() {
            None => {
                let _ = parse_keyword_if_matched(input, "ALL");
                Ok(SelectFilter::All)
            }
            Some(kw) => match parse_keyword_if_matched(input, "ON").ok() {
                None => Ok(SelectFilter::Distinct { kw }),
                Some(on_kw) => {
                    let content;
                    Ok(SelectFilter::DistinctON {
                        on_kw,
                        paren_token: parenthesized!(content in input),
                        exprs: content.call(Punctuated::parse_terminated)?,
                    })
                }
            },
        }
    }
}

impl fmt::Debug for SelectFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "All"),
            Self::Distinct { kw } => kw.fmt(f),
            Self::DistinctON { exprs, .. } => {
                f.write_str("DISTINCT ON ")?;
                exprs.iter().collect::<Vec<_>>().fmt(f)
            }
        }
    }
}

impl fmt::Debug for SelectExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WildCard { except, .. } => f
                .debug_struct("WildCard")
                .field("except", &except.iter().collect::<Vec<_>>())
                .finish(),
            Self::Expr { expr, alias } => f
                .debug_struct("Expr")
                .field("expr", expr)
                .field("alias", alias)
                .finish(),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn test_name() {
        // EXCEPT
        let g: Result<Select> = utils::test::syntex! {
            SELECT adad as awd, adad FROM
        };
        println!("{:#?}", g.unwrap());
    }
}
