use super::*;

/// `||`
#[derive(Debug)]
pub struct ConcatOperator(Span);

impl GetSpan for ConcatOperator {
    fn get_span(&self) -> Span {
        self.0
    }
}

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

macro_rules! ast {
    ($($a: ident<$b: ident, $c: ident>)*) => {
        pub mod ast_kind { $(#[derive(Debug, Default)] pub struct $a;)* }
        $(pub type $a = Ast<ast_kind::$a, $b, $c>;)*
    };
}
ast! {
    Factorial<Term, Factor>
    Arithmetic<Factorial, Sign>
    Operand<Arithmetic, ConcatOperator>
    AndExpr<Condition, AndOperator>
    OrExpr<AndExpr, OrOperator>
}

pub struct Ast<N, T, Operator> {
    pub kind: N,
    pub left: T,
    pub right: Option<(Operator, Box<Self>)>,
}

// -------------------------------------------------------------------------------------

macro_rules! parser {
    (@Operator: $($name: ident = $val: literal)*) => {
        $(
            #[derive(Debug)]
            pub struct $name(Ident);
            impl Parse for $name {
                fn parse(input: ParseStream) -> Result<Self> {
                    let message = "invalid token";
                    input.step(|c| {
                        let (v, rest) = c.ident().ok_or(c.error(&message))?;
                        if !v.to_string().eq_ignore_ascii_case($val) {
                            return Err(c.error(message));
                        }
                        Ok((Self(v), rest))
                    })
                }
            }
            impl GetSpan for $name {
                fn get_span(&self) -> Span {
                    self.0.span()
                }
            }
        )*
    };
    (@Symbol: $($name: ident { $($sym: literal => $kind: ident),* })*) => {
        $(
            #[derive(Debug)]
            pub enum $name { $($kind(Span),)* }
            impl Parse for $name {
                fn parse(input: ParseStream) -> Result<Self> {
                    let message = "invalid token";
                    input.step(|c| {
                        let (p1, rest) = c.punct().ok_or(c.error(message))?;
                        let ret = match p1.as_char() {
                            $($sym => Self::$kind(p1.span()),)*
                            _ => return Err(Error::new(p1.span(), message)),
                        };
                        Ok((ret, rest))
                    })
                }
            }
            impl GetSpan for $name {
                fn get_span(&self) -> Span {
                    match self {
                        $(Self::$kind(s) => *s,)*
                    }
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
                ('|', '|') => Ok((Self(p1.span()), rest)),
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
            kind: N::default(),
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
        match &self.right {
            None => self.left.fmt(f),
            Some((operator, right)) => f
                .debug_struct(&format!("{:?}", self.kind))
                .field("left", &self.left)
                .field("operator", &operator)
                .field("right", &right)
                .finish(),
        }
    }
}

impl<N, T: GetSpan, Operator: GetSpan> GetSpan for Ast<N, T, Operator> {
    fn get_span(&self) -> Span {
        match &self.right {
            Some((o, _)) => o.get_span(),
            None => self.left.get_span(),
        }
    }
}

impl GetSpan for Condition {
    fn get_span(&self) -> Span {
        match self {
            Condition::Operand { left, right } => match right {
                Some(rhs) => match rhs {
                    RightHandSide::Comparison(_, right) => right.get_span(),
                },
                None => left.get_span(),
            },
            Condition::Not(me) => me.get_span(),
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
