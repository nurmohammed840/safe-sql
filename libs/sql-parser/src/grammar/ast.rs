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
    Quantified,
}

impl Parse for RightHandSide {
    fn parse(input: ParseStream) -> Result<Self> {
        let compare: Compare = input.parse()?;
        Ok(Self::Comparison(compare, input.parse()?))
    }
}

pub enum Condition {
    Operand {
        left: Operand,
        right: Option<RightHandSide>,
    },
    Not(Box<Condition>),
}

mod ast_name {
    #[derive(Debug, Default)]
    pub struct Factorial;
    #[derive(Debug, Default)]
    pub struct Arithmetic;
    #[derive(Debug, Default)]
    pub struct Operand;
    #[derive(Debug, Default)]
    pub struct AndExpr;
    #[derive(Debug, Default)]
    pub struct OrExpr;
}

pub type Factorial = Ast<ast_name::Factorial, Term, Factor>;
pub type Arithmetic = Ast<ast_name::Arithmetic, Factorial, Sign>;
pub type Operand = Ast<ast_name::Operand, Arithmetic, ConcatOperator>;
pub type AndExpr = Ast<ast_name::AndExpr, Condition, AndOperator>;
pub type OrExpr = Ast<ast_name::OrExpr, AndExpr, OrOperator>;

pub struct Ast<N, T, Operator> {
    pub name: N,
    pub left: T,
    pub right: Option<(Operator, Box<Self>)>,
}

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

impl<N: Default, T: Parse, O: Parse> Parse for Ast<N, T, O> {
    fn parse(input: ParseStream) -> Result<Self> {
        let left = input.parse()?;
        Ok(Self {
            name: N::default(),
            left,
            right: if let Ok(o) = O::parse(input) {
                Some((o, input.parse()?))
            } else {
                None
            },
        })
    }
}

impl fmt::Debug for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operand { left, right } => match right {
                None => left.fmt(f),
                Some(rhs) => f
                    .debug_struct("Operand")
                    .field("left", left)
                    .field("right", rhs)
                    .finish(),
            },
            Self::Not(arg0) => f.debug_tuple("Not").field(arg0).finish(),
        }
    }
}

impl<N: fmt::Debug, T: fmt::Debug, Operator: fmt::Debug> fmt::Debug for Ast<N, T, Operator> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.right {
            None => self.left.fmt(f),
            Some(ref right) => f
                .debug_struct(&format!("{:?}", self.name))
                .field("left", &self.left)
                .field("right", &right)
                .finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let g: OrExpr = utils::test::syntex! {
            a and 54 or 4 + 4
        }
        .unwrap();
        println!("{:#?}", g);
    }
}
