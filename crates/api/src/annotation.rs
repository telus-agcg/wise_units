use std::fmt;

use crate::Term;

#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Annotation(String);

impl Annotation {
    pub(crate) const fn new(inner: String) -> Self {
        Self(inner)
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Annotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{annotation}}}", annotation = self.0)
    }
}

impl From<Annotation> for Term {
    fn from(value: Annotation) -> Self {
        Self::Annotation(value)
    }
}

impl<'a, T> From<&'a T> for Annotation
where
    T: ToString + ?Sized,
{
    fn from(value: &'a T) -> Self {
        Self(value.to_string())
    }
}
