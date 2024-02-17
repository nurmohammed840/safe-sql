// mod commands;




// use proc_macro2::{TokenStream, TokenTree};
// use quote2::IntoTokens;
// use syn::Token;
// use syn::{
//     parse::{Parse, ParseStream},
//     punctuated::Punctuated,
//     Error, Ident, LitStr,
// };

// pub enum Name {
//     /// Unquoted names are not case sensitive. There is no maximum name length
//     Ident(Ident),
//     /// Quoted names are case sensitive. and can contain spaces. There is no maximum name length.
//     /// Two double quotes can be used to create a single double quote inside an identifier.
//     String(LitStr),
// }

// struct NameAlias {
//     name: Name,
//     alias: Option<(keyword::AS, Name)>,
// }

// pub struct FROM {
//     keyword: keyword::FROM,
//     table_name: NameAlias,
// }

// pub struct SELECT {
//     keyword: keyword::SELECT,
//     fields: Vec<Name>,
// }

// pub struct Query {
//     select: SELECT,
//     from: FROM,
// }

// mod keyword {
//     use syn::custom_keyword;
//     custom_keyword!(SELECT);
//     custom_keyword!(FROM);
//     custom_keyword!(WHERE);
//     custom_keyword!(GROUP);
//     custom_keyword!(AS);
//     // custom_keyword!(ASC);
//     // custom_keyword!(DESC);
// }

// impl Parse for Name {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         match input.parse()? {
//             TokenTree::Ident(v) => Ok(Self::Ident(v)),
//             TokenTree::Literal(v) => {
//                 let mut s = TokenStream::new();
//                 v.into_tokens(&mut s);
//                 Ok(Self::String(syn::parse2::<LitStr>(s)?))
//             }
//             tt => Err(Error::new(tt.span(), "expected table name")),
//         }
//     }
// }

// impl Parse for NameAlias {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         Ok(Self {
//             name: input.parse()?,
//             alias: {
//                 if let Some(kw) = input.parse::<Option<keyword::AS>>()? {
//                     let name = input.parse::<Name>()?;
//                     Some((kw, name))
//                 } else {
//                     None
//                 }
//             },
//         })
//     }
// }

// impl Parse for Query {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         Ok(Self {
//             select: input.parse()?,
//             from: input.parse()?,
//         })
//     }
// }

// impl Parse for SELECT {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         Ok(Self {
//             keyword: input.parse()?,
//             fields: {
//                 let is_end = input.peek(keyword::FROM);
//                 input.parse::<Name>()?;
//                 vec![]
//             },
//         })
//     }
// }

// impl Parse for FROM {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         Ok(FROM {
//             keyword: input.parse()?,
//             table_name: input.parse()?,
//         })
//     }
// }
