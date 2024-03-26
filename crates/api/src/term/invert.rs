// TODO: Remove in 1.0.0 release.
#![allow(deprecated)]

use std::borrow::Cow;

use num_traits::Inv;

use crate::invert::{Invert, ToInverse};

use super::Term;

// ╭──────╮
// │ Term │
// ╰──────╯
impl Invert for &mut Term {
    fn invert(self) {
        let _ = Inv::inv(self);
    }
}

impl ToInverse for Term {
    type Output = Self;

    fn to_inverse(&self) -> Self::Output {
        Inv::inv(self)
    }
}

// ╭───────────╮
// │ Vec<Term> │
// ╰───────────╯
impl Invert for &mut Vec<Term> {
    fn invert(self) {
        for term in self.iter_mut() {
            let _ = Inv::inv(term);
        }
    }
}

impl ToInverse for Vec<Term> {
    type Output = Self;

    fn to_inverse(&self) -> Self::Output {
        self.iter().map(Inv::inv).collect()
    }
}

// ╭─────────────────╮
// │ Cow<'a, [Term]> │
// ╰─────────────────╯
impl<'a> Invert for &mut Cow<'a, [Term]> {
    fn invert(self) {
        for term in self.to_mut().iter_mut() {
            let _ = term.inv();
        }
    }
}

impl<'a> ToInverse for Cow<'a, [Term]> {
    type Output = Self;

    fn to_inverse(&self) -> Self::Output {
        self.iter().map(Inv::inv).collect()
    }
}

#[cfg(test)]
mod tests {
    mod term {
        use crate::invert::{Invert, ToInverse};

        #[test]
        fn validate_invert_numerator_no_exponent() {
            let mut term = term!(Meter);
            term.invert();
            assert_eq!(term, term!(Meter, exponent: -1));
        }

        #[test]
        fn validate_invert_numerator_with_exponent_1() {
            let mut term = term!(Meter, exponent: 1);
            term.invert();
            assert_eq!(term, term!(Meter, exponent: -1));
        }

        #[test]
        fn validate_invert_denominator_with_exponent_minus_1() {
            let mut term = term!(Meter, exponent: -1);
            term.invert();
            assert_eq!(term, term!(Meter));
        }

        #[test]
        fn validate_to_inverse_numerator_no_exponent() {
            let term = term!(Meter);
            let new_term = term.to_inverse();
            assert_eq!(new_term, term!(Meter, exponent: -1));
        }

        #[test]
        fn validate_to_inverse_numerator_with_exponent() {
            let term = term!(Meter, exponent: 1);
            let new_term = term.to_inverse();
            assert_eq!(new_term, term!(Meter, exponent: -1));
        }

        #[test]
        fn validate_to_inverse_denominator_with_exponent_minus_1() {
            let term = term!(Meter, exponent: -1);
            let new_term = term.to_inverse();
            assert_eq!(new_term, term!(Meter));
        }
    }

    mod terms {
        use crate::invert::{Invert, ToInverse};

        #[test]
        fn validate_invert_numerator_no_exponent() {
            let mut terms = vec![term!(Meter)];
            terms.invert();
            assert_eq!(terms, vec![term!(Meter, exponent: -1)]);
        }

        #[test]
        fn validate_invert_numerator_with_exponent_1() {
            let mut terms = vec![term!(Meter, exponent: 1)];
            terms.invert();
            assert_eq!(terms, vec![term!(Meter, exponent: -1)]);
        }

        #[test]
        fn validate_invert_denominator_with_exponent_minus_1() {
            let mut terms = vec![term!(Meter, exponent: -1)];
            terms.invert();
            assert_eq!(terms, vec![term!(Meter)]);
        }

        #[test]
        fn validate_invert_numerator_and_denominator() {
            let mut terms = vec![term!(Meter, exponent: 2), term!(Second, exponent: -2)];
            terms.invert();
            assert_eq!(
                terms,
                vec![term!(Meter, exponent: -2), term!(Second, exponent: 2)]
            );
        }

        #[test]
        fn validate_invert_numerators_and_denominators_mixed() {
            let mut terms = vec![
                term!(Meter, exponent: 2),
                term!(Second, exponent: -2),
                term!(Gram, exponent: -4),
                term!(Kilo, Meter, exponent: 4),
                term!(Hecto, Are, exponent: -5),
            ];
            terms.invert();

            let result = vec![
                term!(Meter, exponent: -2),
                term!(Second, exponent: 2),
                term!(Gram, exponent: 4),
                term!(Kilo, Meter, exponent: -4),
                term!(Hecto, Are, exponent: 5),
            ];
            assert_eq!(terms, result);
        }

        #[test]
        fn validate_to_inverse_numerator_no_exponent() {
            let terms = vec![term!(Meter)];
            let new_terms = terms.to_inverse();
            assert_eq!(new_terms, vec![term!(Meter, exponent: -1)]);
        }

        #[test]
        fn validate_to_inverse_numerator_with_exponent_1() {
            let terms = vec![term!(Meter, exponent: 1)];
            let new_terms = terms.to_inverse();
            assert_eq!(new_terms, vec![term!(Meter, exponent: -1)]);
        }

        #[test]
        fn validate_to_inverse_denominator_with_exponent_minus_1() {
            let terms = vec![term!(Meter, exponent: -1)];
            let new_terms = terms.to_inverse();
            assert_eq!(new_terms, vec![term!(Meter)]);
        }

        #[test]
        fn validate_to_inverse_numerator_and_denominator() {
            let terms = vec![term!(Meter, exponent: 2), term!(Second, exponent: -2)];
            let new_terms = terms.to_inverse();
            assert_eq!(
                new_terms,
                vec![term!(Meter, exponent: -2), term!(Second, exponent: 2)]
            );
        }

        #[test]
        fn validate_to_inverse_numerators_and_denominators_mixed() {
            let terms = vec![
                term!(Meter, exponent: 2),
                term!(Second, exponent: -2),
                term!(Gram, exponent: -4),
                term!(Kilo, Meter, exponent: 4),
                term!(Hecto, Are, exponent: -5),
            ];
            let new_terms = terms.to_inverse();

            let result = vec![
                term!(Meter, exponent: -2),
                term!(Second, exponent: 2),
                term!(Gram, exponent: 4),
                term!(Kilo, Meter, exponent: -4),
                term!(Hecto, Are, exponent: 5),
            ];
            assert_eq!(new_terms, result);
        }
    }
}
