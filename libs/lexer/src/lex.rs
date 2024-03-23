use crate::{cursor::Cursor, *};
use std::mem;
// use std::ops::Range;

pub fn token_stream(bytes: &[u8]) -> Result<Vec<TokenTree<()>>, Diagnostic<()>> {
    let mut c = Cursor::from(bytes);
    let mut stack = vec![];
    let mut tokens = vec![];

    while let Some(&ch) = c.peek() {
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
                    return Err(Diagnostic::new(
                        Level::Error,
                        "Syntax Error: expected an item",
                    ));
                };
                let delimiter = match (delimiter, ch) {
                    (b'(', b')') => Delimiter::Parenthesis,
                    (b'{', b'}') => Delimiter::Brace,
                    (b'[', b']') => Delimiter::Bracket,
                    _ => {
                        return Err(Diagnostic::new(
                            Level::Error,
                            "Syntax Error: mismatch delimeter",
                        ));
                    }
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
            b'"' => {
                let len = cook_string(&c);
            }
            b'0'..=b'9' => {
                let len = (ch == b'0')
                    .then(|| {
                        let mut fork = c.fork();
                        fork.advance_by(1); // 0
                        match fork.next()? {
                            b'b' => Some(
                                parse_num(fork, |ch| matches!(ch, b'0' | b'1')).map(|len| len + 2),
                            ),
                            b'o' => Some(
                                parse_num(fork, |ch| matches!(ch, b'0'..=b'7')).map(|len| len + 2),
                            ),
                            b'x' => Some(
                                parse_num(
                                    fork,
                                    |ch| matches!(ch, b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f'),
                                )
                                .map(|len| len + 2),
                            ),
                            _ => None,
                        }
                    })
                    .flatten()
                    .unwrap_or_else(|| parse_num(c.fork(), |ch| matches!(ch, b'0'..=b'9')))?;

                let value = unsafe { consume_string(&mut c, len) };
                tokens.push(TokenTree::Literal(Literal { span: (), value }));
            }
            _ if is_punct(ch) => {
                c.advance_by(1);
                tokens.push(TokenTree::Punct(Punct {
                    span: (),
                    char: ch.into(),
                    spacing: match c.peek() {
                        Some(ch) if is_punct(*ch) => Spacing::Joint,
                        _ => Spacing::Alone,
                    },
                }));
            }
            _ if is_ident_start(ch) => {
                let len = c.fork().take_while_and_count(is_ident_continue);
                let name = unsafe { consume_string(&mut c, len) };
                tokens.push(TokenTree::Ident(Ident { span: (), name }));
            }
            _ => {
                return Err(Diagnostic::new(
                    Level::Error,
                    format!("unknown charecter: {ch}"),
                ));
            }
        }
    }
    match stack.last() {
        Some(_frame) => Err(Diagnostic::new(Level::Error, "ERROR")),
        None => Ok(tokens),
    }
}

fn cook_string(c: &Cursor<'_, u8>) -> Result<usize, Diagnostic<()>> {
    let mut fork = c.fork();
    let len = fork.len();
    while let Some(ch) = fork.peek() {
        match ch {
            b'"' => {
                fork.advance_by(1);
                return Ok(len - fork.len());
            }
            b'\\' => match fork.at(1) {
                Some(b'x') => {
                    if !(matches!(fork.at(2), Some(b'0'..=b'7'))
                        && matches!(fork.at(3), Some(b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f')))
                    {
                        return Err(Diagnostic::new(
                            Level::Error,
                            "Syntax Error: invalid character in numeric character escape",
                        ));
                    }
                    fork.advance_by(4);
                }
                Some(b'n' | b'r' | b't' | b'\\' | b'\'' | b'"' | b'0') => {
                    fork.advance_by(2);
                }
                _ => {
                    return Err(Diagnostic::new(
                        Level::Error,
                        "Syntax Error: unknown character escape",
                    ))
                }
            },
            _ch => {
                fork.advance_by(1);
            }
        }
    }
    Err(Diagnostic::new(
        Level::Error,
        "Syntax Error: Missing trailing `\"` symbol to terminate the string literal",
    ))
}

unsafe fn consume_string(c: &mut Cursor<'_, u8>, len: usize) -> String {
    String::from_utf8_unchecked(c.advance_by(len).to_vec())
}

fn parse_num(mut c: Cursor<'_, u8>, cb: fn(u8) -> bool) -> Result<usize, Diagnostic<()>> {
    let mut len = c.len();
    let mut has_digit = false;

    while let Some(&ch) = c.peek() {
        let is_valid_digit = cb(ch);
        if !(is_valid_digit || ch == b'_') {
            break;
        }
        c.advance_by(1);
        if !has_digit {
            has_digit = is_valid_digit;
        }
    }
    len = len - c.len();

    let amt = c.fork().take_while_and_count(is_ident_continue);
    if amt != 0 {
        let suffix = unsafe { consume_string(&mut c, amt) };
        return Err(Diagnostic::new(
            Level::Error,
            format!("Syntax Error: invalid suffix `{suffix}` for number literal"),
        ));
    }
    if !has_digit {
        return Err(Diagnostic::new(
            Level::Error,
            "Syntax Error: Missing digits after the integer base prefix",
        ));
    }
    Ok(len)
}

fn is_punct(ch: u8) -> bool {
    matches!(ch, b'!'..=b'&' | b'*'..=b'/' | b':'..=b'@' | b'^' | b'|' | b'~')
}
fn is_ident_start(ch: u8) -> bool {
    matches!(ch, b'A'..=b'Z' | b'_' | b'a'..=b'z')
}
fn is_ident_continue(ch: &u8) -> bool {
    matches!(ch, b'0'..=b'9'| b'A'..=b'Z' | b'_' | b'a'..=b'z')
}

#[test]
fn tree() {
    println!("{:#?}", token_stream(b"(ad, adw)"));
}

#[test]
fn test_name() {
    println!("{}", b'\0');
}
