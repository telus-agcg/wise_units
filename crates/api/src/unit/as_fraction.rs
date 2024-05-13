use num_traits::Inv;

use crate::{as_fraction::AsFraction, Term, Unit};

impl AsFraction for Unit {
    type Numerator = Option<Self>;
    type Denominator = Option<Self>;

    #[inline]
    fn numerator(&self) -> Self::Numerator {
        let positive_terms: Vec<Term> = self
            .terms
            .iter()
            .filter(|term| term.exponent.unwrap_or(1).is_positive())
            .cloned()
            .collect();

        if positive_terms.is_empty() {
            None
        } else {
            Some(Self::new(positive_terms))
        }
    }

    #[inline]
    fn denominator(&self) -> Self::Denominator {
        let negative_terms: Vec<Term> = self
            .terms
            .iter()
            .filter_map(|term| match term.exponent {
                Some(e) if e.is_negative() => Some(term.inv()),
                _ => None,
            })
            .collect();

        if negative_terms.is_empty() {
            None
        } else {
            Some(Self::new(negative_terms))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        as_fraction::AsFraction,
        testing::const_units::{GRAM_METER, METER, METER_PER_SECOND, PER_SECOND},
    };

    use super::Unit;

    #[test]
    fn validate_as_fraction() {
        let (num, den) = Unit::from_str("m/s").unwrap().as_fraction();

        assert!(num.is_some());
        assert!(den.is_some());
    }

    mod numerator {
        use super::*;

        #[test]
        fn validate_one_numerator_term() {
            let numerator = METER.numerator().unwrap();
            assert_eq!(numerator, METER);
        }

        #[test]
        fn validate_two_numerator_terms() {
            let numerator = GRAM_METER.numerator().unwrap();
            assert_eq!(numerator, GRAM_METER);
        }

        #[test]
        fn validate_one_numerator_term_one_denominator_term() {
            let numerator = METER_PER_SECOND.numerator().unwrap();
            assert_eq!(numerator, METER);
        }

        #[test]
        fn validate_one_denominator_term() {
            let numerator = PER_SECOND.numerator();
            assert!(numerator.is_none());
        }
    }

    mod denominator {
        use crate::testing::const_units::{PER_GRAM_METER, SECOND};

        use super::*;

        #[test]
        fn validate_one_numerator_term() {
            let denominator = METER.denominator();
            assert!(denominator.is_none());
        }

        #[test]
        fn validate_two_numerator_terms() {
            let denominator = GRAM_METER.denominator();
            assert!(denominator.is_none());
        }

        #[test]
        fn validate_one_numerator_term_one_denominator_term() {
            let denominator = METER_PER_SECOND.denominator().unwrap();
            assert_eq!(denominator, SECOND);
        }

        #[test]
        fn validate_one_denominator_term() {
            let denominator = PER_SECOND.denominator().unwrap();
            assert_eq!(denominator, SECOND);
        }

        #[test]
        fn validate_two_denominator_terms() {
            let denominator = PER_GRAM_METER.denominator().unwrap();
            assert_eq!(denominator, GRAM_METER);
        }
    }
}
