use parser::{Composable, Composition, Term};
use unit::Unit;

//-----------------------------------------------------------------------------
// impl Composable
//-----------------------------------------------------------------------------
impl Composable for Unit {
    #[inline]
    fn composition(&self) -> Composition {
        let term_slice: &[Term] = self;

        term_slice.composition()
    }
}

impl<'a> Composable for &'a Unit {
    #[inline]
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

    macro_rules! build_composition {
        (
            @params
            $composition:expr,
            $expected_dimension_variant:ident,
            $expected_dimension_value:expr
        ) => {
            $composition.insert(
                Dimension::$expected_dimension_variant,
                $expected_dimension_value,
            );
        };
    }

    macro_rules! valdiate_composition {
        ($test_name:ident, $input_string:expr) => {
            #[test]
            fn $test_name() {
                let unit = Unit::from_str($input_string).unwrap();

                assert_eq!(unit.composition(), Composition::default());
            }
        };

        ($test_name:ident, $input_string:expr $(, $expected_dimension_variant:ident: $expected_dimension_value:expr)* $(,)*) => {
            #[test]
            fn $test_name() {
                let unit = Unit::from_str($input_string).unwrap();
                let mut composition = Composition::default();
                $(
                    build_composition!(@params composition, $expected_dimension_variant, $expected_dimension_value);
                )*

                assert_eq!(unit.composition(), composition);
            }
        };
    }

    valdiate_composition!(validate_composition_m,                                             "m",           Length: 1);
    valdiate_composition!(validate_composition_m_prefix,                                      "km",          Length: 1);
    valdiate_composition!(validate_composition_m_positive_exponent,                           "m2",          Length: 2);
    valdiate_composition!(validate_composition_m_negative_exponent_1,                         "m-1",         Length: -1);
    valdiate_composition!(validate_composition_m_negative_exponent_2,                         "m-2",         Length: -2);
    valdiate_composition!(validate_composition_m_factor,                                      "10m",         Length: 1);
    valdiate_composition!(validate_composition_m_factor_prefix,                               "10km",        Length: 1);
    valdiate_composition!(validate_composition_m_factor_prefix_neg_exponent,                  "10km-1",      Length: -1);
    valdiate_composition!(validate_composition_m_prefix_per_factor_m,                         "km/m",        Length: 0);
    valdiate_composition!(validate_composition_m_prefix_per_m_exponent_dot_cm,                "km/m2.cm",    Length: -2);
    valdiate_composition!(validate_composition_m_prefix_neg_exponent_per_m_exponent,          "km-1/m2",     Length: -3);
    valdiate_composition!(validate_composition_m_prefix_neg_exponent_per_m_exponent_dot_cm,   "km-1/m2.cm",  Length: -4);
    valdiate_composition!(validate_composition_per_m,                                         "/m",          Length: -1);

    valdiate_composition!(validate_composition_m_per_s_exponent,    "m/s2",  Length: 1,  Time: -2);

    valdiate_composition!(validate_composition_lb_av_dot_acr_us_per_har, "[lb_av].[acr_us]/har", Mass: 1);

    valdiate_composition!(validate_composition_dimensionless_pi, "[pi]");
    valdiate_composition!(validate_composition_dimensionless_ppth, "[ppth]");
    valdiate_composition!(validate_composition_per_1, "/1");
    valdiate_composition!(validate_composition_per_annotation, "/{tot}");

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
