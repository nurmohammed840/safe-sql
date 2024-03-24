#[derive(Debug)]
pub struct Cursor<'a, T> {
    pub inner: &'a [T],
}

impl<'a, T> From<&'a [T]> for Cursor<'a, T> {
    fn from(value: &'a [T]) -> Self {
        Cursor { inner: value }
    }
}

impl<'a, T> Clone for Cursor<'a, T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}

impl<'a, T> Cursor<'a, T> {
    pub fn advance_to(&mut self, fork: &Self) {
        self.inner = fork.inner
    }

    pub fn peek(&self) -> Option<&T> {
        self.inner.get(0)
    }

    pub fn advance_by(&mut self, n: usize) -> &[T] {
        let (split, rest) = self.inner.split_at(n);
        self.inner = rest;
        split
    }

    pub fn count_while(&mut self, mut cb: impl FnMut(&T) -> bool) -> usize {
        let len = self.len();
        while let Some(ch) = self.peek() {
            if !cb(ch) {
                break;
            }
            self.inner = unsafe { self.inner.get_unchecked(1..) };
        }
        len - self.len()
    }
}

impl<'a, T> std::ops::Deref for Cursor<'a, T> {
    type Target = &'a [T];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> Iterator for Cursor<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.get(0) {
            Some(s) => {
                self.inner = unsafe { self.inner.get_unchecked(1..) };
                return Some(s);
            }
            None => None,
        }
    }
}

pub trait IterExt<T> {
    fn advance_by_n(&mut self, n: usize) -> &[T];
    fn peek(&self) -> Option<&T>;
}

impl<'a, T> IterExt<T> for std::slice::Iter<'a, T> {
    fn advance_by_n(&mut self, n: usize) -> &[T] {
        let (split, rest) = self.as_slice().split_at(n);
        *self = rest.iter();
        split
    }

    #[inline]
    fn peek(&self) -> Option<&T> {
        self.as_slice().get(0)
    }
}
