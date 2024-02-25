use crate::{
    schema_info::{Column, SchemaInfo, Table},
    AnalyseError,
};
use sql_parser::{grammar::Name, utils::suggest};

pub fn get_table<'a>(info: &'a SchemaInfo, name: &Name) -> Result<&'a Table, AnalyseError> {
    let (name, span) = (name.to_string(), name.span());

    let tables = info
        .get_public_tables()
        .ok_or_else(|| (span, "no table found".to_string()))?;

    tables.get(&name).ok_or_else(|| {
        (
            span,
            format!(
                "table does not exist: `{name}` \nsuggest: {}",
                suggest(&name, tables.keys())
            ),
        )
    })
}

pub fn get_column<'a>(table: &'a Table, name: &Name) -> Result<&'a Column, AnalyseError> {
    let (name, span) = (name.to_string(), name.span());
    table.get(&name).ok_or_else(|| {
        (
            span,
            format!(
                "column does not exist: `{name}` \nsuggest: {}",
                suggest(&name, table.keys())
            ),
        )
    })
}
