use crate::{utils::parse_keyword_if_matched, *};
use grammar::ast::OrExpr;
use grammar::TableName;

#[derive(Debug)]
pub struct Delete {
    /// DELETE FROM
    pub kw: (Ident, Ident),
    pub table_name: TableName,
    pub where_expr: Option<OrExpr>,
}

impl Parse for Delete {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Delete {
            kw: (
                parse_keyword_if_matched(input, "DELETE")?,
                parse_keyword_if_matched(input, "FROM")?,
            ),
            table_name: input.parse()?,
            where_expr: {
                match parse_keyword_if_matched(input, "WHERE") {
                    Ok(_) => Some(input.parse()?),
                    Err(_) => None,
                }
            },
        })
    }
}


#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn test_name() {
        // EXCEPT
        let g: Result<Delete> = utils::test::syntex! {
            DELETE FROM test 
            WHERE
                id = 2 AND name = 353
        };
        println!("{:#?}", g);
    }
}
