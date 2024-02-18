/// - https://www.h2database.com/html/grammar.html
/// - https://forcedotcom.github.io/phoenix/
/// - https://en.wikipedia.org/wiki/SQL_syntax
pub mod command;
pub mod grammar;

pub(crate) mod utils;

use proc_macro2::{Delimiter, Ident, Span, TokenTree};
use std::fmt;
use syn::{
    ext::IdentExt,
    parenthesized,
    parse::{discouraged::Speculative, Parse, ParseStream, Parser},
    punctuated::Punctuated,
    token::Paren,
    Error, LitFloat, LitInt, LitStr, Result, Token
};
use proc_macro2::Literal;
