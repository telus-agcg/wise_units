use measurement::Measurement;
use parser::{Composable, Composition};

//-----------------------------------------------------------------------------
// impl Composable
//-----------------------------------------------------------------------------
impl<'a> Composable for &'a Measurement {
    #[inline]
    fn composition(self) -> Composition {
        self.unit.composition()
    }
}

#[cfg(test)]
mod tests {
    use measurement::Measurement;
    use parser::IsCompatibleWith;
    use std::str::FromStr;
    use unit::Unit;

    #[test]
    fn validate_is_compatible_with_measurement() {
        let subject = Measurement::new(1.0, "m").unwrap();
        let other = Measurement::new(1.0, "km").unwrap();

        assert!(subject.is_compatible_with(&other));

        let other = Measurement::new(1.0, "kg").unwrap();
        assert!(!subject.is_compatible_with(&other));
    }

    #[test]
    fn validate_is_compatible_with_unit() {
        let subject = Measurement::new(1.0, "m").unwrap();
        let other = Unit::from_str("km").unwrap();

        assert!(subject.is_compatible_with(&other));

        let other = Unit::from_str("kg").unwrap();
        assert!(!subject.is_compatible_with(&other));
    }
}
