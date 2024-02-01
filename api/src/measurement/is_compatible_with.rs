use crate::{is_compatible_with::IsCompatibleWith, measurement::Measurement, unit::Unit};

impl IsCompatibleWith for Measurement {
    #[inline]
    fn is_compatible_with(&self, rhs: &Self) -> bool {
        self.is_compatible_with(&rhs.unit)
    }
}

#[cfg_attr(feature = "cffi", ffi_common::derive::expose_impl)]
impl IsCompatibleWith<Unit> for Measurement {
    #[inline]
    fn is_compatible_with(&self, rhs: &Unit) -> bool {
        self.unit.is_compatible_with(rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{is_compatible_with::IsCompatibleWith, measurement::Measurement, unit::Unit};
    use std::str::FromStr;

    #[test]
    fn validate_is_compatible_with_measurement() {
        let subject = Measurement::try_new(1.0, "m").unwrap();
        let other = Measurement::try_new(1.0, "km").unwrap();
        assert!(subject.is_compatible_with(&other));

        let other = Measurement::try_new(1.0, "kg").unwrap();
        assert!(!subject.is_compatible_with(&other));

        let other = Measurement::try_new(1.0, "m{foo}").unwrap();
        assert!(!subject.is_compatible_with(&other));

        let subject = Measurement::try_new(1.0, "m{foo}").unwrap();
        let other = Measurement::try_new(1.0, "km{foo}").unwrap();
        assert!(subject.is_compatible_with(&other));

        let other = Measurement::try_new(1.0, "g{foo}").unwrap();
        assert!(!subject.is_compatible_with(&other));

        let subject = Measurement::try_new(1.0, "GiBy{bit}").unwrap();
        let other = Measurement::try_new(1.0, "Kibit{bit}").unwrap();
        assert!(subject.is_compatible_with(&other));

        assert!(subject.is_compatible_with(&Measurement::try_new(1.0, "Mibit{bit}").unwrap()));
        assert!(subject.is_compatible_with(&Measurement::try_new(1.0, "Mbit{bit}").unwrap()));
        assert!(subject.is_compatible_with(&Measurement::try_new(1.0, "MiBy{bit}").unwrap()));

        let other = Measurement::try_new(1.0, "kbit").unwrap();
        assert!(!subject.is_compatible_with(&other));
    }

    #[test]
    fn validate_is_compatible_with_unit() {
        let subject = Measurement::try_new(1.0, "m").unwrap();
        let other = Unit::from_str("km").unwrap();
        assert!(subject.is_compatible_with(&other));

        let other = Unit::from_str("kg").unwrap();
        assert!(!subject.is_compatible_with(&other));

        let other = Unit::from_str("m{foo}").unwrap();
        assert!(!subject.is_compatible_with(&other));

        let subject = Measurement::try_new(1.0, "m{foo}").unwrap();
        let other = Unit::from_str("km{foo}").unwrap();
        assert!(subject.is_compatible_with(&other));

        let other = Unit::from_str("g{foo}").unwrap();
        assert!(!subject.is_compatible_with(&other));
    }
}
