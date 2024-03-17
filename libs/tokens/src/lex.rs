// pub struct Lex<I> {
//     iter: I,
//     token: String,
// }

// impl<I> Iterator for Lex<I>
// where
//     I: Iterator<Item = u8>,
// {
//     type Item = ();

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.iter.next() {
//             Some(byte) => match byte {

//                 _ => self.token.push(char::from(byte)),
//             },
//             None => {

//             }
//         }

//         todo!()
//     }
// }

use crate::TokenTree;
use std::ops::Range;

pub fn token_stream(_c: &[u8]) -> Vec<TokenTree<Range<u32>>> {
    let tokens = vec![];
    
    tokens
}
