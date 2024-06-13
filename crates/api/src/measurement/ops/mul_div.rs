use std::ops::{Div, Mul};

use crate::{convertible::Convertible, measurement::Measurement};

//          ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
//          ┃                        impl Mul                         ┃
//          ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
#[cfg_attr(
    feature = "cffi",
    ffi_common::derive::expose_fn(extend_type(Measurement))
)]
fn mul_measurements(lhs: &Measurement, rhs: &Measurement) -> Measurement {
    let converted_rhs = rhs.convert_to(&lhs.unit);
    let actual_rhs = converted_rhs.as_ref().unwrap_or(rhs);
    let new_value = lhs.value * actual_rhs.value;
    let new_unit = &lhs.unit * &actual_rhs.unit;

    Measurement {
        value: new_value,
        unit: new_unit,
    }
}

// ╭───────────────╮
// │ Owned * Owned │
// ╰───────────────╯
impl Mul for Measurement {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        let actual_rhs = other.convert_to(&self.unit).unwrap_or(other);

        Self {
            value: self.value * actual_rhs.value,
            unit: self.unit * actual_rhs.unit,
        }
    }
}

// ╭──────────────────╮
// │ Owned * Borrowed │
// ╰──────────────────╯
impl<'a> Mul<&'a Self> for Measurement {
    type Output = Self;

    #[inline]
    fn mul(self, other: &'a Self) -> Self::Output {
        let converted_rhs = other.convert_to(&self.unit);
        let actual_rhs = converted_rhs.as_ref().unwrap_or(other);

        Self {
            value: self.value * actual_rhs.value,
            unit: self.unit * &actual_rhs.unit,
        }
    }
}

// ╭─────────────────────╮
// │ Borrowed * Borrowed │
// ╰─────────────────────╯
impl<'a> Mul for &'a Measurement {
    type Output = Measurement;

    #[inline]
    fn mul(self, other: &'a Measurement) -> Self::Output {
        let converted_rhs = other.convert_to(&self.unit);
        let actual_rhs = converted_rhs.as_ref().unwrap_or(other);

        Measurement {
            value: self.value * actual_rhs.value,
            unit: &self.unit * &actual_rhs.unit,
        }
    }
}

// ╭──────────────────╮
// │ Borrowed * Owned │
// ╰──────────────────╯
impl<'a> Mul<Measurement> for &'a Measurement {
    type Output = Measurement;

    #[inline]
    fn mul(self, other: Measurement) -> Self::Output {
        let actual_rhs = other.convert_to(&self.unit).unwrap_or(other);

        Measurement {
            value: self.value * actual_rhs.value,
            unit: &self.unit * actual_rhs.unit,
        }
    }
}

/// Multiplies the `Measurement`'s scalar by `other` and returns a new
/// `Measurement`.
///
impl Mul<f64> for Measurement {
    type Output = Self;

    #[inline]
    fn mul(self, other: f64) -> Self::Output {
        Self {
            value: self.value * other,
            unit: self.unit,
        }
    }
}

#[cfg_attr(feature = "cffi", ffi_common::derive::expose_impl)]
impl<'a> Mul<f64> for &'a Measurement {
    type Output = Measurement;

    #[inline]
    fn mul(self, other: f64) -> Self::Output {
        Measurement {
            value: self.value * other,
            unit: self.unit.clone(),
        }
    }
}

//          ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
//          ┃                        impl Div                         ┃
//          ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
#[cfg_attr(
    feature = "cffi",
    ffi_common::derive::expose_fn(extend_type(Measurement))
)]
fn div_measurements(lhs: &Measurement, rhs: &Measurement) -> Measurement {
    let converted_rhs = rhs.convert_to(&lhs.unit);
    let actual_rhs = converted_rhs.as_ref().unwrap_or(rhs);
    let new_value = lhs.value / actual_rhs.value;
    let new_unit = &lhs.unit / &actual_rhs.unit;

    Measurement {
        value: new_value,
        unit: new_unit,
    }
}

// ╭───────────────╮
// │ Owned / Owned │
// ╰───────────────╯
impl Div for Measurement {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self::Output {
        let actual_rhs = other.convert_to(&self.unit).unwrap_or(other);

        Self {
            value: self.value / actual_rhs.value,
            unit: self.unit / actual_rhs.unit,
        }
    }
}

// ╭─────────────────────╮
// │ Borrowed * Borrowed │
// ╰─────────────────────╯
impl<'a> Div for &'a Measurement {
    type Output = Measurement;

    #[inline]
    fn div(self, other: &'a Measurement) -> Self::Output {
        let converted_rhs = other.convert_to(&self.unit);
        let actual_rhs = converted_rhs.as_ref().unwrap_or(other);

        Measurement {
            value: self.value / actual_rhs.value,
            unit: &self.unit / &actual_rhs.unit,
        }
    }
}

// ╭──────────────────╮
// │ Owned * Borrowed │
// ╰──────────────────╯
impl<'a> Div<&'a Self> for Measurement {
    type Output = Self;

    #[inline]
    fn div(self, other: &'a Self) -> Self::Output {
        let converted_rhs = other.convert_to(&self.unit);
        let actual_rhs = converted_rhs.as_ref().unwrap_or(other);

        Self {
            value: self.value / actual_rhs.value,
            unit: self.unit / &actual_rhs.unit,
        }
    }
}

// ╭──────────────────╮
// │ Borrowed * Owned │
// ╰──────────────────╯
impl<'a> Div<Measurement> for &'a Measurement {
    type Output = Measurement;

    #[inline]
    fn div(self, other: Measurement) -> Self::Output {
        let actual_rhs = other.convert_to(&self.unit).unwrap_or(other);

        Measurement {
            value: self.value / actual_rhs.value,
            unit: &self.unit / actual_rhs.unit,
        }
    }
}

/// Divides the `Measurement`'s scalar by `other` and returns a new
/// `Measurement`.
///
impl Div<f64> for Measurement {
    type Output = Self;

    #[inline]
    fn div(self, other: f64) -> Self::Output {
        Self {
            value: self.value / other,
            unit: self.unit,
        }
    }
}

#[cfg_attr(feature = "cffi", ffi_common::derive::expose_impl)]
impl<'a> Div<f64> for &'a Measurement {
    type Output = Measurement;

    #[inline]
    fn div(self, other: f64) -> Self::Output {
        Measurement {
            value: self.value / other,
            unit: self.unit.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{testing::const_units::l2::METER_SQUARED, unit::UNITY};

    use super::*;

    macro_rules! assert_eq_op {
        ($lhs:expr, $op:expr, $rhs:expr, $expected:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;

            // borrowed * borrowed
            pretty_assert_field_eq!(*$op(&lhs, &rhs).sort_terms(), &$expected);

            // borrowed * owned
            pretty_assert_field_eq!(*$op(&lhs, rhs.clone()).sort_terms(), $expected);

            // owned * borrowed
            pretty_assert_field_eq!(*$op(lhs.clone(), &rhs).sort_terms(), $expected);

            // owned * owned
            pretty_assert_field_eq!(*$op(lhs.clone(), rhs.clone()).sort_terms(), $expected);
        }};
    }

    macro_rules! assert_eq_mul {
        ($lhs:expr, $rhs:expr, $expected:expr) => {{
            assert_eq_op!($lhs, Mul::mul, $rhs, $expected);
        }};
    }

    macro_rules! assert_eq_div {
        ($lhs:expr, $rhs:expr, $expected:expr) => {{
            assert_eq_op!($lhs, Div::div, $rhs, $expected);
        }};
    }

    macro_rules! validate_ok {
        (
            $test_name:ident,
            $lhs_unit:expr, $rhs_unit:expr,
            mul_values: $expected_mul_lhs_value:expr, $expected_mul_rhs_value:expr,
            mul_unit: $mul_unit:expr,
            div_values: $expected_div_lhs_value:expr, $expected_div_rhs_value:expr,
            div_unit1: $div_unit1:expr,
            div_unit2: $div_unit2:expr
        ) => {
            #[test]
            fn $test_name() {
                let lhs = Measurement::new(1.0, $lhs_unit);
                let rhs = Measurement::new(2.0, $rhs_unit);

                eprintln!("1mul.");
                assert_eq_mul!(
                    lhs.clone(),
                    rhs.clone(),
                    Measurement::new($expected_mul_lhs_value, $mul_unit).sort_terms()
                );
                eprintln!("1div.");
                assert_eq_div!(
                    lhs.clone(),
                    rhs.clone(),
                    Measurement::new($expected_div_lhs_value, $div_unit1).sort_terms()
                );

                eprintln!("2mul.");
                assert_eq_mul!(
                    rhs.clone(),
                    lhs.clone(),
                    Measurement::new($expected_mul_rhs_value, $mul_unit).sort_terms()
                );
                eprintln!("2div.");
                assert_eq_div!(
                    rhs,
                    lhs,
                    Measurement::new($expected_div_rhs_value, $div_unit2).sort_terms()
                );
                eprintln!("done");
            }
        };

        (
            $test_name:ident,
            $lhs_unit:expr, $rhs_unit:expr,
            mul_values: $expected_mul_lhs_value:expr, $expected_mul_rhs_value:expr,
            mul_unit: $mul_unit:expr,
            div_values: $expected_div_lhs_value:expr, $expected_div_rhs_value:expr,
            div_unit: $div_unit:expr
        ) => {
            validate_ok!(
                $test_name,
                $lhs_unit, $rhs_unit,
                mul_values: $expected_mul_lhs_value, $expected_div_rhs_value,
                mul_unit: $mul_unit,
                div_values: $expected_div_lhs_value, $expected_div_rhs_value,
                div_unit1: $div_unit,
                div_unit2: $div_unit
            );
        };
    }

    macro_rules! validate_same_unit {
        (
            $test_name:ident,
            $input_unit:expr,
            mul_unit: $mul_unit:expr,
            div_unit: $div_unit:expr
        ) => {
            validate_ok!(
                $test_name,
                $input_unit.clone(), $input_unit.clone(),
                mul_values: 2.0, 2.0,
                mul_unit: $mul_unit,
                div_values: 0.5, 2.0,
                div_unit: $div_unit
            );
        };

        (
            $input_unit:expr,
            mul_unit: $mul_unit:expr,
            div_unit: $div_unit:expr
        ) => {
            validate_same_unit!(
                validate_same_unit,
                $input_unit,
                mul_unit: $mul_unit,
                div_unit: $div_unit
            );
        };
    }

    macro_rules! validate_dim_and_dimless {
        (
            $test_name:ident,
            $lhs_unit:expr, $rhs_unit:expr,
            mul_values: $expected_mul_lhs_value:expr, $expected_mul_rhs_value:expr,
            mul_unit: $mul_unit:expr,
            div_values:  $expected_div_lhs_value:expr, $expected_div_rhs_value:expr,
            div_unit1: $div_unit1:expr,
            div_unit2: $div_unit2:expr
        ) => {
            validate_ok!(
                $test_name,
                $lhs_unit, $rhs_unit,
                mul_values: $expected_mul_lhs_value, $expected_mul_rhs_value,
                mul_unit: $mul_unit,
                div_values: $expected_div_lhs_value, $expected_div_rhs_value,
                div_unit1: $div_unit1,
                div_unit2: $div_unit2
            );
        };

        (
            $lhs_unit:expr, $rhs_unit:expr,
            mul_values: $expected_mul_lhs_value:expr, $expected_mul_rhs_value:expr,
            mul_unit: $mul_unit:expr,
            div_values:  $expected_div_lhs_value:expr, $expected_div_rhs_value:expr,
            div_unit1: $div_unit1:expr,
            div_unit2: $div_unit2:expr
        ) => {
            validate_ok!(
                validate_dim_and_dimless,
                $lhs_unit, $rhs_unit,
                mul_values: $expected_mul_lhs_value, $expected_mul_rhs_value,
                mul_unit: $mul_unit,
                div_values: $expected_div_lhs_value, $expected_div_rhs_value,
                div_unit1: $div_unit1,
                div_unit2: $div_unit2
            );
        };

        (
            $lhs_unit:expr, $rhs_unit:expr,
            mul_values: $expected_mul_lhs_value:expr, $expected_mul_rhs_value:expr,
            mul_unit: $mul_unit:expr,
            div_values:  $expected_div_lhs_value:expr, $expected_div_rhs_value:expr,
            div_unit: $div_unit:expr
        ) => {
            validate_ok!(
                validate_dim_and_dimless,
                $lhs_unit, $rhs_unit,
                mul_values: $expected_mul_lhs_value, $expected_mul_rhs_value,
                mul_unit: $mul_unit,
                div_values: $expected_div_lhs_value, $expected_div_rhs_value,
                div_unit: $div_unit
            );
        }
    }

    mod annotation {
        use super::*;

        // NOTE: The resulting unit when dividing here should be `{tree}` since that is just
        // an annotation; no different than if the unit was `1`.
        //
        validate_same_unit!(
            parse_unit!("{tree}"),
            mul_unit: unit!(term!(factor: 1, exponent: 2, annotation: "tree")),
            div_unit: UNITY
        );

        // TODO: https://telusagriculture.atlassian.net/browse/NAUM-122
        validate_dim_and_dimless!(
            validate_same_annotation,
            parse_unit!("{stuff}"), parse_unit!("m{stuff}"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("m{stuff}.{stuff}"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("{stuff}/m{stuff}"),
            div_unit2: parse_unit!("m{stuff}/{stuff}")
        );

        // TODO: https://telusagriculture.atlassian.net/browse/NAUM-122
        validate_dim_and_dimless!(
            validate_different_annotation,
            parse_unit!("{tree}"), parse_unit!("m{stuff}"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("m{stuff}.{tree}"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("m-1{stuff}.{tree}"),
            div_unit2: unit!(
                term!(factor: 1, exponent: -1, annotation: "tree"),
                term!(Meter, annotation: "stuff")
            )
        );
    }

    mod atom {
        use super::*;

        validate_same_unit!(
            parse_unit!("m"),
            mul_unit: METER_SQUARED,
            div_unit: UNITY
        );

        validate_same_unit!(
            validate_same_dimless,
            parse_unit!("[ppth]"),
            mul_unit: parse_unit!("[ppth]2"),
            div_unit: UNITY
        );

        validate_dim_and_dimless!(
            parse_unit!("[ppth]"), parse_unit!("m"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("[ppth].m"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("[ppth]/m"),
            div_unit2: parse_unit!("m/[ppth]")
        );
    }

    mod atom_annotation {
        use super::*;

        validate_same_unit!(
            parse_unit!("m{foo}"),
            mul_unit: parse_unit!("m2{foo}"),
            div_unit: UNITY
        );

        validate_same_unit!(
            validate_same_dimless,
            parse_unit!("[ppth]{foo}"),
            mul_unit: parse_unit!("[ppth]2{foo}"),
            div_unit: UNITY
        );

        validate_dim_and_dimless!(
            validate_same_annotation,
            parse_unit!("[ppth]{foo}"), parse_unit!("m{foo}"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("[ppth]{foo}.m{foo}"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("[ppth]{foo}/m{foo}"),
            div_unit2: parse_unit!("m{foo}/[ppth]{foo}")
        );

        validate_dim_and_dimless!(
            validate_different_annotation,
            parse_unit!("[ppth]{foo}"), parse_unit!("m{bar}"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("[ppth]{foo}.m{bar}"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("[ppth]{foo}/m{bar}"),
            div_unit2: parse_unit!("m{bar}/[ppth]{foo}")
        );
    }

    mod atom_exponent {
        use super::*;

        validate_same_unit!(
            parse_unit!("m2"),
            mul_unit: parse_unit!("m4"),
            div_unit: UNITY
        );

        validate_same_unit!(
            validate_same_dimless,
            parse_unit!("[ppth]2"),
            mul_unit: parse_unit!("[ppth]4"),
            div_unit: UNITY
        );

        validate_dim_and_dimless!(
            parse_unit!("m2"), parse_unit!("[ppth]3"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("m2.[ppth]3"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("m2/[ppth]3"),
            div_unit2: parse_unit!("[ppth]3/m2")
        );
    }

    mod atom_exponent_annotation {
        use super::*;

        // NOTE: The resulting unit when dividing here should be `{dirt}` since that is just
        // an annotation; no different than if the unit was `1`.
        //
        validate_same_unit!(
            parse_unit!("m2{dirt}"),
            mul_unit: parse_unit!("m4{dirt}"),
            div_unit: UNITY
        );

        validate_same_unit!(
            validate_same_dimless,
            parse_unit!("[ppth]2{dirt}"),
            mul_unit: parse_unit!("[ppth]4{dirt}"),
            div_unit: UNITY
        );

        validate_dim_and_dimless!(
            validate_same_annotation,
            parse_unit!("m2{dirt}"), parse_unit!("[ppth]{dirt}"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("m2{dirt}.[ppth]{dirt}"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("m2{dirt}/[ppth]{dirt}"),
            div_unit2: parse_unit!("[ppth]{dirt}/m2{dirt}")
        );

        validate_dim_and_dimless!(
            validate_different_annotation,
            parse_unit!("m2{dirt}"), parse_unit!("[ppth]{foo}"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("m2{dirt}.[ppth]{foo}"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("m2{dirt}/[ppth]{foo}"),
            div_unit2: parse_unit!("[ppth]{foo}/m2{dirt}")
        );
    }

    // NOTE: Dimless atoms not tested here because no dimless atoms are metric, and that's
    // a requirement for units to have a prefix.
    mod prefix_atom {
        use super::*;

        validate_same_unit!(
            parse_unit!("km"),
            mul_unit: parse_unit!("km2"),
            div_unit: UNITY
        );
    }

    // NOTE: Dimless atoms not tested here because no dimless atoms are metric, and that's
    // a requirement for units to have a prefix.
    mod prefix_atom_annotation {
        use super::*;

        // NOTE: The resulting unit when dividing here should be `{dirt}` since that is just
        // an annotation; no different than if the unit was `1`.
        //
        validate_same_unit!(
            parse_unit!("km{dirt}"),
            mul_unit: parse_unit!("km2{dirt}"),
            div_unit: UNITY
        );
    }

    // NOTE: Dimless atoms not tested here because no dimless atoms are metric, and that's
    // a requirement for units to have a prefix.
    mod prefix_atom_exponent {
        use super::*;

        validate_same_unit!(
            parse_unit!("km2"),
            mul_unit: parse_unit!("km4"),
            div_unit: UNITY
        );
    }

    // NOTE: Dimless atoms not tested here because no dimless atoms are metric, and that's
    // a requirement for units to have a prefix.
    mod prefix_atom_exponent_annotation {
        use super::*;

        // NOTE: The resulting unit when dividing here should be `{dirt}` since that is just
        // an annotation; no different than if the unit was `1`.
        //
        validate_same_unit!(
            parse_unit!("km2{dirt}"),
            mul_unit: parse_unit!("km4{dirt}"),
            div_unit: UNITY
        );
    }

    mod factor {
        use super::*;

        validate_same_unit!(
            parse_unit!("2"),
            mul_unit: unit!(term!(factor: 2, exponent: 2)),
            div_unit: UNITY
        );

        validate_dim_and_dimless!(
            parse_unit!("2"), parse_unit!("2m"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("2.2m"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("2/2m"),
            div_unit2: parse_unit!("2m/2")
        );
    }

    mod factor_annotation {
        use super::*;

        // NOTE: The resulting unit when dividing here should be `{rabbit}` since that is just
        // an annotation; no different than if the unit was `1`.
        //
        validate_same_unit!(
            parse_unit!("2{rabbit}"),
            mul_unit: unit!(term!(factor: 2, exponent: 2, annotation: "rabbit")),
            div_unit: UNITY
        );

        validate_dim_and_dimless!(
            validate_same_annotation,
            parse_unit!("2{foo}"), parse_unit!("3m{foo}"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("2{foo}.3m{foo}"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("2{foo}/3m{foo}"),
            div_unit2: parse_unit!("3m{foo}/2{foo}")
        );

        validate_dim_and_dimless!(
            validate_different_annotation,
            parse_unit!("2{foo}"), parse_unit!("3m{bar}"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("2{foo}.3m{bar}"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("2{foo}/3m{bar}"),
            div_unit2: parse_unit!("3m{bar}/2{foo}")
        );
    }

    mod factor_exponent {
        use super::*;

        validate_same_unit!(
            unit!(term!(factor: 2, exponent: 2)),
            mul_unit: unit!(term!(factor: 2, exponent: 4)),
            div_unit: UNITY
        );

        validate_dim_and_dimless!(
            unit!(term!(factor: 2, exponent: 2)), parse_unit!("3m3"),
            mul_values: 2.0, 2.0,
            mul_unit: unit!(term!(factor: 2, exponent: 2), term!(Meter, factor: 3, exponent: 3)),
            div_values: 0.5, 2.0,
            div_unit1: unit!(term!(factor: 2, exponent: 2), term!(Meter, factor: 3, exponent: -3)),
            div_unit2: unit!(term!(Meter, factor: 3, exponent: 3), term!(factor: 2, exponent: -2))
        );
    }

    mod factor_exponent_annotation {
        use super::*;

        // NOTE: The resulting unit when dividing here should be `{rabbit}` since that is just
        // an annotation; no different than if the unit was `1`.
        //
        validate_same_unit!(
            unit!(term!(factor: 2, exponent: 2, annotation: "rabbit")),
            mul_unit: unit!(term!(factor: 2, exponent: 4, annotation: "rabbit")),
            div_unit: UNITY
        );

        validate_dim_and_dimless!(
            validate_same_annotation,
            unit!(term!(factor: 2, exponent: 2, annotation: "rabbit")), parse_unit!("3m3{rabbit}"),
            mul_values: 2.0, 2.0,
            mul_unit: unit!(
                term!(factor: 2, exponent: 2, annotation: "rabbit"),
                term!(Meter, factor: 3, exponent: 3, annotation: "rabbit")
            ),
            div_values: 0.5, 2.0,
            div_unit1: unit!(
                term!(factor: 2, exponent: 2, annotation: "rabbit"),
                term!(Meter, factor: 3, exponent: -3, annotation: "rabbit")
            ),
            div_unit2: unit!(
                term!(Meter, factor: 3, exponent: 3, annotation: "rabbit"),
                term!(factor: 2, exponent: -2, annotation: "rabbit")
            )
        );

        validate_dim_and_dimless!(
            validate_different_annotation,
            unit!(term!(factor: 2, exponent: 2, annotation: "rabbit")), parse_unit!("3m3{fox}"),
            mul_values: 2.0, 2.0,
            mul_unit: unit!(
                term!(factor: 2, exponent: 2, annotation: "rabbit"),
                term!(Meter, factor: 3, exponent: 3, annotation: "fox")
            ),
            div_values: 0.5, 2.0,
            div_unit1: unit!(
                term!(factor: 2, exponent: 2, annotation: "rabbit"),
                term!(Meter, factor: 3, exponent: -3, annotation: "fox")
            ),
            div_unit2: unit!(
                term!(Meter, factor: 3, exponent: 3, annotation: "fox"),
                term!(factor: 2, exponent: -2, annotation: "rabbit")
            )
        );
    }

    mod factor_atom {
        use super::*;

        validate_same_unit!(
            parse_unit!("2m"),
            mul_unit: parse_unit!("2m2"),
            div_unit: UNITY
        );

        validate_same_unit!(
            validate_same_dimless,
            parse_unit!("2[ppth]"),
            mul_unit: parse_unit!("2[ppth]2"),
            div_unit: UNITY
        );

        validate_dim_and_dimless!(
            parse_unit!("2m"), parse_unit!("3[ppth]"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("2m.3[ppth]"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("2m/3[ppth]"),
            div_unit2: parse_unit!("3[ppth]/2m")
        );
    }

    mod factor_atom_annotation {
        use super::*;

        // NOTE: The resulting unit when dividing here should be `{raisin}` since that is just
        // an annotation; no different than if the unit was `1`.
        //
        validate_same_unit!(
            parse_unit!("2m{raisin}"),
            mul_unit: parse_unit!("2m2{raisin}"),
            div_unit: UNITY
        );

        validate_same_unit!(
            validate_same_dimless,
            parse_unit!("2[ppth]{foo}"),
            mul_unit: parse_unit!("2[ppth]2{foo}"),
            div_unit: UNITY
        );

        validate_dim_and_dimless!(
            validate_same_annotation,
            parse_unit!("2m{raisin}"), parse_unit!("3[ppth]{raisin}"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("2m{raisin}.3[ppth]{raisin}"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("2m{raisin}/3[ppth]{raisin}"),
            div_unit2: parse_unit!("3[ppth]{raisin}/2m{raisin}")
        );

        validate_dim_and_dimless!(
            validate_different_annotation,
            parse_unit!("2m{raisin}"), parse_unit!("3[ppth]{apple}"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("2m{raisin}.3[ppth]{apple}"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("2m{raisin}/3[ppth]{apple}"),
            div_unit2: parse_unit!("3[ppth]{apple}/2m{raisin}")
        );
    }

    mod factor_atom_exponent {
        use super::*;

        validate_same_unit!(
            parse_unit!("2m2"),
            mul_unit: parse_unit!("2m4"),
            div_unit: UNITY
        );

        validate_same_unit!(
            validate_same_dimless,
            parse_unit!("2[ppth]2"),
            mul_unit: parse_unit!("2[ppth]4"),
            div_unit: UNITY
        );

        validate_dim_and_dimless!(
            parse_unit!("2m2"), parse_unit!("3[ppth]3"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("2m2.3[ppth]3"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("2m2/3[ppth]3"),
            div_unit2: parse_unit!("3[ppth]3/2m2")
        );
    }

    mod factor_atom_exponent_annotation {
        use super::*;

        // NOTE: The resulting unit when dividing here should be `{dirt}` since that is just
        // an annotation; no different than if the unit was `1`.
        //
        validate_same_unit!(
            parse_unit!("2m2{dirt}"),
            mul_unit: parse_unit!("2m4{dirt}"),
            div_unit: UNITY
        );

        validate_same_unit!(
            validate_same_dimless,
            parse_unit!("2[ppth]2{dirt}"),
            mul_unit: parse_unit!("2[ppth]4{dirt}"),
            div_unit: UNITY
        );

        validate_dim_and_dimless!(
            validate_same_annotation,
            parse_unit!("2m2{dirt}"), parse_unit!("3[ppth]3{dirt}"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("2m2{dirt}.3[ppth]3{dirt}"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("2m2{dirt}/3[ppth]3{dirt}"),
            div_unit2: parse_unit!("3[ppth]3{dirt}/2m2{dirt}")
        );

        validate_dim_and_dimless!(
            validate_different_annotation,
            parse_unit!("2m2{dirt}"), parse_unit!("3[ppth]3{water}"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("2m2{dirt}.3[ppth]3{water}"),
            div_values: 0.5, 2.0,
            div_unit1: parse_unit!("2m2{dirt}/3[ppth]3{water}"),
            div_unit2: parse_unit!("3[ppth]3{water}/2m2{dirt}")
        );
    }

    // NOTE: Dimless atoms not tested here because no dimless atoms are metric, and that's
    // a requirement for units to have a prefix.
    mod factor_prefix_atom {
        use super::*;

        validate_same_unit!(
            parse_unit!("2km"),
            mul_unit: parse_unit!("2km2"),
            div_unit: UNITY
        );
    }

    // NOTE: Dimless atoms not tested here because no dimless atoms are metric, and that's
    // a requirement for units to have a prefix.
    mod factor_prefix_atom_annotation {
        use super::*;

        // NOTE: The resulting unit when dividing here should be `{string}` since that is just
        // an annotation; no different than if the unit was `1`.
        //
        validate_same_unit!(
            parse_unit!("2km{string}"),
            mul_unit: parse_unit!("2km2{string}"),
            div_unit: UNITY
        );
    }

    // NOTE: Dimless atoms not tested here because no dimless atoms are metric, and that's
    // a requirement for units to have a prefix.
    mod factor_prefix_atom_exponent {
        use super::*;

        validate_same_unit!(
            parse_unit!("2km2"),
            mul_unit: parse_unit!("2km4"),
            div_unit: UNITY
        );
    }

    // NOTE: Dimless atoms not tested here because no dimless atoms are metric, and that's
    // a requirement for units to have a prefix.
    mod factor_prefix_atom_exponent_annotation {
        use super::*;

        // NOTE: The resulting unit when dividing here should be `{dirt}` since that is just
        // an annotation; no different than if the unit was `1`.
        //
        validate_same_unit!(
            parse_unit!("2km2{dirt}"),
            mul_unit: parse_unit!("2km4{dirt}"),
            div_unit: UNITY
        );
    }

    mod combinations {
        use super::*;

        validate_ok!(
            same_annotation_in_numerator_and_denominator,
            parse_unit!("{meow}/m2"), parse_unit!("g/{meow}"),
            mul_values: 2.0, 2.0,
            mul_unit: parse_unit!("g/m2"),
            div_values: 0.5, 2.0,
            div_unit1: unit!(
                term!(factor: 1, exponent: 2, annotation: "meow"),
                term!(Gram, exponent: -1),
                term!(Meter, exponent: -2)
            ),
            div_unit2: unit!(
                term!(factor: 1, exponent: -2, annotation: "meow"),
                term!(Gram),
                term!(Meter, exponent: 2)
            )
        );
    }
}
