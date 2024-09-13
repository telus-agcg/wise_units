use crate::{
    as_fraction::AsFraction,
    v2::convert::{ToFraction, ToReduced},
    Unit,
};

impl ToFraction for Unit {
    fn to_fraction(&self) -> (Option<Self>, Option<Self>) {
        // Just delegate to the old trait impl for now.
        AsFraction::as_fraction(self)
    }

    fn to_numerator(&self) -> Option<Self> {
        // Just delegate to the old trait impl for now.
        AsFraction::numerator(self)
    }

    fn to_denominator(&self) -> Option<Self> {
        // Just delegate to the old trait impl for now.
        AsFraction::denominator(self)
    }
}

impl ToReduced for Unit {
    fn to_reduced(&self) -> Self {
        // Just delegate to the old trait impl for now.
        crate::reduce::ToReduced::to_reduced(self)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        testing::const_units::{
            l1::METER, l1m1::GRAM_METER, l1t_1::METER_PER_SECOND, t_1::PER_SECOND,
        },
        Unit,
    };

    use super::ToFraction;

    #[test]
    fn validate_to_fraction() {
        let (num, den) = Unit::from_str("m/s").unwrap().to_fraction();

        assert!(num.is_some());
        assert!(den.is_some());
    }

    mod numerator {
        use super::*;

        #[test]
        fn validate_one_numerator_term() {
            let numerator = METER.to_numerator().unwrap();
            assert_eq!(numerator, METER);
        }

        #[test]
        fn validate_two_numerator_terms() {
            let numerator = GRAM_METER.to_numerator().unwrap();
            assert_eq!(numerator, GRAM_METER);
        }

        #[test]
        fn validate_one_numerator_term_one_denominator_term() {
            let numerator = METER_PER_SECOND.to_numerator().unwrap();
            assert_eq!(numerator, METER);
        }

        #[test]
        fn validate_one_denominator_term() {
            let numerator = PER_SECOND.to_numerator();
            assert!(numerator.is_none());
        }
    }

    mod denominator {
        use crate::testing::const_units::{l_1m_1::PER_GRAM_METER, t1::SECOND};

        use super::*;

        #[test]
        fn validate_one_numerator_term() {
            let denominator = METER.to_denominator();
            assert!(denominator.is_none());
        }

        #[test]
        fn validate_two_numerator_terms() {
            let denominator = GRAM_METER.to_denominator();
            assert!(denominator.is_none());
        }

        #[test]
        fn validate_one_numerator_term_one_denominator_term() {
            let denominator = METER_PER_SECOND.to_denominator().unwrap();
            assert_eq!(denominator, SECOND);
        }

        #[test]
        fn validate_one_denominator_term() {
            let denominator = PER_SECOND.to_denominator().unwrap();
            assert_eq!(denominator, SECOND);
        }

        #[test]
        fn validate_two_denominator_terms() {
            let denominator = PER_GRAM_METER.to_denominator().unwrap();
            assert_eq!(denominator, GRAM_METER);
        }
    }
}
