use self::{grammar::TableName, utils::parse_kw_if_matched};
use crate::*;

pub struct Update {
    pub kw: Ident,
    pub table_name: TableName,
}

impl Parse for Update {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Update {
            kw: parse_kw_if_matched(input, "UPDATE")?,
            table_name: input.parse()?,
        })
    }
}
