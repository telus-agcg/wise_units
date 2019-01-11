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

impl Div for Unit {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self::Output {
        divide_terms(&self.terms, &other.terms).into()
    }
}

impl<'a> Div<&'a Unit> for Unit {
    type Output = Self;

    #[inline]
    fn div(self, other: &'a Self) -> Self::Output {
        divide_terms(&self.terms, &other.terms).into()
    }
}

impl<'a> Div for &'a Unit {
    type Output = Unit;

    #[inline]
    fn div(self, other: &'a Unit) -> Self::Output {
        divide_terms(&self.terms, &other.terms).into()
    }
}

impl<'a> Div<Unit> for &'a Unit {
    type Output = Unit;

    #[inline]
    fn div(self, other: Unit) -> Self::Output {
        divide_terms(self, &other).into()
    }
}

//-----------------------------------------------------------------------------
// impl Mul
//-----------------------------------------------------------------------------
impl Mul for Unit {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        multiply_terms(&self.terms, &other.terms).into()
    }
}

impl<'a> Mul<&'a Unit> for Unit {
    type Output = Self;

    #[inline]
    fn mul(self, other: &'a Self) -> Self::Output {
        multiply_terms(&self.terms, &other.terms).into()
    }
}

impl<'a> Mul for &'a Unit {
    type Output = Unit;

    #[inline]
    fn mul(self, other: &'a Unit) -> Self::Output {
        multiply_terms(&self.terms, &other.terms).into()
    }
}

impl<'a> Mul<Unit> for &'a Unit {
    type Output = Unit;

    #[inline]
    fn mul(self, other: Unit) -> Self::Output {
        multiply_terms(&self.terms, &other.terms).into()
    }
}

fn multiply_terms(lhs: &[Term], rhs: &[Term]) -> Vec<Term> {
    let mut terms = Vec::with_capacity(lhs.len() + rhs.len());

    terms.extend_from_slice(lhs);
    terms.extend_from_slice(rhs);

    term_reducing::reduce_terms(&terms)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    lazy_static! {
        static ref ACRE: Unit = Unit::from_str("[acr_us]").unwrap();
        static ref METER: Unit = Unit::from_str("m").unwrap();
        static ref KILOMETER: Unit = Unit::from_str("km").unwrap();
        static ref SEED: Unit = Unit::from_str("{seed}").unwrap();
        static ref UNITY: Unit = Unit::from_str("1").unwrap();
    }

    #[test]
    fn validate_div() {
        let expected = Unit::from_str("m/km").unwrap();
        assert_eq!(&*METER / &*KILOMETER, expected);

        let unit = Unit::from_str("10m").unwrap();
        let other = Unit::from_str("20m").unwrap();
        let expected = Unit::from_str("10m/20m").unwrap();
        assert_eq!(unit / other, expected);

        assert_eq!(&*SEED / &*SEED, *&*UNITY);
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
        assert_eq!(&*SEED * &per_seed, *&*UNITY);

        let seed_per_acre = Unit::from_str("{seed}/[acr_us]").unwrap();
        assert_eq!(seed_per_acre * &*ACRE, *&*SEED);
    }
}
