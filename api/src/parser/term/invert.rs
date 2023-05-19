use super::Term;
use crate::invert::{Invert, ToInverse};

// Term
impl Invert for &mut Term {
    fn invert(self) {
        self.exponent = self.exponent.map_or(Some(-1), |e| match e {
            -1 => None,
            _ => Some(-e),
        });
    }
}

impl ToInverse for Term {
    type Output = Self;

    fn to_inverse(&self) -> Self::Output {
        let mut new_term = self.clone();
        new_term.invert();

        new_term
    }
}

// Vec<Term>
impl Invert for &mut Vec<Term> {
    fn invert(self) {
        for term in self.iter_mut() {
            term.invert();
        }
    }
}

impl ToInverse for Vec<Term> {
    type Output = Self;

    fn to_inverse(&self) -> Self::Output {
        self.iter().map(ToInverse::to_inverse).collect()
    }
}

#[cfg(test)]
mod tests {
    mod term {
        use crate::{
            invert::{Invert, ToInverse},
            Atom, Term,
        };

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
        use crate::{
            invert::{Invert, ToInverse},
            Atom, Prefix, Term,
        };

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
