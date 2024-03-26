use std::{
    fmt::{self, Debug},
    slice::Iter,
};
pub mod playground;

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

#[derive(Debug, Clone, Copy, PartialEq)]
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

pub trait Group: Debug {
    type Span: Clone;
    fn span(&self) -> Self::Span;
    fn span_open(&self) -> Self::Span;
    fn span_close(&self) -> Self::Span;
    fn delimiter(&self) -> Delimiter;
    fn stream(&self) -> &[TokenTree<Self::Span>];
}

pub enum TokenTree<Span> {
    Group(Box<dyn Group<Span = Span>>),
    Punct {
        span: Span,
        char: char,
        spacing: Spacing,
    },
    Ident(Span),
    Literal(Span),
}

// impl<Span> TokenTree<Span> {
//     pub fn print(&self, ctx: &impl Contex<Span>) {
//         // match self {
//         //     TokenTree::Group(_) => todo!(),
//         //     TokenTree::Punct { span, char, spacing } => todo!(),
//         //     TokenTree::Ident(_) => todo!(),
//         //     TokenTree::Literal(_) => todo!(),
//         // }
//     }
// }

pub trait Contex<Span> {
    fn text(&self, span: Span) -> &str;
}

pub trait Parse<Span>: Sized {
    fn parse(
        ctx: &impl Contex<Span>,
        cursor: &mut Iter<TokenTree<Span>>,
    ) -> Result<Self, Diagnostic<Span>>;
}

impl<Span: fmt::Debug> fmt::Debug for TokenTree<Span> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Group(arg0) => arg0.fmt(f),
            Self::Ident(arg0) => arg0.fmt(f),
            Self::Punct { char, .. } => char.fmt(f),
            Self::Literal(arg0) => arg0.fmt(f),
        }
    }
}

pub trait Cursor<T> {
    fn advance(&mut self, n: usize) -> &[T];
    fn peek(&self) -> Option<&T>;
}

impl<'a, T> Cursor<T> for std::slice::Iter<'a, T> {
    fn advance(&mut self, n: usize) -> &[T] {
        let (split, rest) = self.as_slice().split_at(n);
        *self = rest.iter();
        split
    }

    #[inline]
    fn peek(&self) -> Option<&T> {
        self.as_slice().get(0)
    }
}
