use crate::{schema_info::DataType, AnalyseError};
use sql_parser::GetSpan;

pub fn msg<T>(span: impl GetSpan, msg: impl Into<String>) -> Result<T, AnalyseError> {
    Err((span.get_span(), msg.into()))
}

pub fn expect_bool(ty: &DataType, span: impl GetSpan) -> Result<(), AnalyseError> {
    if !ty.is_bool() {
        return msg(span, format!("expected `boolean` type, found `{ty:?}`"));
    }
    Ok(())
}

pub fn expect_numeric(ty: &DataType, span: impl GetSpan) -> Result<(), AnalyseError> {
    if !ty.is_numeric() {
        return msg(span, format!("expected `numeric` type, found `{ty:?}`"));
    }
    Ok(())
}
