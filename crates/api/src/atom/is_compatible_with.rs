use crate::{Atom, Composable, Term};

impl crate::IsCompatibleWith for Atom {}

impl crate::IsCompatibleWith<Term> for Atom {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition() && rhs.annotation().is_none()
    }
}
