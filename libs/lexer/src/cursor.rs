#[derive(Debug, Clone)]
pub struct Cursor<'a, T> {
    pub inner: &'a [T],
}

impl<'a, T> From<&'a [T]> for Cursor<'a, T> {
    fn from(value: &'a [T]) -> Self {
        Cursor { inner: value }
    }
}

impl<'a, T> Cursor<'a, T> {
    pub fn fork(&self) -> Self {
        Self { inner: self.inner }
    }

    pub fn advance_to(&mut self, fork: &Self) {
        self.inner = fork.inner
    }

    pub fn peek(&self) -> Option<&T> {
        self.inner.get(0)
    }

    pub fn peek_nth(&self, n: usize) -> Option<&T> {
        self.inner.get(n)
    }

    pub fn advance_by(&mut self, n: usize) -> &[T] {
        let (split, rest) = self.inner.split_at(n);
        self.inner = rest;
        split
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