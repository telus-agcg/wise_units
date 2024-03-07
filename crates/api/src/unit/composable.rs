use crate::{parser::Composition, Composable, Unit};

//-----------------------------------------------------------------------------
// impl Composable
//-----------------------------------------------------------------------------
impl Composable for Unit {
    #[inline]
    fn composition(&self) -> Composition {
        self.terms.composition()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    use crate::{parser::Dimension, Unit};

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
}
