#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Annotation<'input>(pub(crate) &'input str);

impl<'input> Annotation<'input> {
    #[must_use]
    pub const fn new(inner: &'input str) -> Self {
        Self(inner)
    }

    #[must_use]
    pub const fn as_str(&self) -> &'input str {
        self.0
    }
}
