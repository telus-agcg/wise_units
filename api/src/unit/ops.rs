use super::term_reducing;
use crate::{invert::ToInverse, Term, Unit};
use std::ops::{Div, Mul};

//-----------------------------------------------------------------------------
// impl Div
//-----------------------------------------------------------------------------
fn divide_terms(lhs: &[Term], rhs: &[Term]) -> Vec<Term> {
    let mut terms = Vec::with_capacity(lhs.len() + rhs.len());
    terms.extend_from_slice(lhs);

    for term in rhs.iter() {
        terms.push(term.to_inverse());
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
    use super::*;
    use std::str::FromStr;

    lazy_static::lazy_static! {
        static ref ACRE: Unit = Unit::from_str("[acr_us]").unwrap();
        static ref METER: Unit = Unit::from_str("m").unwrap();
        static ref KILOMETER: Unit = Unit::from_str("km").unwrap();
        static ref SEED: Unit = Unit::from_str("{seed}").unwrap();
        static ref UNITY: Unit = Unit::from_str("1").unwrap();
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn validate_div() {
        let expected = Unit::from_str("m/km").unwrap();
        assert_eq!(&*METER / &*KILOMETER, expected);

        let unit = Unit::from_str("10m").unwrap();
        let other = Unit::from_str("20m").unwrap();
        let expected = Unit::from_str("10m/20m").unwrap();
        assert_eq!(unit / other, expected);

        assert_eq!(&*SEED / &*SEED, *UNITY);
        assert_eq!(&*UNITY / &*SEED, Unit::from_str("/{seed}").unwrap());
        assert_eq!(&*SEED / &*ACRE, Unit::from_str("{seed}/[acr_us]").unwrap());
    }

    #[test]
    fn validate_mul() {
        let expected = Unit::from_str("m.km").unwrap();
        assert_eq!(&*METER * &*KILOMETER, expected);

        let unit = Unit::from_str("10m").unwrap();
        let other = Unit::from_str("20m").unwrap();
        let expected = Unit::from_str("10m.20m").unwrap();
        assert_eq!(unit * other, expected);

        let per_seed = Unit::from_str("/{seed}").unwrap();
        assert_eq!(&*SEED * &per_seed, *UNITY);

        let seed_per_acre = Unit::from_str("{seed}/[acr_us]").unwrap();
        assert_eq!(seed_per_acre * &*ACRE, *SEED);
    }
}
