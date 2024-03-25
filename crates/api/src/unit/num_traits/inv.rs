use std::borrow::Cow;

use num_traits::Inv;

use crate::{term, Unit};

impl Inv for Unit {
    type Output = Self;

    fn inv(self) -> Self::Output {
        Self {
            terms: Cow::Owned(term::num_traits::inv::inv_terms_into(self.terms.to_vec())),
        }
    }
}

impl<'a> Inv for &'a Unit {
    type Output = Unit;

    fn inv(self) -> Self::Output {
        Unit {
            terms: Cow::Owned(term::num_traits::inv::inv_terms_into(self.terms.to_vec())),
        }
    }
}

impl<'a> Inv for &'a mut Unit {
    type Output = Self;

    fn inv(self) -> Self::Output {
        term::num_traits::inv::inv_terms(self.terms.to_mut());

        self
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    macro_rules! test_inv {
        ($test_name:ident, $subject:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                // Test &mut Unit impl
                let mut mut_borrowed = $subject.clone();
                let _ = Inv::inv(&mut mut_borrowed);
                assert_eq!(mut_borrowed, $expected);

                // Test &Unit impl
                let owned = $subject;
                let inverted = Inv::inv(&owned);
                assert_eq!(inverted, $expected);

                // Test Unit impl
                let owned = $subject;
                let inverted = Inv::inv(owned);
                assert_eq!(inverted, $expected);
            }
        };
    }

    test_inv!(
        validate_numerator_no_exponent,
        Unit::from_str("m").unwrap(),
        Unit::from_str("m-1").unwrap()
    );

    test_inv!(
        validate_numerator_with_exponent_1,
        Unit::from_str("m1").unwrap(),
        Unit::from_str("m-1").unwrap()
    );

    test_inv!(
        validate_numerator_with_exponent_minus_1,
        Unit::from_str("m-1").unwrap(),
        Unit::from_str("m").unwrap()
    );

    test_inv!(
        validate_numerator_and_denominator,
        Unit::from_str("m2/s2").unwrap(),
        Unit::from_str("s2/m2").unwrap()
    );

    test_inv!(
        validate_numerator_and_denominator_mixed,
        Unit::from_str("m2/s2.g4/km4/har5").unwrap(),
        Unit::from_str("s2.g4.har5/m2.km4").unwrap()
    );
}
