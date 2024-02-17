mod query_parser;

use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2;
// use query_parser::Query;

#[proc_macro]
pub fn sqlx(input: TokenStream) -> TokenStream {
    // match syn::parse::<Query>(input) {
    //     Ok(_input) => TokenStream::new(),
    //     Err(err) => err.into_compile_error().into(),
    // }
    input
}
