use std::mem;

use crate::{cursor::Cursor, *};
// use std::ops::Range;

pub fn token_stream(bytes: &[u8]) -> Vec<TokenTree<()>> {
    let mut c = Cursor::from(bytes);
    let mut stack = vec![];
    let mut tokens = vec![];

    while let Some(ch) = c.peek().copied() {
        match ch {
            // skip whitespace
            b' ' | b'\t'..=b'\r' => {
                c.advance_by(1);
            }
            b'(' | b'{' | b'[' => {
                c.advance_by(1);
                stack.push((ch, mem::take(&mut tokens)));
            }
            b')' | b'}' | b']' => {
                c.advance_by(1);
                let Some((delimiter, stream)) = stack.pop() else {
                    panic!("Syntax Error: expected an item")
                };
                let delimiter = match (delimiter, ch) {
                    (b'(', b')') => Delimiter::Parenthesis,
                    (b'{', b'}') => Delimiter::Brace,
                    (b'[', b']') => Delimiter::Bracket,
                    _ => panic!("Syntax Error: mismatch delimeter"),
                };
                let stream = mem::replace(&mut tokens, stream);
                tokens.push(TokenTree::Group(Group {
                    span: (),
                    span_open: (),
                    span_close: (),
                    delimiter,
                    stream,
                }));
            }
            ch if is_punct(ch) => {
                c.advance_by(1);
                tokens.push(TokenTree::Punct(Punct {
                    span: (),
                    char: char::from(ch),
                    spacing: Spacing::Joint,
                }));
            }
            _ => {
                c.advance_by(1);
                tokens.push(TokenTree::Ident(Ident {
                    span: (),
                    name: char::from(ch).to_string(),
                }));
            }
        }
    }
    match stack.last() {
        Some(_frame) => panic!("error"),
        None => tokens,
    }
}

fn is_punct(ch: u8) -> bool {
    matches!(ch, b'!'..=b'&' | b'*'..=b'/' | b':'..=b'@' | b'^' | b'|' | b'~')
}

#[test]
fn tree() {
    println!("{:#?}", token_stream(b"03 ({[ 12 ]})"));
}

#[test]
fn ident() {
    let _d = (b'A'..=b'Z', b'a'..=b'z', b'_');
    println!("{:#?}", _d);
}

#[test]
fn punct() {
    let _d = (b'!'..=b'&', b'*'..=b'/', b':'..=b'@', b'^', b'|', b'~');
    println!("{:#?}", _d);

    let mut a = "~!@#$%^&*-=+|;:,<.>/?"
        .chars()
        .map(u32::from)
        .collect::<Vec<_>>();

    a.sort();

    for v in a {
        println!("{v}: {}", char::from_u32(v).unwrap());
    }
}
