//! This module defines functionality for both `Add` and `Sub`. Their behavior are very similar
//! internally and it's easier to test them both if they're in the same module.
//!
use std::ops::{Add, Sub};

use crate::{convertible::Convertible, error::Error, measurement::Measurement};

//-----------------------------------------------------------------------------
// impl Add
//-----------------------------------------------------------------------------
#[cfg_attr(
    feature = "cffi",
    ffi_common::derive::expose_fn(extend_type(Measurement))
)]
#[allow(clippy::result_large_err)]
fn add_measurements(lhs: &Measurement, rhs: &Measurement) -> Result<Measurement, Error> {
    let rhs_converted = rhs.convert_to(&lhs.unit)?;
    let new_value = lhs.value + rhs_converted.value;

    Ok(Measurement {
        value: new_value,
        unit: lhs.unit.clone(),
    })
}

impl Add for Measurement {
    type Output = Result<Self, Error>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        add_measurements(&self, &other)
    }
}

impl<'a> Add<&'a Self> for Measurement {
    type Output = Result<Self, Error>;

    #[inline]
    fn add(self, other: &'a Self) -> Self::Output {
        add_measurements(&self, other)
    }
}

impl<'a> Add for &'a Measurement {
    type Output = Result<Measurement, Error>;

    #[inline]
    fn add(self, other: &'a Measurement) -> Self::Output {
        add_measurements(self, other)
    }
}

impl<'a> Add<Measurement> for &'a Measurement {
    type Output = Result<Measurement, Error>;

    #[inline]
    fn add(self, other: Measurement) -> Self::Output {
        add_measurements(self, &other)
    }
}

//-----------------------------------------------------------------------------
// impl Sub
//-----------------------------------------------------------------------------
#[cfg_attr(
    feature = "cffi",
    ffi_common::derive::expose_fn(extend_type(Measurement))
)]
#[allow(clippy::result_large_err)]
fn sub_measurements(lhs: &Measurement, rhs: &Measurement) -> Result<Measurement, Error> {
    let rhs_converted = rhs.convert_to(&lhs.unit)?;
    let new_value = lhs.value - rhs_converted.value;

    Ok(Measurement {
        value: new_value,
        unit: lhs.unit.clone(),
    })
}

impl Sub for Measurement {
    type Output = Result<Self, Error>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        sub_measurements(&self, &other)
    }
}

impl<'a> Sub<&'a Self> for Measurement {
    type Output = Result<Self, Error>;

    #[inline]
    fn sub(self, other: &'a Self) -> Self::Output {
        sub_measurements(&self, other)
    }
}

impl<'a> Sub for &'a Measurement {
    type Output = Result<Measurement, Error>;

    #[inline]
    fn sub(self, other: &'a Measurement) -> Self::Output {
        sub_measurements(self, other)
    }
}

impl<'a> Sub<Measurement> for &'a Measurement {
    type Output = Result<Measurement, Error>;

    #[inline]
    fn sub(self, other: Measurement) -> Self::Output {
        sub_measurements(self, &other)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::const_units::{
        l1::{DECIMETER, FOOT, KILOMETER, METER, NANOMETER, NANOPARSEC},
        l2::{
            CENTIMETER_SQUARED, DECIMETER_SQUARED, FOOT_SQUARED, METER_SQUARED, YOCTOPARSEC_SQUARED,
        },
        l3::{DECIMETER_CUBED, METER_CUBED},
        m1::{CENTITONNE, GRAM, KILOGRAM},
        m2::CENTIGRAM_SQUARED,
        t2::SECOND_SQUARED,
    };

    use super::*;

    // ╭─────────────────────────────────╮
    // │ Positive testing macros section │
    // ╰─────────────────────────────────╯
    macro_rules! assert_eq_add {
        ($lhs:expr, $rhs:expr, $expected:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;

            // borrowed + borrowed
            pretty_assertions::assert_eq!((&lhs + &rhs).unwrap(), $expected);

            // borrowed + owned
            pretty_assertions::assert_eq!((&lhs + rhs.clone()).unwrap(), $expected);

            // owned + borrowed
            pretty_assertions::assert_eq!((lhs.clone() + &rhs).unwrap(), $expected);

            // owned + owned
            pretty_assertions::assert_eq!((lhs + rhs).unwrap(), $expected);
        }};
    }

    macro_rules! assert_eq_sub {
        ($lhs:expr, $rhs:expr, $expected:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;

            // borrowed - borrowed
            pretty_assertions::assert_eq!((&lhs - &rhs).unwrap(), $expected);

            // borrowed - owned
            pretty_assertions::assert_eq!((&lhs - rhs.clone()).unwrap(), $expected);

            // owned - borrowed
            pretty_assertions::assert_eq!((lhs.clone() - &rhs).unwrap(), $expected);

            // owned - owned
            pretty_assertions::assert_eq!((lhs - rhs).unwrap(), $expected);
        }};
    }

    macro_rules! validate_ok {
        (
            $test_name:ident,
            $lhs_unit:expr, $rhs_unit:expr,
            add_values: $expected_add_lhs_value:expr, $expected_add_rhs_value:expr,
            sub_values: $expected_sub_lhs_value: expr, $expected_sub_rhs_value: expr
        ) => {
            #[test]
            fn $test_name() {
                let lhs = Measurement::new(1.0, $lhs_unit);
                let rhs = Measurement::new(1.0, $rhs_unit);

                assert_eq_add!(
                    lhs.clone(),
                    rhs.clone(),
                    Measurement::new($expected_add_lhs_value, $lhs_unit)
                );
                assert_eq_sub!(
                    lhs.clone(),
                    rhs.clone(),
                    Measurement::new($expected_sub_lhs_value, $lhs_unit)
                );

                assert_eq_add!(
                    rhs.clone(),
                    lhs.clone(),
                    Measurement::new($expected_add_rhs_value, $rhs_unit)
                );
                assert_eq_sub!(
                    rhs,
                    lhs,
                    Measurement::new($expected_sub_rhs_value, $rhs_unit)
                );
            }
        };
    }

    macro_rules! validate_same_unit {
        ($unit:expr) => {
            validate_ok!(
                validate_same_unit,
                $unit.clone(), $unit.clone(),
                add_values: 2.0, 2.0,
                sub_values: 0.0, 0.0
            );
        };
    }

    /// Macro to validate add/sub for units that differ only by their atom, where the atoms are
    /// comparable/of the same dimension.
    macro_rules! validate_same_unit_different_atom_same_dimension {
        (
            $lhs_unit:expr, $rhs_unit:expr,
            add_values: $expected_add_lhs_value:expr, $expected_add_rhs_value:expr,
            sub_values: $expected_sub_lhs_value:expr, $expected_sub_rhs_value:expr
        ) => {
            validate_ok!(
                validate_same_unit_different_atom_same_dimension,
                $lhs_unit, $rhs_unit,
                add_values: $expected_add_lhs_value, $expected_add_rhs_value,
                sub_values: $expected_sub_lhs_value, $expected_sub_rhs_value
            );
        };
    }

    macro_rules! validate_same_unit_different_prefix {
        (
            $lhs_unit:expr, $rhs_unit:expr,
            add_values: $expected_add_lhs_value:expr, $expected_add_rhs_value:expr,
            sub_values: $expected_sub_lhs_value: expr, $expected_sub_rhs_value: expr
        ) => {
            validate_ok!(
                validate_same_unit_different_prefix,
                $lhs_unit, $rhs_unit,
                add_values: $expected_add_lhs_value, $expected_add_rhs_value,
                sub_values: $expected_sub_lhs_value, $expected_sub_rhs_value
            );
        };
    }

    macro_rules! validate_same_prefix_different_compatible_atom {
        (
            $lhs_unit:expr, $rhs_unit:expr,
            add_values: $expected_add_lhs_value:expr, $expected_add_rhs_value:expr,
            sub_values: $expected_sub_lhs_value: expr, $expected_sub_rhs_value: expr
        ) => {
            validate_ok!(
                validate_same_prefix_different_compatible_atom ,
                $lhs_unit, $rhs_unit,
                add_values: $expected_add_lhs_value, $expected_add_rhs_value,
                sub_values: $expected_sub_lhs_value, $expected_sub_rhs_value
            );
        };
    }

    macro_rules! validate_same_unit_different_factor {
        (
            $lhs_unit:expr, $rhs_unit:expr,
            add_values: $expected_add_lhs_value:expr, $expected_add_rhs_value:expr,
            sub_values: $expected_sub_lhs_value: expr, $expected_sub_rhs_value: expr
        ) => {
            validate_ok!(
                validate_same_unit_different_factor ,
                $lhs_unit, $rhs_unit,
                add_values: $expected_add_lhs_value, $expected_add_rhs_value,
                sub_values: $expected_sub_lhs_value, $expected_sub_rhs_value
            );
        };
    }

    macro_rules! validate_dimless_different_exponent {
        (
            $lhs_unit:expr, $rhs_unit:expr,
            add_values: $expected_add_lhs_value:expr, $expected_add_rhs_value:expr,
            sub_values: $expected_sub_lhs_value: expr, $expected_sub_rhs_value: expr
        ) => {
            validate_ok!(
                validate_dimless_different_exponent,
                $lhs_unit, $rhs_unit,
                add_values: $expected_add_lhs_value, $expected_add_rhs_value,
                sub_values: $expected_sub_lhs_value, $expected_sub_rhs_value
            );
        };
    }

    // ╭─────────────────────────────────╮
    // │ Negative testing macros section │
    // ╰─────────────────────────────────╯
    macro_rules! validate_is_error {
        ($test_name:ident, $lhs_unit:expr, $rhs_unit:expr) => {
            #[test]
            fn $test_name() {
                let lhs = Measurement::new(1.0, $lhs_unit);
                let rhs = Measurement::new(2.0, $rhs_unit);

                assert_err_add!(lhs.clone(), rhs.clone());
                assert_err_sub!(lhs, rhs);
            }
        };
    }

    macro_rules! validate_different_atom_different_dimension {
        ($lhs_unit:expr, $rhs_unit:expr) => {
            validate_is_error!(
                validate_different_atom_different_dimension,
                $lhs_unit,
                $rhs_unit
            );
        };
    }

    /// Macro for validating that a LHS unit that's all the same as the RHS, _except_ for the
    /// annotation, cannot be added or subtracted.
    ///
    macro_rules! validate_same_unit_different_annotation {
        ($lhs_unit:expr, $rhs_unit:expr) => {
            validate_is_error!(
                validate_same_unit_different_annotation,
                $lhs_unit,
                $rhs_unit
            );
        };
    }

    macro_rules! validate_same_unit_different_exponent {
        ($lhs_unit:expr, $rhs_unit:expr) => {
            validate_is_error!(validate_same_unit_different_exponent, $lhs_unit, $rhs_unit);
        };
    }

    macro_rules! assert_err_add {
        ($lhs:expr, $rhs:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;

            // borrowed + borrowed
            assert!((&lhs + &rhs).is_err());

            // borrowed + owned
            assert!((&lhs + rhs.clone()).is_err());

            // owned + borrowed
            assert!((lhs.clone() + &rhs).is_err());

            // owned + owned
            assert!((lhs + rhs).is_err());
        }};
    }

    macro_rules! assert_err_sub {
        ($lhs:expr, $rhs:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;

            // borrowed - borrowed
            assert!((&lhs + &rhs).is_err());

            // borrowed - owned
            assert!((&lhs + rhs.clone()).is_err());

            // owned - borrowed
            assert!((lhs.clone() + &rhs).is_err());

            // owned - owned
            assert!((lhs + rhs).is_err());
        }};
    }

    //          ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
    //          ┃                    End macro section                    ┃
    //          ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

    mod annotation {
        use super::*;

        validate_same_unit!(parse_unit!("{tree}"));

        validate_same_prefix_different_compatible_atom!(
            parse_unit!("{tree}"), parse_unit!("[ppth]{tree}"),
            add_values: 1.001, 1001.0,
            sub_values: 0.999, -999.0
        );

        validate_same_unit_different_factor!(
            parse_unit!("{tree}"), parse_unit!("2{tree}"),
            add_values:  3.0, 1.5,
            sub_values: -1.0, 0.5
        );

        validate_dimless_different_exponent!(
            parse_unit!("{tree}"), unit!(term!(factor: 1, exponent: 2, annotation: "tree")),
            add_values: 2.0, 2.0,
            sub_values: 0.0, 0.0
        );

        validate_ok!(
            validate_same_multi_term_annotation,
            parse_unit!("{thing}/{tree}"), parse_unit!("{thing}/{tree}"),
            add_values: 2.0, 2.0,
            sub_values: 0.0, 0.0
        );

        validate_same_unit_different_annotation!(
            unit!(term!(annotation: "tree")),
            unit!(term!(annotation: "pants"))
        );
    }

    mod atom {
        use super::*;

        validate_same_unit!(METER);

        validate_same_unit_different_atom_same_dimension!(
            METER, FOOT,
            add_values: 1.3048, 4.280_839_895_013_123_5,
            sub_values: 0.695_2, -2.280_839_895_013_123
        );

        validate_same_unit_different_prefix!(
            METER, KILOMETER,
            add_values: 1001.0, 1.001,
            sub_values: -999.0, 0.999
        );

        validate_same_unit_different_factor!(
            METER, parse_unit!("2m"),
            add_values:  3.0, 1.5,
            sub_values: -1.0, 0.5
        );

        validate_dimless_different_exponent!(
            parse_unit!("[ppth]"), unit!(term!(PartsPerThousand, factor: 1, exponent: 2)),
            add_values: 1.001, 1001.0,
            sub_values: 0.999, -999.0
        );

        validate_different_atom_different_dimension!(METER, GRAM);
        validate_same_unit_different_exponent!(METER, METER_SQUARED);
    }

    mod atom_annotation {
        use super::*;

        validate_same_unit!(parse_unit!("g{tree}"));

        validate_same_unit_different_atom_same_dimension!(
            parse_unit!("g{seed}"), parse_unit!("[lb_av]{seed}"),
            add_values:  454.592_37, 1.002_204_622_621_848_8,
            sub_values: -452.592_37, 0.997_795_377_378_151_3
        );

        validate_same_unit_different_prefix!(
            parse_unit!("g{seed}"), parse_unit!("kg{seed}"),
            add_values: 1001.0, 1.001,
            sub_values: -999.0, 0.999
        );

        validate_same_unit_different_factor!(
            parse_unit!("g{tree}"), parse_unit!("2g{tree}"),
            add_values:  3.0, 1.5,
            sub_values: -1.0, 0.5
        );

        validate_dimless_different_exponent!(
            parse_unit!("[ppth]{dirt}"),
            unit!(term!(PartsPerThousand, factor: 1, exponent: 2, annotation: "dirt")),
            add_values: 1.001, 1001.0,
            sub_values: 0.999, -999.0
        );

        validate_same_unit_different_annotation!(
            unit!(term!(Gram, annotation: "seed")),
            unit!(term!(Gram, annotation: "tree"))
        );

        validate_different_atom_different_dimension!(
            parse_unit!("g{seed}"),
            parse_unit!("m{seed}")
        );

        validate_same_unit_different_exponent!(parse_unit!("g{seed}"), parse_unit!("g2{seed}"));
    }

    mod atom_exponent {
        use super::*;

        validate_same_unit!(METER_SQUARED);

        validate_same_unit_different_atom_same_dimension!(
            METER_SQUARED, FOOT_SQUARED,
            add_values: 1.092_903_04, 11.763_910_416_709_722,
            sub_values: 0.907_096_96, -9.763_910_416_709_722
        );

        validate_same_unit_different_prefix!(
            METER_SQUARED, DECIMETER_SQUARED,
            add_values: 1.01, 100.999_999_999_999_999,
            sub_values: 0.99, -98.999_999_999_999_999
        );

        validate_same_unit_different_factor!(
            METER_SQUARED, parse_unit!("2m2"),
            add_values:  5.0, 1.25,
            sub_values: -3.0, 0.75
        );

        validate_dimless_different_exponent!(
            parse_unit!("[ppth]2"), parse_unit!("[ppth]3"),
            add_values: 1.001, 1001.0,
            sub_values: 0.999, -999.0
        );

        validate_different_atom_different_dimension!(METER_SQUARED, SECOND_SQUARED);
        validate_same_unit_different_exponent!(METER_SQUARED, METER_CUBED);
    }

    mod atom_exponent_annotation {
        use super::*;

        validate_same_unit!(parse_unit!("s2{wind}"));

        validate_same_unit_different_atom_same_dimension!(
            parse_unit!("s2{wind}"), parse_unit!("min2{wind}"),
            add_values:  3601.0, 1.000_277_777_777_778,
            sub_values: -3599.0, 0.999_722_222_222_222
        );

        validate_same_unit_different_prefix!(
            parse_unit!("s2{wind}"), parse_unit!("ms2{wind}"),
            add_values: 1.000_001, 1_000_001.0,
            sub_values: 0.999_999, -999_999.0
        );

        validate_same_unit_different_factor!(
            parse_unit!("s2{wind}"), parse_unit!("2s2{wind}"),
            add_values:  5.0, 1.25,
            sub_values: -3.0, 0.75
        );

        validate_dimless_different_exponent!(
            parse_unit!("[ppth]2{stuff}"), parse_unit!("[ppth]3{stuff}"),
            add_values: 1.001, 1001.0,
            sub_values: 0.999, -999.0
        );

        validate_different_atom_different_dimension!(
            parse_unit!("s2{wind}"),
            parse_unit!("g2{wind}")
        );

        validate_same_unit_different_annotation!(parse_unit!("s2{wind}"), parse_unit!("s2{earth}"));
        validate_same_unit_different_exponent!(parse_unit!("s2{wind}"), parse_unit!("s3{wind}"));

        validate_is_error!(
            validate_different_atom_same_dimension_same_exponent_different_annotation,
            parse_unit!("s2{wind}"),
            parse_unit!("min2{fire}")
        );
    }

    // NOTE: There are 0 dimless atoms that are also metric, thus no dimless tests here.
    mod prefix_atom {
        use super::*;

        validate_same_unit!(KILOMETER);

        validate_same_unit_different_atom_same_dimension!(
            NANOMETER, NANOPARSEC,
            add_values: 3.085_678e16, 1.000_000_000_000_000_032_407_788_498_994_386_322_876_204_192,
            sub_values: -3.085_678e16, 1.000_000_000_000_000_032_407_788_498_994_386_322_876_204_192
        );

        validate_same_unit_different_prefix!(
            KILOMETER, DECIMETER,
            add_values: 1.000_1, 10_001.0,
            sub_values: 0.999_9, -9_999.0
        );

        validate_same_prefix_different_compatible_atom!(
            NANOMETER, NANOPARSEC,
            add_values: 3.085_678e16, 1.000_000_000_000_000_032_407_788_498_994_386_322_876_204_192,
            sub_values: -3.085_678e16, 1.000_000_000_000_000_032_407_788_498_994_386_322_876_204_192
        );

        validate_same_unit_different_factor!(
            KILOMETER, parse_unit!("2km"),
            add_values:  3.0, 1.5,
            sub_values: -1.0, 0.5
        );

        validate_different_atom_different_dimension!(KILOMETER, KILOGRAM);
        validate_same_unit_different_exponent!(KILOMETER, parse_unit!("km2"));

        #[test]
        fn validate_different_prefix_different_atom_same_dimension() {
            let kg = Measurement::new(1.0, KILOGRAM);
            let ct = Measurement::new(1.0, CENTITONNE);

            assert_eq_add!(kg.clone(), ct.clone(), Measurement::new(11.0, KILOGRAM));
            assert_eq_sub!(kg, ct, Measurement::new(-9.0, KILOGRAM));
        }
    }

    // NOTE: There are 0 dimless atoms that are also metric, thus no dimless tests here.
    mod prefix_atom_annotation {
        use super::*;

        validate_same_unit!(parse_unit!("kg{tree}"));

        validate_same_unit_different_atom_same_dimension!(
            parse_unit!("cg{tree}"), parse_unit!("ct{tree}"),
            add_values:  1_000_001.0, 1.000_001,
            sub_values: -999_999.0,   0.999_999
        );

        validate_same_unit_different_prefix!(
            parse_unit!("kg{tree}"), parse_unit!("dg{tree}"),
            add_values: 1.000_1, 10_001.0,
            sub_values: 0.999_9, -9_999.0
        );

        validate_same_unit_different_factor!(
            parse_unit!("kg2"), parse_unit!("2kg2"),
            add_values:  5.0, 1.25,
            sub_values: -3.0, 0.75
        );

        validate_same_unit_different_annotation!(parse_unit!("kg{tree}"), parse_unit!("kg{pants}"));

        validate_different_atom_different_dimension!(
            parse_unit!("Mg{tree}"),
            parse_unit!("cm{tree}")
        );

        validate_same_unit_different_exponent!(parse_unit!("kg{tree}"), parse_unit!("kg2{tree}"));
    }

    // NOTE: There are 0 dimless atoms that are also metric, thus no dimless tests here.
    mod prefix_atom_exponent {
        use super::*;

        validate_same_unit!(DECIMETER_SQUARED);

        validate_same_unit_different_atom_same_dimension!(
            parse_unit!("kg2"), parse_unit!("ct2"),
            add_values:  101.0, 1.01,
            sub_values: -99.0,  0.99
        );

        validate_same_unit_different_prefix!(
            DECIMETER_SQUARED, CENTIMETER_SQUARED,
            add_values: 1.01, 101.0,
            sub_values: 0.99, -99.0
        );

        validate_same_prefix_different_compatible_atom!(
            parse_unit!("cg2"), parse_unit!("ct2"),
            add_values: 1_000_000_000_001.0, 1.000_000_000_001,
            sub_values: -999_999_999_999.0,  0.999_999_999_999
        );

        validate_same_unit_different_factor!(
            parse_unit!("kg2"), parse_unit!("2kg2"),
            add_values:  5.0, 1.25,
            sub_values: -3.0, 0.75
        );

        validate_different_atom_different_dimension!(YOCTOPARSEC_SQUARED, CENTIGRAM_SQUARED);
        validate_same_unit_different_exponent!(DECIMETER_SQUARED, DECIMETER_CUBED);
    }

    // NOTE: There are 0 dimless atoms that are also metric, thus no dimless tests here.
    mod prefix_atom_exponent_annotation {
        use super::*;

        validate_same_unit!(parse_unit!("kg2{seed}"));

        validate_same_unit_different_atom_same_dimension!(
            parse_unit!("kg2{seed}"), parse_unit!("ct2{seed}"),
            add_values:  101.0, 1.01,
            sub_values: -99.0,  0.99
        );

        validate_same_unit_different_prefix!(
            parse_unit!("dg2{seed}"), parse_unit!("cg2{seed}"),
            add_values: 1.01, 101.0,
            sub_values: 0.99, -99.0
        );

        validate_same_prefix_different_compatible_atom!(
            parse_unit!("cg2{seed}"), parse_unit!("ct2{seed}"),
            add_values: 1_000_000_000_001.0, 1.000_000_000_001,
            sub_values: -999_999_999_999.0,  0.999_999_999_999
        );

        validate_same_unit_different_factor!(
            parse_unit!("kg2{seed}"), parse_unit!("2kg2{seed}"),
            add_values:  5.0, 1.25,
            sub_values: -3.0, 0.75
        );

        validate_same_unit_different_annotation!(
            parse_unit!("kg2{tree}"),
            parse_unit!("kg2{pants}")
        );

        validate_different_atom_different_dimension!(
            parse_unit!("kg2{seed}"),
            parse_unit!("km2{seed}")
        );

        validate_same_unit_different_exponent!(parse_unit!("kg2{seed}"), parse_unit!("kg3{seed}"));
    }

    mod factor {
        use super::*;

        validate_same_unit!(parse_unit!("2"));

        validate_same_prefix_different_compatible_atom!(
            parse_unit!("2"), parse_unit!("[ppth]"),
            add_values: 1.000_5,  2001.0,
            sub_values: 0.999_5, -1999.0
        );

        // NOTE: While the units are factors of different values, they are of the same dimension,
        // thus it makes sense that these should pass.
        validate_same_unit_different_factor!(
            parse_unit!("2"), parse_unit!("3"),
            add_values: 2.5,  1.666_666_666_666_666_5,
            sub_values: -0.5, 0.333_333_333_333_333_37
        );

        validate_dimless_different_exponent!(
            parse_unit!("2"), unit!(term!(factor: 2, exponent: 2)),
            add_values: 3.0,  1.5,
            sub_values: -1.0, 0.5
        );

        validate_different_atom_different_dimension!(parse_unit!("2"), parse_unit!("2g"));
        validate_same_unit_different_annotation!(parse_unit!("2"), parse_unit!("{thing}"));

        validate_is_error!(
            validate_factor_and_dimensioned_atom,
            parse_unit!("2"),
            METER
        );

        validate_is_error!(
            validate_factor_and_factor_annotation,
            parse_unit!("2"),
            parse_unit!("3{thing}")
        );

        validate_is_error!(
            validate_factor_and_factor_exponent_annotation,
            parse_unit!("2"),
            unit!(term!(factor: 2, exponent: 2, annotation: "meow"))
        );
    }

    mod factor_annotation {
        use super::*;

        validate_same_unit!(parse_unit!("2{seed}"));

        validate_same_prefix_different_compatible_atom!(
            parse_unit!("2{seed}"), parse_unit!("[ppth]{seed}"),
            add_values: 1.000_5,  2001.0,
            sub_values: 0.999_5, -1999.0
        );

        // NOTE: While the units are factors of different values, they are of the same dimension,
        // thus it makes sense that these should pass.
        validate_same_unit_different_factor!(
            parse_unit!("2{seed}"), parse_unit!("3{seed}"),
            add_values:  2.5, 1.666_666_666_666_666_5,
            sub_values: -0.5, 0.333_333_333_333_333_37
        );

        validate_dimless_different_exponent!(
            parse_unit!("2{meow}"), unit!(term!(factor: 2, exponent: 2, annotation: "meow")),
            add_values: 3.0,  1.5,
            sub_values: -1.0, 0.5
        );

        validate_ok!(
            validate_factor_annotation_and_same_annotation,
            parse_unit!("2{thing}"), parse_unit!("{thing}"),
            add_values: 1.5,  3.0,
            sub_values: 0.5, -1.0
        );

        validate_same_unit_different_annotation!(parse_unit!("2{seed}"), parse_unit!("2{plant}"));

        validate_different_atom_different_dimension!(
            parse_unit!("2{seed}"),
            parse_unit!("2g{seed}")
        );

        validate_is_error!(
            validate_different_factor_different_annotation,
            parse_unit!("2{seed}"),
            parse_unit!("3{plant}")
        );

        validate_is_error!(
            validate_factor_annotation_and_factor,
            parse_unit!("3{thing}"),
            parse_unit!("2")
        );

        validate_is_error!(
            validate_factor_annotation_and_factor_exponent,
            parse_unit!("2{stuff}"),
            unit!(term!(factor: 2, exponent: 2))
        );

        validate_is_error!(
            validate_factor_annotation_and_dimensionless_atom,
            parse_unit!("2{stuff}"),
            unit!(term!(PartsPerThousand))
        );

        validate_is_error!(
            validate_factor_annotation_and_dimensionless_atom_different_annotation,
            parse_unit!("2{stuff}"),
            parse_unit!("[ppth]{things}")
        );

        validate_is_error!(
            validate_factor_annotation_and_dimensioned_atom,
            parse_unit!("2{cat}"),
            METER
        );
    }

    mod factor_exponent {
        use super::*;

        validate_same_unit!(unit!(term!(factor: 2, exponent: 2)));

        validate_same_unit_different_factor!(
            unit!(term!(factor: 2, exponent: 2)), unit!(term!(factor: 3, exponent: 2)),
            add_values:  3.25, 1.444_444_444_444_444_4,
            sub_values: -1.25, 0.555_555_555_555_555_6
        );

        validate_dimless_different_exponent!(
            unit!(term!(factor: 2, exponent: 2)), unit!(term!(factor: 2, exponent: 3)),
            add_values:  3.0, 1.5,
            sub_values: -1.0, 0.5
        );

        validate_ok!(
            validate_with_factor,
            unit!(term!(factor: 2, exponent: 2)), parse_unit!("2"),
            add_values: 1.5,  3.0,
            sub_values: 0.5, -1.0
        );

        validate_ok!(
            validate_with_dimensionless_atom,
            unit!(term!(factor: 2, exponent: 2)), parse_unit!("[ppth]"),
            add_values: 1.000_25,  4001.0,
            sub_values: 0.999_75, -3999.0
        );

        validate_different_atom_different_dimension!(
            unit!(term!(factor: 2, exponent: 2)),
            unit!(term!(Gram, factor: 2, exponent: 2))
        );

        validate_is_error!(
            validate_with_annotation,
            unit!(term!(factor: 2, exponent: 3)),
            parse_unit!("{thing}")
        );

        validate_is_error!(
            validate_with_factor_exponent_annotation,
            unit!(term!(factor: 2, exponent: 3)),
            unit!(term!(factor: 2, exponent: 3, annotation: "thing"))
        );

        validate_is_error!(
            validate_with_dimensioned_atom,
            unit!(term!(factor: 2, exponent: 3)),
            METER
        );
    }

    mod factor_exponent_annotation {
        use super::*;

        validate_same_unit!(unit!(term!(factor: 2, exponent: 2, annotation: "seed")));

        validate_same_unit_different_factor!(
            unit!(term!(factor: 2, exponent: 2, annotation: "seed")),
            unit!(term!(factor: 4, exponent: 2, annotation: "seed")),
            add_values:  5.0, 1.25,
            sub_values: -3.0, 0.75
        );

        validate_dimless_different_exponent!(
            unit!(term!(factor: 2, exponent: 2, annotation: "seed")),
            unit!(term!(factor: 2, exponent: 3, annotation: "seed")),
            add_values:  3.0, 1.5,
            sub_values: -1.0, 0.5
        );

        // NOTE: While the units are factors of different values, they are of the same dimension,
        // thus it makes sense that these should pass.
        validate_ok!(
            validate_different_factor_different_exponent_same_annotation,
            unit!(term!(factor: 2, exponent: 2, annotation: "seed")),
            unit!(term!(factor: 3, exponent: 3, annotation: "seed")),
            add_values:  7.75, 1.148_148_148_148_148_1,
            sub_values: -5.75, 0.851_851_851_851_851_8
        );

        validate_same_unit_different_annotation!(
            unit!(term!(factor: 2, exponent: 2, annotation: "seed")),
            unit!(term!(factor: 2, exponent: 2, annotation: "plant"))
        );

        validate_different_atom_different_dimension!(
            unit!(term!(factor: 2, exponent: 2, annotation: "seed")),
            unit!(term!(Gram, factor: 2, exponent: 2, annotation: "seed"))
        );

        validate_is_error!(
            validate_with_factor,
            unit!(term!(factor: 2, exponent: 2, annotation: "seed")),
            parse_unit!("2")
        );
    }

    mod factor_atom {
        use super::*;

        validate_same_unit!(parse_unit!("2L"));

        validate_same_unit_different_atom_same_dimension!(
            parse_unit!("2000g"), parse_unit!("2t"),
            add_values:  1001.0, 1.001,
            sub_values: -999.0,  0.999
        );

        validate_same_unit_different_prefix!(
            parse_unit!("2g"), parse_unit!("2dg"),
            add_values: 1.1, 11.0,
            sub_values: 0.9, -9.0
        );

        validate_same_unit_different_factor!(
            parse_unit!("2g"), parse_unit!("3g"),
            add_values:  2.5, 1.666_666_666_666_666_5,
            sub_values: -0.5, 0.333_333_333_333_333_37
        );

        validate_dimless_different_exponent!(
            parse_unit!("2[ppth]"), parse_unit!("2[ppth]2"),
            add_values: 1.002,  501.0,
            sub_values: 0.998, -499.0
        );

        validate_different_atom_different_dimension!(parse_unit!("2g"), parse_unit!("2m"));
        validate_same_unit_different_exponent!(parse_unit!("2L"), parse_unit!("2L2"));
    }

    mod factor_atom_annotation {
        use super::*;

        validate_same_unit!(parse_unit!("2g{tree}"));

        validate_same_unit_different_atom_same_dimension!(
            parse_unit!("2g{seed}"), parse_unit!("3[lb_av]{seed}"),
            add_values:  681.388_555, 1.001_469_748_414_565_8,
            sub_values: -679.388_555, 0.998_530_251_585_434_2
        );

        validate_same_unit_different_prefix!(
            parse_unit!("2g{tree}"), parse_unit!("2dg{tree}"),
            add_values: 1.1, 11.0,
            sub_values: 0.9, -9.0
        );

        validate_same_unit_different_factor!(
            parse_unit!("2g{tree}"), parse_unit!("3g{tree}"),
            add_values:  2.5, 1.666_666_666_666_666_5,
            sub_values: -0.5, 0.333_333_333_333_333_37
        );

        validate_dimless_different_exponent!(
            parse_unit!("2[ppth]{beans}"), parse_unit!("2[ppth]2{beans}"),
            add_values: 1.002,  501.0,
            sub_values: 0.998, -499.0
        );

        validate_same_unit_different_annotation!(
            unit!(term!(Gram, factor: 2, annotation: "seed")),
            unit!(term!(Gram, factor: 2, annotation: "tree"))
        );

        validate_different_atom_different_dimension!(
            parse_unit!("2g{tree}"),
            parse_unit!("2m{tree}")
        );

        validate_same_unit_different_exponent!(parse_unit!("2g{tree}"), parse_unit!("2g2{tree}"));

        validate_is_error!(
            validate_different_factor_different_atom_same_dimension_different_annotation,
            parse_unit!("2g{seed}"),
            parse_unit!("2[lb_av]{tree}")
        );

        validate_is_error!(
            validate_same_factor_different_atom_different_dimension_same_annotation,
            parse_unit!("2g{seed}"),
            parse_unit!("2m{tree}")
        );
    }

    mod factor_atom_exponent {
        use super::*;

        validate_same_unit!(parse_unit!("2m2"));

        validate_same_unit_different_atom_same_dimension!(
            parse_unit!("2m2"), parse_unit!("2[in_i]2"),
            add_values: 1.000_645_16,  1_551.003_100_006_199_7,
            sub_values: 0.999_354_84, -1_549.003_100_006_199_7
        );

        validate_same_unit_different_prefix!(
            parse_unit!("2m2"), parse_unit!("2dm2"),
            add_values: 1.01, 101.0,
            sub_values: 0.99, -99.0
        );

        validate_same_unit_different_factor!(
            parse_unit!("2m2"), parse_unit!("4m2"),
            add_values:  5.0, 1.25,
            sub_values: -3.0, 0.75
        );

        validate_dimless_different_exponent!(
            parse_unit!("2[ppth]2"), parse_unit!("2[ppth]3"),
            add_values: 1.002,  501.0,
            sub_values: 0.998, -499.0
        );

        validate_ok!(
            validate_different_atom_different_dimension_different_exponent,
            parse_unit!("2cm3"),  parse_unit!("2L"),
            add_values:  251.0, 1.004,
            sub_values: -249.0, 0.996
        );

        validate_different_atom_different_dimension!(parse_unit!("2g2"), parse_unit!("2m2"));
        validate_same_unit_different_exponent!(parse_unit!("2m2"), parse_unit!("2m3"));

        validate_is_error!(
            validate_different_atom_different_dimension_same_exponent,
            parse_unit!("2m2"),
            parse_unit!("2s2")
        );
    }

    mod factor_atom_exponent_annotation {
        use super::*;

        validate_same_unit!(parse_unit!("2m2{stuff}"));

        validate_same_unit_different_atom_same_dimension!(
            parse_unit!("2[in_i]2"), parse_unit!("2[ft_i]2"),
            add_values:  145.0, 1.006_944_444_444_444_4,
            sub_values: -143.0, 0.993_055_555_555_555_6
        );

        validate_same_unit_different_prefix!(
            parse_unit!("2m2{stuff}"), parse_unit!("2dm2{stuff}"),
            add_values: 1.01, 101.0,
            sub_values: 0.99, -99.0
        );

        validate_same_unit_different_factor!(
            parse_unit!("2m2{stuff}"), parse_unit!("4m2{stuff}"),
            add_values:  5.0, 1.25,
            sub_values: -3.0, 0.75
        );

        validate_dimless_different_exponent!(
            parse_unit!("2[ppth]2{things}"), parse_unit!("2[ppth]3{things}"),
            add_values: 1.002,  501.0,
            sub_values: 0.998, -499.0
        );

        validate_same_unit_different_annotation!(
            parse_unit!("2m2{stuff}"),
            parse_unit!("2m2{things}")
        );

        validate_different_atom_different_dimension!(
            parse_unit!("2g2{stuff}"),
            parse_unit!("2m2{stuff}")
        );

        validate_same_unit_different_exponent!(
            parse_unit!("2m2{stuff}"),
            parse_unit!("2m3{stuff}")
        );
    }

    mod factor_prefix_atom {
        use super::*;

        validate_same_unit!(parse_unit!("2km"));

        validate_same_unit_different_atom_same_dimension!(
            parse_unit!("2cg"), parse_unit!("2nt"),
            add_values: 1.1, 11.0,
            sub_values: 0.9, -9.0
        );

        validate_same_unit_different_prefix!(
            parse_unit!("2cm"), parse_unit!("2dm"),
            add_values: 11.0, 1.1,
            sub_values: -9.0, 0.9
        );

        validate_same_unit_different_factor!(
            parse_unit!("2m2{stuff}"), parse_unit!("4m2{stuff}"),
            add_values:  5.0, 1.25,
            sub_values: -3.0, 0.75
        );

        validate_different_atom_different_dimension!(parse_unit!("2km"), parse_unit!("2kg"));
        validate_same_unit_different_exponent!(parse_unit!("2km"), parse_unit!("2km2"));
    }

    // NOTE: There are 0 dimless atoms that are also metric, thus no dimless tests here.
    mod factor_prefix_atom_annotation {
        use super::*;

        validate_same_unit!(parse_unit!("2kg{tree}"));

        validate_same_unit_different_atom_same_dimension!(
            parse_unit!("2cg{goop}"), parse_unit!("2nt{goop}"),
            add_values: 1.1, 11.0,
            sub_values: 0.9, -9.0
        );

        validate_same_unit_different_prefix!(
            parse_unit!("2cm{tree}"), parse_unit!("2dm{tree}"),
            add_values: 11.0, 1.1,
            sub_values: -9.0, 0.9
        );

        validate_same_unit_different_factor!(
            parse_unit!("2cg{stuff}"), parse_unit!("4cg{stuff}"),
            add_values:  3.0, 1.5,
            sub_values: -1.0, 0.5
        );

        validate_ok!(
            validate_factor_prefix_different_atom_same_dimension_same_annotation,
            parse_unit!("2kg{seed}"), parse_unit!("2nt{seed}"),
            add_values: 1.000_001,  1_000_001.0,
            sub_values: 0.999_999, -999_999.0
        );

        validate_same_unit_different_annotation!(
            parse_unit!("2kg{stuff}"),
            parse_unit!("2kg{things}")
        );

        validate_different_atom_different_dimension!(
            parse_unit!("2km{foo}"),
            parse_unit!("2kg{foo}")
        );

        validate_same_unit_different_exponent!(parse_unit!("2kg{tree}"), parse_unit!("2kg2{tree}"));

        validate_is_error!(
            validate_different_factor_different_prefix_different_atom_same_dimension_different_annotation,
            parse_unit!("2kg{seed}"), parse_unit!("3nt{tree}")
        );

        validate_is_error!(
            validate_same_factor_same_prefix_different_atom_different_dimension_same_annotation,
            parse_unit!("2kg{seed}"),
            parse_unit!("2km{seed}")
        );
    }

    // NOTE: There are 0 dimless atoms that are also metric, thus no dimless tests here.
    mod factor_prefix_atom_exponent {
        use super::*;

        validate_same_unit!(parse_unit!("2cm2"));

        validate_same_unit_different_atom_same_dimension!(
            parse_unit!("2cg{goop}"), parse_unit!("2nt{goop}"),
            add_values: 1.1, 11.0,
            sub_values: 0.9, -9.0
        );

        validate_same_unit_different_prefix!(
            parse_unit!("2cm2"), parse_unit!("2dm2"),
            add_values: 101.000_000_000_000_1, 1.01,
            sub_values: -99.000_000_000_000_1, 0.99
        );

        validate_same_unit_different_factor!(
            parse_unit!("2cm2"), parse_unit!("4cm2"),
            add_values:  5.0, 1.25,
            sub_values: -3.0, 0.75
        );

        validate_ok!(
            validate_different_atom_same_dimension_same_exponent,
            parse_unit!("2000kg2"), parse_unit!("2mt2"),
            add_values: 1.000_001, 1_000_001.0,
            sub_values: 0.999_999, -999_999.0
        );

        validate_ok!(
            validate_different_atom_different_dimension_different_exponent,
            parse_unit!("2cm3"), parse_unit!("2L"),
            add_values: 251.0,  1.004,
            sub_values: -249.0, 0.996
        );

        validate_different_atom_different_dimension!(parse_unit!("2km2"), parse_unit!("2kg2"));
        validate_same_unit_different_exponent!(parse_unit!("2kg2"), parse_unit!("2kg3"));

        validate_is_error!(
            validate_different_atom_different_dimension_same_exponent,
            parse_unit!("2km2"),
            parse_unit!("2ms2")
        );
    }

    // NOTE: There are 0 dimless atoms that are also metric, thus no dimless tests here.
    mod factor_prefix_atom_exponent_annotation {
        use super::*;

        validate_same_unit!(parse_unit!("2cm2{things}"));

        validate_same_unit_different_atom_same_dimension!(
            parse_unit!("2cg2{goop}"), parse_unit!("2nt2{goop}"),
            add_values: 1.01, 101.000_000_000_000_01,
            sub_values: 0.99, -99.000_000_000_000_01
        );

        validate_same_unit_different_prefix!(
            parse_unit!("2cm2{goop}"), parse_unit!("2dm2{goop}"),
            add_values: 101.000_000_000_000_1, 1.01,
            sub_values: -99.000_000_000_000_1, 0.99
        );

        validate_same_unit_different_factor!(
            parse_unit!("2cm2{things}"), parse_unit!("4cm2{things}"),
            add_values:  5.0, 1.25,
            sub_values: -3.0, 0.75
        );

        validate_same_unit_different_annotation!(
            parse_unit!("2cm2{stuff}"),
            parse_unit!("2cm2{things}")
        );

        validate_different_atom_different_dimension!(
            parse_unit!("2km2{foo}"),
            parse_unit!("2kg2{foo}")
        );
        validate_same_unit_different_exponent!(parse_unit!("2kg2{foo}"), parse_unit!("2kg3{foo}"));
    }
}
