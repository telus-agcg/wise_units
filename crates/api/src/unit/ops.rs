use std::ops::{Div, Mul};

use num_traits::Inv;

use crate::Unit;

use super::term_reducing;

//          ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
//          ┃                      impl Div                       ┃
//          ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

// ╭───────────────╮
// │ Owned / Owned │
// ╰───────────────╯
impl Div for Unit {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self::Output {
        Self::new(term_reducing::compare_and_cancel(
            &self.terms,
            other.terms.iter().map(Inv::inv).collect(),
        ))
    }
}

// ╭──────────────────╮
// │ Owned / Borrowed │
// ╰──────────────────╯
impl<'a> Div<&'a Self> for Unit {
    type Output = Self;

    #[inline]
    fn div(self, other: &'a Self) -> Self::Output {
        Self::new(term_reducing::compare_and_cancel(
            &self.terms,
            other.terms.iter().map(Inv::inv).collect(),
        ))
    }
}

// ╭─────────────────────╮
// │ Borrowed / Borrowed │
// ╰─────────────────────╯
impl<'a> Div for &'a Unit {
    type Output = Unit;

    #[inline]
    fn div(self, other: &'a Unit) -> Self::Output {
        Unit::new(term_reducing::compare_and_cancel(
            &self.terms,
            other.terms.iter().map(Inv::inv).collect(),
        ))
    }
}

// ╭──────────────────╮
// │ Borrowed / Owned │
// ╰──────────────────╯
impl<'a> Div<Unit> for &'a Unit {
    type Output = Unit;

    #[inline]
    fn div(self, other: Unit) -> Self::Output {
        Unit::new(term_reducing::compare_and_cancel(
            &self.terms,
            other.terms.iter().map(Inv::inv).collect(),
        ))
    }
}

//          ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
//          ┃                        impl Mul                         ┃
//          ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
// ╭───────────────╮
// │ Owned * Owned │
// ╰───────────────╯
impl Mul for Unit {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        Self::new(term_reducing::compare_and_cancel(
            &self.terms,
            other.terms.to_vec(),
        ))
    }
}

// ╭──────────────────╮
// │ Owned * Borrowed │
// ╰──────────────────╯
impl<'a> Mul<&'a Self> for Unit {
    type Output = Self;

    #[inline]
    fn mul(self, other: &'a Self) -> Self::Output {
        Self::new(term_reducing::compare_and_cancel(
            &self.terms,
            other.terms.to_vec(),
        ))
    }
}

// ╭─────────────────────╮
// │ Borrowed * Borrowed │
// ╰─────────────────────╯
impl<'a> Mul for &'a Unit {
    type Output = Unit;

    #[inline]
    fn mul(self, other: &'a Unit) -> Self::Output {
        Unit::new(term_reducing::compare_and_cancel(
            &self.terms,
            other.terms.to_vec(),
        ))
    }
}

// ╭──────────────────╮
// │ Borrowed * Owned │
// ╰──────────────────╯
impl<'a> Mul<Unit> for &'a Unit {
    type Output = Unit;

    #[inline]
    fn mul(self, other: Unit) -> Self::Output {
        Unit::new(term_reducing::compare_and_cancel(
            &self.terms,
            other.terms.to_vec(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        testing::const_units::{
            l1::{KILOMETER, METER},
            l1t_1::METER_PER_SECOND,
            l2::ACRE,
            t1::SECOND,
            t_1::PER_SECOND,
        },
        unit::UNITY,
    };

    use super::*;

    fn seed() -> Unit {
        Unit::from_str("{seed}").unwrap()
    }

    mod div {
        use super::*;

        macro_rules! test_div {
            ($test_name:ident: $lhs:expr, $rhs:expr => $expected:expr) => {
                #[test]
                fn $test_name() {
                    // Borrowed / Borrowed
                    {
                        let result = &$lhs / &$rhs;
                        assert_field_eq!(result, &$expected, "Actual: {:#?}", result);
                    }

                    // Owned / Borrowed
                    {
                        let result = $lhs / &$rhs;
                        assert_field_eq!(result, &$expected, "Actual: {:#?}", result);
                    }

                    // Borrowed / Owned
                    {
                        let result = &$lhs / $rhs;
                        assert_field_eq!(result, &$expected, "Actual: {:#?}", result);
                    }

                    // Owned / Owned
                    {
                        let result = $lhs / $rhs;
                        assert_field_eq!(result, &$expected, "Actual: {:#?}", result);
                    }
                }
            };
        }

        test_div!(test_atom_div_same_atom:
            METER, METER => UNITY);
        test_div!(test_atom_div_different_atom:
            METER, SECOND => METER_PER_SECOND);
        test_div!(test_atom_div_prefix_same_atom:
            METER, KILOMETER => parse_unit!("m/km"));
        test_div!(test_factor_atom_div_factor_same_atom:
            parse_unit!("10m"), parse_unit!("20m") => parse_unit!("10m/20m"));
        test_div!(test_nondim_div_same_nondim:
            seed(), seed() => UNITY);
        test_div!(test_unity_div_same_nondim:
            UNITY, seed() => Unit::new(vec![term!(factor: 1, exponent: -1, annotation: "seed")]));
        test_div!(test_nondim_div_atom:
            seed(), ACRE => parse_unit!("{seed}/[acr_us]"));
        test_div!(test_atom_div_per_atom:
            METER, PER_SECOND => parse_unit!("m.s"));
        test_div!(test_atom_div_per_atom_per_atom:
            METER, METER_PER_SECOND => parse_unit!("s"));
        test_div!(test_annotatable_div_different_annotatable:
            parse_unit!("42m{foo}"), parse_unit!("42m{bar}") => parse_unit!("42m{foo}/42m{bar}"));
    }

    mod mul {
        use super::*;

        macro_rules! test_mul {
            ($test_name:ident: $lhs:expr, $rhs:expr => $expected:expr) => {
                #[test]
                fn $test_name() {
                    // Borrowed / Borrowed
                    {
                        let result = &$lhs * &$rhs;
                        assert_field_eq!(result, &$expected, "Actual: {:#?}", result);
                    }

                    // Owned / Borrowed
                    {
                        let result = $lhs * &$rhs;
                        assert_field_eq!(result, &$expected, "Actual: {:#?}", result);
                    }

                    // Borrowed / Owned
                    {
                        let result = &$lhs * $rhs;
                        assert_field_eq!(result, &$expected, "Actual: {:#?}", result);
                    }

                    // Owned / Owned
                    {
                        let result = $lhs * $rhs;
                        assert_field_eq!(result, &$expected, "Actual: {:#?}", result);
                    }
                }
            };
        }

        test_mul!(test_atom_mul_same_atom:
            METER, METER => parse_unit!["m2"]);
        test_mul!(test_atom_mul_different_atom:
            METER, SECOND => parse_unit!["m.s"]);
        test_mul!(test_atom_mul_prefix_same_atom:
            METER, KILOMETER => parse_unit!["m.km"]);
        test_mul!(test_factor_atom_mul_factor_same_atom:
            parse_unit!("10m"), parse_unit!("20m") => parse_unit!["10m.20m"]);
        test_mul!(test_nondim_mul_same_nondim:
            seed(), seed() => unit!(term!(factor: 1, exponent: 2, annotation: "seed")));
        test_mul!(test_unity_mul_same_nondim:
            UNITY, seed() => parse_unit!["{seed}"]);
        test_mul!(test_nondim_mul_atom:
            seed(), ACRE => parse_unit!["{seed}.[acr_us]"]);
        test_mul!(test_atom_mul_per_atom:
            METER, PER_SECOND => METER_PER_SECOND);
        test_mul!(test_atom_mul_per_atom_per_atom:
            METER, METER_PER_SECOND => parse_unit!["m2/s"]);
        test_mul!(test_annotatable_mul_different_annotatable:
            parse_unit!("42m{foo}"), parse_unit!("42m-1{bar}") => parse_unit!("42m{foo}/42m{bar}"));
    }
}
