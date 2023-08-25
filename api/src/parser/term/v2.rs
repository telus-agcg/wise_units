use crate::{
    v2::traits::{convert, ops, term::Term as V2Term, ucum},
    Term,
};

impl V2Term for Term {
    type Prefix = crate::Prefix;
    type Atom = crate::Atom;
    type Annotation = String;

    fn factor(&self) -> Option<u32> {
        self.factor
    }

    fn prefix_symbol(&self) -> Option<Self::Prefix> {
        self.prefix
    }

    fn atom_symbol(&self) -> Option<Self::Atom> {
        self.atom
    }

    fn exponent(&self) -> Option<i32> {
        self.exponent
    }

    fn exponent_mut(&mut self) -> &mut Option<i32> {
        &mut self.exponent
    }

    fn annotation(&self) -> &Option<Self::Annotation> {
        &self.annotation
    }
}

impl convert::Invert for Term {
    fn invert(&mut self) {
        // Just delegate to existing impl for now.
        crate::invert::Invert::invert(self);
    }
}

impl convert::ToInverse for Term {
    #[must_use]
    fn to_inverse(&self) -> Self {
        // Just delegate to existing impl for now.
        crate::invert::ToInverse::to_inverse(self)
    }
}

impl convert::ToScalar<f64> for Term {
    fn to_scalar(&self) -> f64 {
        // Just delegate to existing impl for now.
        crate::UcumUnit::scalar(self)
    }
}

impl convert::ToMagnitude<f64> for Term {
    fn to_magnitude(&self) -> f64 {
        // Just delegate to existing impl for now.
        crate::UcumUnit::magnitude(self)
    }
}

impl ops::IsCommensurableWith<Self> for Term {
    fn is_commensurable_with(&self, rhs: &Self) -> bool {
        // TODO: Delegating, but the current implementation is incorrect (it should behave how the
        // current FieldEq implementation behaves.).
        PartialEq::eq(self, rhs)
    }
}

impl ops::FieldEq<Self> for Term {
    fn field_eq(&self, rhs: &Self) -> bool {
        crate::FieldEq::field_eq(self, rhs)
    }
}

impl ucum::Dim<crate::Composition> for Term {
    fn dim(&self) -> crate::Composition {
        // Just delegate to existing impl for now.
        crate::Composable::composition(self)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;

    use super::*;
    use crate::{
        parser::{Atom, Prefix},
        v2::traits::{
            convert::{Invert, ToInverse},
            ucum::Dim,
        },
    };

    #[test]
    fn invert_test() {
        let mut term = term!(Meter, exponent: 1);
        term.invert();
        assert_eq!(term, term!(Meter, exponent: -1));
    }

    #[test]
    fn to_inverse_test() {
        let term = term!(Meter);
        let new_term = term.to_inverse();
        assert_eq!(new_term, term!(Meter, exponent: -1));
    }

    #[test]
    fn to_scalar_test() {
        use convert::ToScalar;

        let term = term!(Kilo, Meter);
        assert_ulps_eq!(1000.0, term.to_scalar());
    }

    #[test]
    fn is_compatible_with_test() {
        let lhs = term!(Meter);
        let rhs = term!(Kilo, Meter);
        assert!(V2Term::is_compatible_with(&lhs, &rhs));

        let m = term!(Meter, annotation: "stuff".to_string());
        let km_stuff = term!(Kilo, Meter, annotation: "stuff".to_string());
        assert!(V2Term::is_compatible_with(&m, &km_stuff));
    }

    #[test]
    fn is_commensurable_with_test() {
        let lhs = term!(Meter, factor: 1000);
        let rhs = term!(Kilo, Meter);
        assert!(V2Term::is_commensurable_with(&lhs, &rhs));

        let lhs = term!(Meter);
        assert!(!V2Term::is_commensurable_with(&lhs, &rhs));

        let m = term!(Meter, factor: 1000, annotation: "stuff".to_string());
        let km_stuff = term!(Kilo, Meter, annotation: "stuff".to_string());
        assert!(V2Term::is_commensurable_with(&m, &km_stuff));

        let m = term!(Meter, factor: 1000, annotation: "123".to_string());
        let km_stuff = term!(Kilo, Meter, annotation: "456".to_string());
        assert!(!V2Term::is_commensurable_with(&m, &km_stuff));
    }

    #[test]
    fn equals_test() {
        let lhs = term!(Meter, factor: 1000);
        let rhs = term!(Kilo, Meter);
        assert!(!V2Term::equals(&lhs, &rhs));
        assert!(V2Term::equals(&lhs, &lhs));
    }

    #[test]
    fn dim_test() {
        let term = term!(Meter);

        assert_eq!(
            term.dim(),
            crate::Composition::new(crate::Dimension::Length, 1)
        );
    }
}
