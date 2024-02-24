use proc_macro::TokenStream;
use sql_parser::command::Command;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    match syn::parse::<Command>(input) {
        Ok(_input) => {
            let mut errs = sql_analyzer::analyse_command(_input).into_iter();
            if let Some((span, message)) = errs.next() {
                let mut err = syn::Error::new(span, message);
                for e in errs {
                    err.combine(syn::Error::new(e.0, e.1));
                }
                return err.into_compile_error().into();
            }
            TokenStream::new()
        }
        Err(err) => err.into_compile_error().into(),
    }
}
