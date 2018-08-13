use parser::{Composable, Composition, Term};
use unit::Unit;

//-----------------------------------------------------------------------------
// impl Composable
//-----------------------------------------------------------------------------
impl Composable for Unit {
    fn composition(&self) -> Composition {
        let term_slice: &[Term] = self;

        term_slice.composition()
    }
}

impl<'a> Composable for &'a Unit {
    fn composition(&self) -> Composition {
        let term_slice: &[Term] = self;

        term_slice.composition()
    }
}

#[cfg(test)]
mod tests {
    use parser::{Composable, Composition, Dimension};
    use std::str::FromStr;
    use unit::Unit;

    #[test]
    fn validate_composition() {
        let unit = Unit::from_str("[pi]").unwrap();
        assert_eq!(unit.composition(), Composition::default());

        let unit = Unit::from_str("[ppth]").unwrap();
        assert_eq!(unit.composition(), Composition::default());

        let unit = Unit::from_str("m").unwrap();
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("km").unwrap();
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("km/10m").unwrap();
        let composition = Composition::new(Dimension::Length, 0);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("m-1").unwrap();
        let composition = Composition::new(Dimension::Length, -1);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("10m").unwrap();
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("10km").unwrap();
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("10km-1").unwrap();
        let composition = Composition::new(Dimension::Length, -1);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("km-1/m2").unwrap();
        let composition = Composition::new(Dimension::Length, -3);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("km/m2.cm").unwrap();
        let composition = Composition::new(Dimension::Length, -2);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        let composition = Composition::new(Dimension::Length, -4);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("m/s2").unwrap();
        let mut composition = Composition::new(Dimension::Length, 1);
        composition.insert(Dimension::Time, -2);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("/1").unwrap();
        assert_eq!(unit.composition(), Composition::default());

        let unit = Unit::from_str("/m").unwrap();
        let composition = Composition::new(Dimension::Length, -1);
        assert_eq!(unit.composition(), composition);

        let unit = Unit::from_str("/{tot}").unwrap();
        assert_eq!(unit.composition(), Composition::default());
    }

    #[test]
    fn validate_is_compatible_with() {
        let meter = Unit::from_str("m").unwrap();
        let km = Unit::from_str("km").unwrap();
        assert!(meter.is_compatible_with(&km));

        let km_per_10m = Unit::from_str("km/10m").unwrap();
        assert!(!meter.is_compatible_with(&km_per_10m));

        let per_meter = Unit::from_str("m-1").unwrap();
        assert!(!meter.is_compatible_with(&per_meter));

        let ten_meter = Unit::from_str("10m").unwrap();
        assert!(meter.is_compatible_with(&ten_meter));

        let ten_km = Unit::from_str("10km").unwrap();
        assert!(meter.is_compatible_with(&ten_km));

        let per_ten_km = Unit::from_str("10km-1").unwrap();
        assert!(!meter.is_compatible_with(&per_ten_km));

        let per_meter_cubed = Unit::from_str("km-1/m2").unwrap();
        assert!(!meter.is_compatible_with(&per_meter_cubed));

        let km_per_length_cubed = Unit::from_str("km/m2.cm").unwrap();
        assert!(!meter.is_compatible_with(&km_per_length_cubed));

        let km_per_length_fourthed = Unit::from_str("km-1/m2.cm").unwrap();
        assert!(!meter.is_compatible_with(&km_per_length_fourthed));

        let meter_per_second_squared = Unit::from_str("m/s2").unwrap();
        assert!(!meter.is_compatible_with(&meter_per_second_squared));

        let km_cubed_per_nanometer_squared = Unit::from_str("km3/nm2").unwrap();
        assert!(meter.is_compatible_with(&km_cubed_per_nanometer_squared));
    }
}
