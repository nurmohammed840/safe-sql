/// - https://www.h2database.com/html/grammar.html
/// - https://forcedotcom.github.io/phoenix/
/// - https://en.wikipedia.org/wiki/SQL_syntax
pub mod command;
pub mod function;
pub mod window;
pub mod grammar;
pub mod spanned;

pub mod utils;
pub use spanned::GetSpan;
pub use spanned::WithSpan;

use proc_macro2::Literal;
use proc_macro2::{Delimiter, Ident, Span, TokenTree};
use std::fmt;
use syn::{
    ext::IdentExt,
    parenthesized,
    parse::{discouraged::Speculative, Parse, ParseStream, Parser},
    punctuated::Punctuated,
    token::Paren,
    Error, LitFloat, LitInt, LitStr, Result, Token,
};
