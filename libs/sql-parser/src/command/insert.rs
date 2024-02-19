use crate::{utils::parse_keyword_if_matched, *};
use grammar::{ast::OrExpr, Name, TableName};

/// Ref: https://www.h2database.com/html/commands.html#insert
pub struct InsertInto {
    pub kw: (Ident, Ident),
    pub table_name: TableName,
    pub column_name: Punctuated<Name, Token![,]>,
    pub values: InsertKind,
}

#[derive(Debug)]
pub enum InsertKind {
    Values { kw: Ident, rows: Vec<Row> },
    // OverridingUserValue(OverrideClause),
    // OverridingSystemValue(OverrideClause),
    DefaultValues(Ident),
}

// enum OverrideClause {
//     InsertExpr(InsertExpr),
//     Query(Query),
// }

impl Parse for InsertKind {
    fn parse(input: ParseStream) -> Result<Self> {
        let kw: Ident = input.parse()?;
        let kind = kw.to_string();
        if kind.eq_ignore_ascii_case("VALUES") {
            return Ok(Self::Values {
                kw: parse_keyword_if_matched(input, "VALUES")?,
                rows: {
                    let mut rows = vec![];
                    while !input.cursor().eof() {
                        rows.push(input.parse()?);
                        if !input.peek(Token![,]) {
                            break;
                        }
                        input.parse::<Token![,]>()?;
                    }
                    rows
                },
            });
        }
        if kind.eq_ignore_ascii_case("DEFAULT") {
            parse_keyword_if_matched(input, "VALUES")?;
            return Ok(Self::DefaultValues(kw));
        }
        todo!()
    }
}

pub enum Row {
    InsertExpr(InsertExpr),
    Row(Punctuated<InsertExpr, Token![,]>),
}

#[derive(Debug)]
pub enum InsertExpr {
    Default,
    Insert(OrExpr),
}

impl Parse for InsertExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        match parse_keyword_if_matched(input, "DEFAULT") {
            Ok(_) => Ok(Self::Default),
            Err(_) => Ok(Self::Insert(input.parse()?)),
        }
    }
}

impl Parse for Row {
    fn parse(input: ParseStream) -> Result<Self> {
        if parse_keyword_if_matched(input, "ROW").is_ok()
            || input.cursor().group(Delimiter::Parenthesis).is_some()
        {
            let contain;
            parenthesized!(contain in input);
            return Ok(Self::Row(contain.call(Punctuated::parse_terminated)?));
        }
        Ok(Self::InsertExpr(input.parse()?))
    }
}

impl Parse for InsertInto {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            kw: (
                parse_keyword_if_matched(input, "INSERT")?,
                parse_keyword_if_matched(input, "INTO")?,
            ),
            table_name: input.parse()?,
            column_name: {
                match input.cursor().group(Delimiter::Parenthesis) {
                    Some(_) => {
                        let contain;
                        parenthesized!(contain in input);
                        contain.call(Punctuated::parse_terminated)?
                    }
                    None => Punctuated::new(),
                }
            },
            values: input.parse()?,
        })
    }
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsertExpr(arg0) => f.debug_tuple("InsertExpr").field(arg0).finish(),
            Self::Row(arg0) => f
                .debug_tuple("Row")
                .field(&arg0.iter().collect::<Vec<_>>())
                .finish(),
        }
    }
}

impl fmt::Debug for InsertInto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InsertInto")
            .field("kw", &self.kw)
            .field("table_name", &self.table_name)
            .field("column_name", &self.column_name.iter().collect::<Vec<_>>())
            .field("values", &self.values)
            .finish()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn test_name() {
        // EXCEPT
        let g: Result<InsertInto> = utils::test::syntex! {
            INSERT INTO test (id, age) VALUES (1, "Hello")
        };
        println!("{:#?}", g);
    }
}
