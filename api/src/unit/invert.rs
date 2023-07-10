use super::Unit;
use crate::invert::{Invert, ToInverse};

impl Invert for &mut Unit {
    #[inline]
    fn invert(self) {
        self.terms.invert();
    }
}

#[cfg(feature = "v2")]
impl crate::v2::Invert for Unit {
    type Error = crate::Error;

    fn invert(&mut self) -> Result<(), Self::Error> {
        crate::v2::Invert::invert(&mut self.terms)
    }
}

impl ToInverse for Unit {
    type Output = Self;

    #[inline]
    fn to_inverse(&self) -> Self::Output {
        Self::new(self.terms.to_inverse())
    }
}

#[cfg(feature = "v2")]
impl crate::v2::ToInverse for Unit {
    type Error = crate::Error;

    fn to_inverse(&self) -> Result<Self, Self::Error> {
        Ok(Self::new(crate::v2::ToInverse::to_inverse(&self.terms)?))
    }
}

#[cfg(test)]
mod tests {
    mod invert {
        use crate::{invert::Invert, Unit};
        use std::str::FromStr;

        #[test]
        fn validate_numerator_no_exponent() {
            let mut unit = Unit::from_str("m").unwrap();
            unit.invert();
            assert_eq!(unit, Unit::from_str("m-1").unwrap());
        }

        #[test]
        fn validate_numerator_with_exponent_1() {
            let mut unit = Unit::from_str("m1").unwrap();
            unit.invert();
            assert_eq!(unit, Unit::from_str("m-1").unwrap());
        }

        #[test]
        fn validate_denominator_with_exponent_minus_1() {
            let mut unit = Unit::from_str("m-1").unwrap();
            unit.invert();
            assert_eq!(unit, Unit::from_str("m").unwrap());
        }

        #[test]
        fn validate_numerator_and_denominator() {
            let mut unit = Unit::from_str("m2/s2").unwrap();
            unit.invert();
            assert_eq!(unit, Unit::from_str("s2/m2").unwrap());
        }

        #[test]
        fn validate_numerators_and_denominators_mixed() {
            let mut unit = Unit::from_str("m2/s2.g4/km4/har5").unwrap();
            unit.invert();
            assert_eq!(unit, Unit::from_str("s2.g4.har5/m2.km4").unwrap());
        }
    }

    mod to_inverse {
        use crate::{invert::ToInverse, Unit};
        use std::str::FromStr;

        #[test]
        fn validate_numerator_no_exponent() {
            let unit = Unit::from_str("m").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, Unit::from_str("m-1").unwrap());
        }

        #[test]
        fn validate_numerator_with_exponent_1() {
            let unit = Unit::from_str("m1").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, Unit::from_str("m-1").unwrap());
        }

        #[test]
        fn validate_denominator_with_exponent_minus_1() {
            let unit = Unit::from_str("m-1").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, Unit::from_str("m").unwrap());
        }

        #[test]
        fn validate_numerator_and_denominator() {
            let unit = Unit::from_str("m2/s2").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, Unit::from_str("s2/m2").unwrap());
        }

        #[test]
        fn validate_numerators_and_denominators_mixed() {
            let unit = Unit::from_str("m2/s2.g4/km4/har5").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, Unit::from_str("s2.g4.har5/m2.km4").unwrap());
        }
    }

    mod into_inverse {
        use crate::{invert::IntoInverse, Unit};
        use std::str::FromStr;

        #[test]
        fn validate_numerator_no_exponent() {
            let unit = Unit::from_str("m").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Unit::from_str("m-1").unwrap());
        }

        #[test]
        fn validate_numerator_with_exponent_1() {
            let unit = Unit::from_str("m1").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Unit::from_str("m-1").unwrap());
        }

        #[test]
        fn validate_denominator_with_exponent_minus_1() {
            let unit = Unit::from_str("m-1").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Unit::from_str("m").unwrap());
        }

        #[test]
        fn validate_numerator_and_denominator() {
            let unit = Unit::from_str("m2/s2").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Unit::from_str("s2/m2").unwrap());
        }

        #[test]
        fn validate_numerators_and_denominators_mixed() {
            let unit = Unit::from_str("m2/s2.g4/km4/har5").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Unit::from_str("s2.g4.har5/m2.km4").unwrap());
        }
    }
}
