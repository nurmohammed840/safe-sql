/// - https://www.h2database.com/html/grammar.html
/// - https://forcedotcom.github.io/phoenix/
/// - https://en.wikipedia.org/wiki/SQL_syntax
pub mod command;
pub mod grammar;

pub(crate) mod utils;

use proc_macro2::{Delimiter, Ident, Span, TokenStream, TokenTree};
use quote::TokenStreamExt;
use std::fmt;
use syn::{
    ext::IdentExt,
    parse::{discouraged::Speculative, Parse, ParseStream, Parser},
    punctuated::Punctuated,
    Error, LitFloat, LitInt, LitStr, Result, Token,
};
