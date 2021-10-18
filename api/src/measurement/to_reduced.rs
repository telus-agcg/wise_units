use super::Measurement;
use crate::{convertible::Convertible, error::Error, reduce::ToReduced};

impl ToReduced for Measurement {
    type Output = Result<Self, Error>;

    /// Reduces `self`'s `Term`s into a new `Unit`, consuming `self`.
    ///
    /// ```
    /// use std::str::FromStr;
    /// use wise_units::reduce::ToReduced;
    /// use wise_units::Measurement;
    ///
    /// // "m2" doesn't reduce down...
    /// let m1 = Measurement::try_new(10.0, "m2").unwrap();
    /// let m2 = Measurement::try_new(10.0, "m2").unwrap();
    /// assert_eq!(m1.to_reduced().unwrap(), m2);
    ///
    /// // ...but "m4/m2" does.
    /// let m1 = Measurement::try_new(10.0, "m4/m2").unwrap();
    /// let m2 = Measurement::try_new(10.0, "m2").unwrap();
    /// assert_eq!(m1.to_reduced().unwrap(), m2);
    ///
    /// // This also works for Units of the same dimension, but with different scalar
    /// // representations.
    /// let m1 = Measurement::try_new(5.0, "[lb_av].har/m2").unwrap();
    /// let m2 = Measurement::try_new(50_000.0, "[lb_av]").unwrap();
    /// assert_eq!(m1.to_reduced().unwrap(), m2);
    /// ```
    ///
    #[inline]
    fn to_reduced(&self) -> Self::Output {
        let reduced_unit = self.unit.to_reduced();

        self.convert_to(&reduced_unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod to_reduced {
        use super::*;

        macro_rules! validate_reduction {
            ($test_name:ident, $input_value:expr, $input_unit:expr, $expected_value:expr, $expected_unit:expr) => {
                #[test]
                fn $test_name() {
                    let measurement = Measurement::try_new($input_value, $input_unit).unwrap();
                    let actual = measurement.to_reduced().expect("Unable to reduce");
                    let expected = Measurement::try_new($expected_value, $expected_unit).unwrap();

                    assert_eq!(&actual, &expected);
                    assert_eq!(actual.unit.expression(), expected.unit.expression());
                }
            };
        }

        validate_reduction!(validate_m, 1.0, "m", 1.0, "m");
        validate_reduction!(validate_m2_per_m, 1.0, "m2/m", 1.0, "m");
        validate_reduction!(validate_100m2_per_m, 1.0, "100m2/m", 1.0, "100m2/m");
        validate_reduction!(validate_m2_dot_m2, 1.0, "m2.m2", 1.0, "m4");
        validate_reduction!(
            validate_m2_dot_m2_per_s_dot_s,
            1.0,
            "m2.m2/s.s",
            1.0,
            "m4/s2"
        );
        validate_reduction!(validate_m2_dot_s_per_s_dot_m2, 1.0, "m2.s/s.m2", 1.0, "1");

        validate_reduction!(
            validate_lb_av_dot_har_per_m2,
            1.0,
            "kg.har/m2",
            10_000.0,
            "kg"
        );
        validate_reduction!(
            validate_lb_av_dot_har_per_m2_dot_g,
            1.0,
            "kg.har/m2.g",
            10_000_000.0,
            "1"
        );

        validate_reduction!(
            validate_acr_us_per_m2_per_har,
            1.0,
            "[acr_us]/m2/har",
            10_000.0,
            "[acr_us]"
        );
        validate_reduction!(
            validate_acr_us_per_m2_per_har_per_sft_i,
            1.0,
            "[acr_us]/m2/har/[sft_i]",
            435_601_742.405_227_1,
            "1"
        );
    }

    mod into_reduced {
        use super::*;
        use crate::reduce::IntoReduced;

        macro_rules! validate_reduction {
            ($test_name:ident, $input_value:expr, $input_unit:expr, $expected_value:expr, $expected_unit:expr) => {
                #[test]
                fn $test_name() {
                    let measurement = Measurement::try_new($input_value, $input_unit).unwrap();
                    let actual = measurement.into_reduced().expect("Unable to reduce");
                    let expected = Measurement::try_new($expected_value, $expected_unit).unwrap();

                    assert_eq!(&actual, &expected);
                    assert_eq!(actual.unit.expression(), expected.unit.expression());
                }
            };
        }

        validate_reduction!(validate_m, 1.0, "m", 1.0, "m");
        validate_reduction!(validate_m2_per_m, 1.0, "m2/m", 1.0, "m");
        validate_reduction!(validate_100m2_per_m, 1.0, "100m2/m", 1.0, "100m2/m");
        validate_reduction!(validate_m2_dot_m2, 1.0, "m2.m2", 1.0, "m4");
        validate_reduction!(
            validate_m2_dot_m2_per_s_dot_s,
            1.0,
            "m2.m2/s.s",
            1.0,
            "m4/s2"
        );
        validate_reduction!(validate_m2_dot_s_per_s_dot_m2, 1.0, "m2.s/s.m2", 1.0, "1");

        validate_reduction!(
            validate_lb_av_dot_har_per_m2,
            1.0,
            "kg.har/m2",
            10_000.0,
            "kg"
        );
        validate_reduction!(
            validate_lb_av_dot_har_per_m2_dot_g,
            1.0,
            "kg.har/m2.g",
            10_000_000.0,
            "1"
        );

        validate_reduction!(
            validate_acr_us_per_m2_per_har,
            1.0,
            "[acr_us]/m2/har",
            10_000.0,
            "[acr_us]"
        );
        validate_reduction!(
            validate_acr_us_per_m2_per_har_per_sft_i,
            1.0,
            "[acr_us]/m2/har/[sft_i]",
            435_601_742.405_227_1,
            "1"
        );
    }
}
