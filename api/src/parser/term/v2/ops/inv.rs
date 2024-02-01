use crate::{v2::behavior_traits::ops::inv, Term};

impl inv::InvertMut for Term {
    fn invert_mut(&mut self) {
        // Just delegate to existing impl for now.
        crate::invert::Invert::invert(self);
    }
}

impl inv::ToInverse for Term {
    #[must_use]
    fn to_inverse(&self) -> Self {
        // Just delegate to existing impl for now.
        crate::invert::ToInverse::to_inverse(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Atom;

    #[test]
    fn invert_test() {
        let mut term = term!(Meter, exponent: 1);
        inv::InvertMut::invert_mut(&mut term);
        assert_eq!(term, term!(Meter, exponent: -1));
    }

    #[test]
    fn to_inverse_test() {
        let term = term!(Meter);
        let new_term = inv::ToInverse::to_inverse(&term);
        assert_eq!(new_term, term!(Meter, exponent: -1));
    }
}
