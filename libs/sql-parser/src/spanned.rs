use proc_macro2::Span;
use std::ops::Deref;

pub trait GetSpan {
    fn get_span(&self) -> Span;
}

pub struct WithSpan<T> {
    pub span: Span,
    pub value: T,
}

impl<T> WithSpan<T> {
    pub fn new(span: Span, value: T) -> Self {
        Self { span, value }
    }
}

impl<T> Deref for WithSpan<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> GetSpan for WithSpan<T> {
    fn get_span(&self) -> Span {
        self.span
    }
}


impl<T: GetSpan> GetSpan for Box<T> {
    fn get_span(&self) -> Span {
        T::get_span(&self)
    }
}

impl<T: GetSpan> GetSpan for &T {
    fn get_span(&self) -> Span {
        T::get_span(&self)
    }
}
