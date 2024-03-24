use std::fmt;

pub mod cursor;
pub mod lex;
// pub mod playground;

/// An enum representing a diagnostic level.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Level {
    /// An error.
    Error,
    /// A warning.
    Warning,
    /// A note.
    Note,
    /// A help message.
    Help,
}

/// Trait implemented by types that can be converted into a set of `Span`s.
/// A structure representing a diagnostic message and associated children
/// messages.

#[derive(Clone, Debug)]
pub struct Diagnostic<Span> {
    level: Level,
    message: String,
    spans: Vec<Span>,
    children: Vec<Diagnostic<Span>>,
}

macro_rules! diagnostic_child_methods {
    ($spanned:ident, $regular:ident, $level:expr) => {

        #[doc = concat!("Adds a new child diagnostics message to `self` with the [`", stringify!($level), "`] level, and the given `spans` and `message`.")]
        pub fn $spanned<S, T>(mut self, spans: S, message: T) -> Self
        where
            S: Into<Vec<Span>>,
            T: Into<String>,
        {
            self.children.push(Diagnostic::spanned(spans, $level, message));
            self
        }

        #[doc = concat!("Adds a new child diagnostic message to `self` with the [`", stringify!($level), "`] level, and the given `message`.")]
        pub fn $regular<T: Into<String>>(mut self, message: T) -> Self {
            self.children.push(Diagnostic::new($level, message));
            self
        }
    };
}

impl<Span> Diagnostic<Span> {
    /// Creates a new diagnostic with the given `level` and `message`.
    pub fn new<T: Into<String>>(level: Level, message: T) -> Self {
        Diagnostic {
            level,
            message: message.into(),
            spans: vec![],
            children: vec![],
        }
    }

    /// Creates a new diagnostic with the given `level` and `message` pointing to
    /// the given set of `spans`.

    pub fn spanned<S, T>(spans: S, level: Level, message: T) -> Self
    where
        S: Into<Vec<Span>>,
        T: Into<String>,
    {
        Diagnostic {
            level,
            message: message.into(),
            spans: spans.into(),
            children: vec![],
        }
    }

    diagnostic_child_methods!(span_error, error, Level::Error);
    diagnostic_child_methods!(span_warning, warning, Level::Warning);
    diagnostic_child_methods!(span_note, note, Level::Note);
    diagnostic_child_methods!(span_help, help, Level::Help);

    /// Returns the diagnostic `level` for `self`.
    pub fn level(&self) -> Level {
        self.level
    }

    /// Sets the level in `self` to `level`.
    pub fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    /// Returns the message in `self`.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Sets the message in `self` to `message`.
    pub fn set_message<T: Into<String>>(&mut self, message: T) {
        self.message = message.into();
    }

    /// Returns the `Span`s in `self`.
    pub fn spans(&self) -> &[Span] {
        &self.spans
    }

    /// Sets the `Span`s in `self` to `spans`.
    pub fn set_spans(&mut self, spans: Vec<Span>) {
        self.spans = spans;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Delimiter {
    Parenthesis,
    Brace,
    Bracket,
    None,
}

#[derive(Debug, Clone)]
pub enum Spacing {
    Joint,
    Alone,
}

#[derive(Debug, Clone)]
pub struct Group<Span> {
    pub span: Span,
    pub delimiter: Delimiter,
    pub span_open: Span,
    pub span_close: Span,
    pub stream: Vec<TokenTree<Span>>,
}

#[derive(Debug, Clone)]
pub struct Ident<Span> {
    pub span: Span,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Punct<Span> {
    pub span: Span,
    pub char: char,
    pub spacing: Spacing,
}

#[derive(Debug, Clone)]
pub struct Literal<Span> {
    pub span: Span,
    pub value: String,
}

#[derive(Clone)]
pub enum TokenTree<Span> {
    Group(Group<Span>),
    Ident(Ident<Span>),
    Punct(Punct<Span>),
    Literal(Literal<Span>),
}

impl<Span: fmt::Debug> fmt::Debug for TokenTree<Span> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Group(arg0) => arg0.fmt(f),
            Self::Ident(arg0) => arg0.fmt(f),
            Self::Punct(arg0) => arg0.fmt(f),
            Self::Literal(arg0) => arg0.fmt(f),
        }
    }
}

pub struct Cursor<'a, Span> {
    pub scope: &'a Span,
    pub tokens: cursor::Cursor<'a, TokenTree<Span>>,
}

impl<Span> Cursor<'_, Span>
where
    Span: Clone,
{
    pub fn error<T: fmt::Display>(&self, message: T) -> Diagnostic<Span> {
        Diagnostic::spanned(
            vec![self.scope.clone()],
            Level::Error,
            if self.tokens.is_empty() {
                format!("unexpected end of input, {}", message)
            } else {
                message.to_string()
            },
        )
    }
}

impl<Span> Cursor<'_, Span> {
    pub fn fork(&self) -> Self {
        Self {
            scope: self.scope,
            tokens: self.tokens.clone(),
        }
    }

    pub fn advance_to(&mut self, fork: &Self) {
        self.tokens.advance_to(&fork.tokens);
    }

    pub fn peek(&self) -> Option<&TokenTree<Span>> {
        self.tokens.peek()
    }

    pub fn parse<T: Parse<Span>>(&mut self) -> Result<T, Diagnostic<Span>> {
        T::parse(self)
    }
}

impl<'a, Span> Iterator for Cursor<'a, Span> {
    type Item = &'a TokenTree<Span>;
    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next()
    }
}

pub trait Parse<Span>: Sized {
    fn parse(cursor: &mut Cursor<Span>) -> Result<Self, Diagnostic<Span>>;
}
