use super::Term;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub struct MainTerm<'input> {
    pub(crate) leading_slash: bool,
    pub(crate) term: Term<'input>,
}

impl<'input> MainTerm<'input> {
    #[must_use]
    pub const fn leading_slash(&self) -> bool {
        self.leading_slash
    }

    #[must_use]
    pub const fn term_ref(&self) -> &Term<'input> {
        &self.term
    }

    #[must_use]
    pub fn into_term(self) -> Term<'input> {
        self.term
    }
}
