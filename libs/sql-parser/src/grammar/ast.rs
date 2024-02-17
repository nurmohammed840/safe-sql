use super::*;

/// `||`
#[derive(Debug)]
pub struct ConcatOperator;

#[derive(Debug)]
pub enum Compare {
    /// `=`
    Equal,
    /// `!=`
    /// The operator `!=`` is the same as `<>`
    NotEqual,
    /// `<`
    LessThan,
    /// `<=`
    LessThanOrEqual,
    /// `>`
    GreaterThan,
    /// `>=`
    GreaterThanOrEqual,
}

#[derive(Debug)]
pub enum RightHandSide {
    Comparison(Compare, Operand),
    Quantified
}

impl Parse for RightHandSide {
    fn parse(input: ParseStream) -> Result<Self> {
        let compare: Compare = input.parse()?;
        Ok(Self::Comparison(compare, input.parse()?))
    }
}

#[derive(Debug)]
pub enum Condition {
    Operand {
        left: Operand,
        right: Option<RightHandSide>,
    },
    Not(Box<Condition>),
}

#[derive(Debug)]
pub struct Ast<T, Operator> {
    pub left: T,
    pub right: Option<(Operator, Box<Self>)>,
}

pub type Factorial = Ast<Term, Factor>;
pub type Arithmetic = Ast<Factorial, Sign>;
pub type Operand = Ast<Arithmetic, ConcatOperator>;
pub type And = Ast<Condition, AndOperator>;
pub type Expression = Ast<And, OrOperator>;

// -------------------------------------------------------------------------------------

macro_rules! parser {
    (@Operator: $($name: ident = $val: literal)*) => {
        $(
            #[derive(Debug)]
            pub struct $name;
            impl Parse for $name {
                fn parse(input: ParseStream) -> Result<Self> {
                    let message = "invalid token";
                    input.step(|c| {
                        let (v, rest) = c.ident().ok_or(c.error(&message))?;
                        if !v.to_string().eq_ignore_ascii_case($val) {
                            return Err(c.error(message));
                        }
                        Ok((Self, rest))
                    })
                }
            }
        )*
    };
    (@Symbol: $($name: ident { $($sym: literal => $kind: ident),* })*) => {
        $(
            #[derive(Debug)]
            pub enum $name { $($kind,)* }
            impl Parse for $name {
                fn parse(input: ParseStream) -> Result<Self> {
                    let message = "invalid token";
                    input.step(|c| {
                        let (p1, rest) = c.punct().ok_or(c.error(message))?;
                        let ret = match p1.as_char() {
                            $($sym => Self::$kind,)*
                            _ => return Err(Error::new(p1.span(), message)),
                        };
                        Ok((ret, rest))
                    })
                }
            }
        )*
    };
}

parser!(@Operator:
    OrOperator = "OR"
    AndOperator = "AND"
);
parser!(@Symbol: 
    Factor {
        '*' => Multiply,
        '/' => Divide,
        '%' => Modulo
    }
    Sign {
        '+' => Plus,
        '-' => Minus
    }
);


impl Parse for ConcatOperator {
    fn parse(input: ParseStream) -> Result<Self> {
        let message = "expected operator: `||`";
        input.step(|c| {
            let (p1, rest) = c.punct().ok_or(c.error(message))?;
            let (p2, rest) = rest.punct().ok_or(c.error(message))?;
            match (p1.as_char(), p2.as_char()) {
                ('|', '|') => Ok((Self, rest)),
                _ => Err(Error::new(p1.span(), message)),
            }
        })
    }
}

impl Parse for Compare {
    fn parse(input: ParseStream) -> Result<Self> {
        input.step(|c| {
            let (p1, rest) = c
                .punct()
                .ok_or(c.error("expected operator: `=`, `<`, `>`, `!=`, `<=`, `>=`"))?;

            Ok(match rest.punct() {
                None => match p1.as_char() {
                    '=' => (Self::Equal, rest),
                    '<' => (Self::LessThan, rest),
                    '>' => (Self::GreaterThan, rest),
                    ch => return Err(c.error(format!("invalid operator: {ch}"))),
                },
                Some((p2, rest)) => match (p1.as_char(), p2.as_char()) {
                    ('<', '=') => (Self::LessThan, rest),
                    ('>', '=') => (Self::GreaterThanOrEqual, rest),
                    ('!', '=') | ('<', '>') => (Self::NotEqual, rest),
                    (ch1, ch2) => return Err(c.error(format!("invalid operator: {ch1}{ch2}"))),
                },
            })
        })
    }
}

impl Parse for Condition {
    fn parse(input: ParseStream) -> Result<Self> {
        enum Kind {
            Not,
        }
        let kind = input.cursor().ident().and_then(|(s, _)| {
            if s.to_string().eq_ignore_ascii_case("NOT") {
                return Some(Kind::Not);
            }
            None
        });
        let this = match kind {
            None => Self::Operand {
                left: input.parse()?,
                right: input.parse().ok(),
            },
            Some(kind) => {
                input.parse::<Ident>()?;
                match kind {
                    Kind::Not => Self::Not(input.parse()?),
                }
            }
        };
        Ok(this)
    }
}

impl<T: Parse, O: Parse> Parse for Ast<T, O> {
    fn parse(input: ParseStream) -> Result<Self> {
        let left = input.parse()?;
        Ok(Self {
            left,
            right: if let Ok(o) = O::parse(input) {
                Some((o, input.parse()?))
            } else {
                None
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let g: Expression = utils::test::syntex! {
            //  1 / c + b + 2 * 3 % 4 + a || ad
             dad 
        }
        .unwrap();

        println!("{:#?}", g);
    }
}
