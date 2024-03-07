use crate::{Error, Unit};
use std::str::FromStr;

//-----------------------------------------------------------------------------
// impl FromStr
//-----------------------------------------------------------------------------
impl FromStr for Unit {
    type Err = Error;

    #[inline]
    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        Ok(super::parser::parse(expression).and_then(Self::try_from)?)
    }
}

#[cfg(test)]
mod tests {
    use super::{FromStr, Unit};

    #[test]
    fn validate_from_str_error() {
        let unit = Unit::from_str("ZZZXXXXXXXXXXXXx");
        assert!(unit.is_err());
    }

    #[test]
    fn validate_annotation() {
        let unit = Unit::from_str("{foo}").unwrap();
        let term = unit.terms.first().unwrap();
        let annotation = &term.annotation;

        assert_eq!(annotation, &Some("foo".to_string()));
    }

    #[test]
    fn meow_test() {
        assert_eq!(Unit::from_str("m").unwrap(), Unit::new(vec![term!(Meter)]));
        assert_eq!(
            Unit::from_str("2m").unwrap(),
            Unit::new(vec![term!(Meter, factor: 2)])
        );
        assert_eq!(
            Unit::from_str("2m2").unwrap(),
            Unit::new(vec![term!(Meter, factor: 2, exponent: 2)])
        );
        assert_eq!(
            Unit::from_str("2m2/m3").unwrap(),
            Unit::new(vec![
                term!(Meter, factor: 2, exponent: 2),
                term!(Meter, exponent: -3),
            ])
        );
        assert_eq!(Unit::from_str("L").unwrap(), Unit::new(vec![term!(Liter)]));
        assert_eq!(Unit::from_str("l").unwrap(), Unit::new(vec![term!(Liter)]));
        assert_eq!(
            Unit::from_str("har").unwrap(),
            Unit::new(vec![term!(Hecto, Are)])
        );
        assert_eq!(
            Unit::from_str("km/m2.cm").unwrap(),
            Unit::new(vec![
                term!(Kilo, Meter),
                term!(Meter, exponent: -2),
                term!(Centi, Meter),
            ])
        );
    }
}
// TODO: Move these tests to be with the code that converts the parse tree to terms/unit.
//
#[cfg(test)]
mod old_mapper_tests {
    use super::*;

    macro_rules! validate_interpret {
        ($test_name:ident, $input:expr, $($terms:expr),+) => {
            #[test]
            fn $test_name() {
                let actual = Unit::from_str($input).unwrap();
                let expected = Unit::new(vec![$($terms),+]);

                assert_eq!(actual, expected);
            }
        };
    }

    #[test]
    fn validate_exponent() {
        let actual = Unit::from_str("m-3").unwrap();
        let expected = Unit::new(vec![term!(Meter, exponent: -3)]);

        assert_eq!(actual, expected);

        let actual = Unit::from_str("km2/m-3").unwrap();

        let term1 = term!(Kilo, Meter, exponent: 2);
        let term2 = term!(Meter, exponent: 3);
        let expected = Unit::new(vec![term1, term2]);

        assert_eq!(actual, expected);
    }

    validate_interpret!(validate_interpret_meter, "m", term!(Meter));
    validate_interpret!(
        validate_interpret_meter_positive_exponent,
        "m2",
        term!(Meter, exponent: 2)
    );
    validate_interpret!(
        validate_interpret_meter_negative_exponent,
        "m-2",
        term!(Meter, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_meter_factor,
        "2m",
        term!(Meter, factor: 2)
    );
    validate_interpret!(validate_interpret_kilometer, "km", term!(Kilo, Meter));

    validate_interpret!(validate_interpret_kilobyte, "kBy", term!(Kilo, Byte));
    validate_interpret!(validate_sec_interpret_klobyte, "KBY", term!(Kilo, Byte));
    validate_interpret!(validate_sec_interpret_kibibyte, "KIBBY", term!(Kibi, Byte));
    validate_interpret!(validate_interpret_kibibyte, "KiBy", term!(Kibi, Byte));
    validate_interpret!(validate_interpret_mebibyte, "MiBy", term!(Mebi, Byte));
    validate_interpret!(validate_interpret_gibibyte, "GiBy", term!(Gibi, Byte));
    validate_interpret!(validate_interpret_tebibyte, "TiBy", term!(Tebi, Byte));
    validate_interpret!(validate_interpret_yottabyte, "YBy", term!(Yotta, Byte));

    // Slash terms
    validate_interpret!(
        validate_interpret_meter_per_second,
        "m/s",
        term!(Meter),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_kilometer_per_second,
        "km/s",
        term!(Kilo, Meter),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter_per_kilosecond,
        "m/ks",
        term!(Meter),
        term!(Kilo, Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter2_per_second,
        "m2/s",
        term!(Meter, exponent: 2),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter_per_second2,
        "m/s2",
        term!(Meter),
        term!(Second, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_meter2_per_second2,
        "m2/s2",
        term!(Meter, exponent: 2),
        term!(Second, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_2meter_per_second,
        "2m/s",
        term!(Meter, factor: 2),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter_per_2second,
        "m/2s",
        term!(Meter),
        term!(Second, exponent: -1, factor: 2)
    );
    validate_interpret!(
        validate_interpret_2meter2_per_2second2,
        "2m2/2s2",
        term!(Meter, factor: 2, exponent: 2),
        term!(Second, factor: 2, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_foot_per_factor,
        "[ft_i]/12",
        term!(FootInternational),
        term!(factor: 12, exponent: -1)
    );

    // Dot terms
    validate_interpret!(
        validate_interpret_meter_second,
        "m.s",
        term!(Meter),
        term!(Second)
    );
    validate_interpret!(
        validate_interpret_meter2_second,
        "m2.s",
        term!(Meter, exponent: 2),
        term!(Second)
    );
    validate_interpret!(
        validate_interpret_meter_second2,
        "m.s2",
        term!(Meter),
        term!(Second, exponent: 2)
    );
    validate_interpret!(
        validate_interpret_meter2_second2,
        "m2.s2",
        term!(Meter, exponent: 2),
        term!(Second, exponent: 2)
    );
    validate_interpret!(
        validate_interpret_2meter_second,
        "2m.s",
        term!(Meter, factor: 2),
        term!(Second)
    );
    validate_interpret!(
        validate_interpret_meter_2second,
        "m.2s",
        term!(Meter),
        term!(Second, factor: 2)
    );
    validate_interpret!(
        validate_interpret_2meter_2second,
        "2m.2s",
        term!(Meter, factor: 2),
        term!(Second, factor: 2)
    );
    validate_interpret!(
        validate_interpret_2meter2_2second2,
        "2m2.2s2",
        term!(Meter, factor: 2, exponent: 2),
        term!(Second, factor: 2, exponent: 2)
    );

    // Dot and slash combined terms
    validate_interpret!(
        validate_interpret_acre_inch_per_acre,
        "[acr_us].[in_i]/[acr_us]",
        term!(AcreUS),
        term!(InchInternational),
        term!(AcreUS, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_2acre3_3inch4_per_4acre5,
        "2[acr_us]3.3[in_i]4/4[acr_us]5",
        term!(AcreUS, factor: 2, exponent: 3),
        term!(InchInternational, factor: 3, exponent: 4),
        term!(AcreUS, factor: 4, exponent: -5)
    );

    #[test]
    #[ignore]
    fn validate_custom_atom() {
        let actual = Unit::from_str("[meow]").unwrap();

        let acre_term = term!(AcreUS);
        let inch_term = term!(InchInternational);
        let acre_inverse_term = term!(AcreUS, exponent: -1);

        let expected = Unit::new(vec![acre_term, inch_term, acre_inverse_term]);

        assert_eq!(actual, expected);
    }
}
