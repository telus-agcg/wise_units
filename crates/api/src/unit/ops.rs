use std::ops::{Div, Mul};

use num_traits::Inv;

use crate::{Term, Unit};

use super::term_reducing;

//-----------------------------------------------------------------------------
// impl Div
//-----------------------------------------------------------------------------
fn divide_terms(lhs: &[Term], rhs: &[Term]) -> Vec<Term> {
    let mut terms = Vec::with_capacity(lhs.len() + rhs.len());
    terms.extend_from_slice(lhs);

    for term in rhs {
        terms.push(term.inv());
    }

    term_reducing::reduce_terms(&terms)
}

#[cfg_attr(feature = "cffi", ffi_common::derive::expose_fn(extend_type(Unit)))]
fn divide_units(lhs: &Unit, rhs: &Unit) -> Unit {
    Unit::new(divide_terms(&lhs.terms, &rhs.terms))
}

impl Div for Unit {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self::Output {
        divide_units(&self, &other)
    }
}

impl<'a> Div<&'a Self> for Unit {
    type Output = Self;

    #[inline]
    fn div(self, other: &'a Self) -> Self::Output {
        divide_units(&self, other)
    }
}

impl<'a> Div for &'a Unit {
    type Output = Unit;

    #[inline]
    fn div(self, other: &'a Unit) -> Self::Output {
        divide_units(self, other)
    }
}

impl<'a> Div<Unit> for &'a Unit {
    type Output = Unit;

    #[inline]
    fn div(self, other: Unit) -> Self::Output {
        divide_units(self, &other)
    }
}

//-----------------------------------------------------------------------------
// impl Mul
//-----------------------------------------------------------------------------

fn multiply_terms(lhs: &[Term], rhs: &[Term]) -> Vec<Term> {
    let mut terms = Vec::with_capacity(lhs.len() + rhs.len());

    terms.extend_from_slice(lhs);
    terms.extend_from_slice(rhs);

    term_reducing::reduce_terms(&terms)
}

#[cfg_attr(feature = "cffi", ffi_common::derive::expose_fn(extend_type(Unit)))]
fn multiply_units(lhs: &Unit, rhs: &Unit) -> Unit {
    Unit::new(multiply_terms(&lhs.terms, &rhs.terms))
}

impl Mul for Unit {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        multiply_units(&self, &other)
    }
}

impl<'a> Mul<&'a Self> for Unit {
    type Output = Self;

    #[inline]
    fn mul(self, other: &'a Self) -> Self::Output {
        multiply_units(&self, other)
    }
}

impl<'a> Mul for &'a Unit {
    type Output = Unit;

    #[inline]
    fn mul(self, other: &'a Unit) -> Self::Output {
        multiply_units(self, other)
    }
}

impl<'a> Mul<Unit> for &'a Unit {
    type Output = Unit;

    #[inline]
    fn mul(self, other: Unit) -> Self::Output {
        multiply_units(self, &other)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        testing::const_units::{ACRE, KILOMETER, METER},
        unit::UNITY,
    };

    use super::*;

    fn seed() -> Unit {
        Unit::from_str("{seed}").unwrap()
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn validate_div() {
        assert_eq!(METER / METER, UNITY);

        let expected = Unit::from_str("m/km").unwrap();
        assert_eq!(METER / KILOMETER, expected);

        let unit = Unit::from_str("10m").unwrap();
        let other = Unit::from_str("20m").unwrap();
        let expected = Unit::from_str("10m/20m").unwrap();
        assert_eq!(unit / other, expected);

        assert_eq!(seed() / seed(), UNITY);
        assert_eq!(UNITY / seed(), Unit::from_str("/{seed}").unwrap());
        assert_eq!(seed() / ACRE, Unit::from_str("{seed}/[acr_us]").unwrap());
    }

    #[test]
    fn validate_mul() {
        let expected = Unit::from_str("m.km").unwrap();
        assert_eq!(METER * KILOMETER, expected);

        let unit = Unit::from_str("10m").unwrap();
        let other = Unit::from_str("20m").unwrap();
        let expected = Unit::from_str("10m.20m").unwrap();
        assert_eq!(unit * other, expected);

        let per_seed = Unit::from_str("/{seed}").unwrap();
        assert_eq!(seed() * &per_seed, UNITY);

        let seed_per_acre = Unit::from_str("{seed}/[acr_us]").unwrap();
        assert_eq!(seed_per_acre * ACRE, seed());
    }
}
