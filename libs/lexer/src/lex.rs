use crate::cursor::IterExt;
use crate::*;
use std::mem;
use std::ops::Range;
use std::slice::Iter;

type Result<T, E = Diagnostic<Range<u32>>> = std::result::Result<T, E>;

struct Offset {
    bytes_len: u32,
}
impl Offset {
    fn new(bytes: &[u8]) -> Self {
        Self {
            bytes_len: bytes.len() as u32,
        }
    }
    fn get(&self, c: &Iter<u8>) -> u32 {
        self.bytes_len - c.len() as u32
    }
}

pub fn token_stream(bytes: &[u8]) -> Result<Vec<TokenTree<Range<u32>>>> {
    let offset = Offset::new(bytes);
    let mut c = bytes.iter();
    let mut stack = vec![];
    let mut tokens = vec![];

    loop {
        let start = offset.get(&c);
        let Some(&ch) = c.next() else { break };
        match ch {
            // skip whitespace
            b' ' | b'\t'..=b'\r' => {}
            b'(' | b'{' | b'[' => {
                stack.push((ch, start..start + 1, mem::take(&mut tokens)));
            }
            b')' | b'}' | b']' => {
                let span_close = start..start + 1;
                let Some((delimiter, span_open, stream)) = stack.pop() else {
                    return Err(Diagnostic::spanned(
                        vec![span_close],
                        Level::Error,
                        "Syntax Error: expected an item",
                    ));
                };
                let delimiter = match (delimiter, ch) {
                    (b'(', b')') => Delimiter::Parenthesis,
                    (b'{', b'}') => Delimiter::Brace,
                    (b'[', b']') => Delimiter::Bracket,
                    _ => {
                        return Err(Diagnostic::spanned(
                            vec![span_close],
                            Level::Error,
                            "Syntax Error: mismatch delimeter",
                        )
                        .span_note(vec![span_open], "open delimeter"));
                    }
                };
                let stream = mem::replace(&mut tokens, stream);
                tokens.push(TokenTree::Group(Group {
                    span: span_open.start..span_close.end,
                    span_open,
                    span_close,
                    delimiter,
                    stream,
                }));
            }
            delimiter @ (b'"' | b'\'') => {
                parse_string(&mut c, delimiter)
                    .map_err(|msg| Diagnostic::new(Level::Error, msg))?;

                let span = start..offset.get(&c);
                let value = data(bytes, span.clone());
                tokens.push(TokenTree::Literal(Literal { span, value }));
            }
            b'0'..=b'9' => {
                let result = if ch == b'0' {
                    match c.next() {
                        Some(b'b') => parse_num(&mut c, |ch| matches!(ch, b'0' | b'1')),
                        Some(b'o') => parse_num(&mut c, |ch| matches!(ch, b'0'..=b'7')),
                        Some(b'x') => parse_num(
                            &mut c,
                            |ch| matches!(ch, b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f'),
                        ),
                        _ => parse_num_base10(&mut c),
                    }
                } else {
                    parse_num_base10(&mut c)
                };
                let span = start..offset.get(&c);
                result.map_err(|msg| Diagnostic::spanned(vec![span.clone()], Level::Error, msg))?;

                let value = data(bytes, span.clone());
                tokens.push(TokenTree::Literal(Literal { span, value }));
            }
            _ if is_punct(ch) => {
                tokens.push(TokenTree::Punct(Punct {
                    span: start..offset.get(&c),
                    char: ch.into(),
                    spacing: match c.peek() {
                        Some(ch) if is_punct(*ch) => Spacing::Joint,
                        _ => Spacing::Alone,
                    },
                }));
            }
            _ if is_ident_start(ch) => {
                skip_while(&mut c, is_ident_continue);
                let span = start..offset.get(&c);
                let name = data(bytes, span.clone());
                tokens.push(TokenTree::Ident(Ident { span, name }));
            }
            _ => {
                return Err(Diagnostic::spanned(
                    vec![(start as u32)..offset.get(&c) as u32],
                    Level::Error,
                    format!("unknown charecter: {}", char::from(ch)),
                ));
            }
        }
    }
    match stack.last() {
        Some(_frame) => Err(Diagnostic::new(Level::Error, "ERROR")),
        None => Ok(tokens),
    }
}

#[inline]
fn data(bytes: &[u8], Range { start, end }: Range<u32>) -> String {
    String::from_utf8(bytes[(start as usize)..end as usize].to_vec()).unwrap()
}

fn skip_and_count_while(c: &mut Iter<u8>, cb: fn(u8) -> bool) -> usize {
    let len = c.len();
    skip_while(c, cb);
    len - c.len()
}

fn skip_while(c: &mut Iter<u8>, cb: fn(u8) -> bool) {
    while let Some(&ch) = c.peek() {
        if !cb(ch) {
            break;
        }
        c.next();
    }
}

fn parse_string(fork: &mut Iter<u8>, delimiter: u8) -> Result<(), &'static str> {
    while let Some(ch) = fork.next() {
        match ch {
            b'\\' => match fork.next() {
                Some(b'x') => {
                    if !(matches!(fork.next(), Some(b'0'..=b'7'))
                        && matches!(fork.next(), Some(b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f')))
                    {
                        return Err("Syntax Error: invalid character in numeric character escape");
                    }
                }
                Some(b'n' | b'r' | b't' | b'\\' | b'\'' | b'"' | b'0') => {}
                _ => return Err("Syntax Error: unknown character escape"),
            },
            ch if *ch == delimiter => return Ok(()),
            _ => {}
        }
    }
    Err("Syntax Error: Missing trailing `\"` symbol to terminate the string literal")
}

fn parse_num_base10(c: &mut Iter<'_, u8>) -> Result<(), String> {
    if c.len() == 0 {
        return Ok(());
    }
    parse_num(c, |ch| matches!(ch, b'0'..=b'9'))
}

fn parse_num(c: &mut Iter<u8>, is_digit: fn(u8) -> bool) -> Result<(), String> {
    if skip_and_count_while(c, is_digit) == 0 {
        return Err("Syntax Error: Missing digits after the integer base prefix".into());
    }
    {
        let data = c.as_slice();
        let amt = skip_and_count_while(c, is_ident_continue);
        if amt != 0 {
            let suffix = unsafe { std::str::from_utf8_unchecked(&data[..amt]) };
            return Err(format!(
                "Syntax Error: invalid suffix `{suffix}` for number literal"
            ));
        }
    }
    Ok(())
}

fn is_punct(ch: u8) -> bool {
    matches!(ch, b'!'..=b'&' | b'*'..=b'/' | b':'..=b'@' | b'^' | b'|' | b'~')
}
fn is_ident_start(ch: u8) -> bool {
    matches!(ch, b'A'..=b'Z' | b'_' | b'a'..=b'z')
}
fn is_ident_continue(ch: u8) -> bool {
    matches!(ch, b'0'..=b'9'| b'A'..=b'Z' | b'_' | b'a'..=b'z')
}

#[test]
fn tree() {
    let input = b"0b1";
    println!("{:#?}", token_stream(input));
    // println!("{:#?}", std::str::from_utf8(&input[3..10]));
}
