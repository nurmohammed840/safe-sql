/// - https://www.h2database.com/html/grammar.html
/// - https://forcedotcom.github.io/phoenix/
/// - https://en.wikipedia.org/wiki/SQL_syntax
pub mod command;
pub mod grammar;

pub(crate) mod utils;

use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::TokenStreamExt;
use syn::{
    parse::{Parse, ParseStream},
    Error,  LitFloat, LitInt, LitStr, Result, Token,
};
use syn::ext::IdentExt;
use proc_macro2::Span;
use syn::parse::discouraged::Speculative;
use syn::punctuated::Punctuated;
