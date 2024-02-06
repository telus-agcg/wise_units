use num_traits::Inv;

use crate::Term;

impl Inv for Term {
    type Output = Self;

    fn inv(self) -> Self::Output {
        let mut s = self;

        s.exponent = s.exponent.map_or(Some(-1), |e| match e {
            -1 => None,
            _ => Some(-e),
        });

        s
    }
}

impl<'a> Inv for &'a Term {
    type Output = Term;

    fn inv(self) -> Self::Output {
        let new_term = self.clone();
        new_term.inv()
    }
}

impl<'a> Inv for &'a mut Term {
    type Output = Self;

    fn inv(self) -> Self::Output {
        self.exponent = self.exponent.map_or(Some(-1), |e| match e {
            -1 => None,
            _ => Some(-e),
        });

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{Atom, Term};

    use super::*;

    macro_rules! test_inv {
        ($test_name:ident, $subject:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                // Test &mut Term impl
                let mut mut_borrowed = $subject.clone();
                let _ = Inv::inv(&mut mut_borrowed);
                assert_eq!(mut_borrowed, $expected);

                // Test Term impl
                let owned = $subject;
                let inverted = Inv::inv(owned);
                assert_eq!(inverted, $expected);
            }
        };
    }

    test_inv!(
        validate_invert_numerator_no_exponent,
        term!(Meter),
        term!(Meter, exponent: -1)
    );

    test_inv!(
        validate_invert_numerator_with_exponent_1,
        term!(Meter, exponent: 1),
        term!(Meter, exponent: -1)
    );

    test_inv!(
        validate_invert_numerator_with_exponent_minus_1,
        term!(Meter, exponent: -1),
        term!(Meter)
    );
}
