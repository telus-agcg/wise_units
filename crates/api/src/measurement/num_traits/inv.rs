use num_traits::Inv;

use crate::Measurement;

impl Inv for Measurement {
    type Output = Self;

    fn inv(self) -> Self::Output {
        let mut s = self;
        s.value = s.value.inv();
        s.unit = s.unit.inv();
        s
    }
}

impl<'a> Inv for &'a mut Measurement {
    type Output = Self;

    fn inv(self) -> Self::Output {
        self.value = self.value.inv();
        let _ = Inv::inv(&mut self.unit);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Measurement;
    use num_traits::Inv;

    macro_rules! test_inv {
        ($test_name:ident, $subject:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                // Test &Measurement impl
                // let new_measurement = Inv::inv(&$subject);
                // assert_eq!(new_measurement, $expected);

                // Test &mut Measurement impl
                let mut mut_borrowed = $subject.clone();
                let _ = Inv::inv(&mut mut_borrowed);
                assert_eq!(mut_borrowed, $expected);

                // Test Measurement impl
                let owned = $subject;
                let inverted = Inv::inv(owned);
                assert_eq!(inverted, $expected);
            }
        };
    }

    test_inv!(
        validate_numerator_no_exponent,
        Measurement::try_new(10.0, "m").unwrap(),
        Measurement::try_new(0.1, "m-1").unwrap()
    );

    test_inv!(
        validate_numerator_with_exponent_1,
        Measurement::try_new(10.0, "m1").unwrap(),
        Measurement::try_new(0.1, "m-1").unwrap()
    );

    test_inv!(
        validate_numerator_with_exponent_minus_1,
        Measurement::try_new(10.0, "m-1").unwrap(),
        Measurement::try_new(0.1, "m").unwrap()
    );

    test_inv!(
        validate_numerator_and_denominator,
        Measurement::try_new(10.0, "m2/s2").unwrap(),
        Measurement::try_new(0.1, "s2/m2").unwrap()
    );

    test_inv!(
        validate_numerator_and_denominator_mixed,
        Measurement::try_new(10.0, "m2/s2.g4/km4/har5").unwrap(),
        Measurement::try_new(0.1, "s2.g4.har5/m2.km4").unwrap()
    );
}
