use super::term_reducing;
use crate::parser::Term;
use std::ops::{Div, Mul};
use crate::unit::Unit;

//-----------------------------------------------------------------------------
// impl Div
//-----------------------------------------------------------------------------
fn divide_terms(lhs: &[Term], rhs: &[Term]) -> Vec<Term> {
    let mut terms = Vec::with_capacity(lhs.len() + rhs.len());

    for term in lhs.iter() {
        terms.push(term.clone());
    }

    for term in rhs.iter() {
        let mut new_other_term = term.clone();
        new_other_term.invert_exponent();

        terms.push(new_other_term);
    }

    term_reducing::reduce_terms(&terms)
}

impl Div for Unit {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self::Output {
        let terms = divide_terms(&self.terms, &other.terms);

        Self::from(terms)
    }
}

impl<'a> Div<&'a Unit> for Unit {
    type Output = Self;

    #[inline]
    fn div(self, other: &'a Self) -> Self::Output {
        let terms = divide_terms(&self.terms, &other.terms);

        Self::from(terms)
    }
}

impl<'a> Div for &'a Unit {
    type Output = Unit;

    #[inline]
    fn div(self, other: &'a Unit) -> Self::Output {
        let terms = divide_terms(&self.terms, &other.terms);

        Unit::from(terms)
    }
}

impl<'a> Div<Unit> for &'a Unit {
    type Output = Unit;

    #[inline]
    fn div(self, other: Unit) -> Self::Output {
        let terms = divide_terms(self, &other);

        Unit::from(terms)
    }
}

//-----------------------------------------------------------------------------
// impl Mul
//-----------------------------------------------------------------------------
impl Mul for Unit {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        let terms = multiply_terms(&self.terms, &other.terms);

        Self::from(terms)
    }
}

impl<'a> Mul<&'a Unit> for Unit {
    type Output = Self;

    #[inline]
    fn mul(self, other: &'a Self) -> Self::Output {
        let terms = multiply_terms(&self.terms, &other.terms);

        Self::from(terms)
    }
}

impl<'a> Mul for &'a Unit {
    type Output = Unit;

    #[inline]
    fn mul(self, other: &'a Unit) -> Self::Output {
        let terms = multiply_terms(&self.terms, &other.terms);

        Unit::from(terms)
    }
}

impl<'a> Mul<Unit> for &'a Unit {
    type Output = Unit;

    #[inline]
    fn mul(self, other: Unit) -> Self::Output {
        let terms = multiply_terms(&self.terms, &other.terms);

        Unit::from(terms)
    }
}

fn multiply_terms(lhs: &[Term], rhs: &[Term]) -> Vec<Term> {
    let mut terms = Vec::with_capacity(lhs.len() + rhs.len());

    for term in lhs.iter() {
        terms.push(term.clone());
    }

    for term in rhs.iter() {
        terms.push(term.clone());
    }

    term_reducing::reduce_terms(&terms)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::{Div, Mul};
    use std::str::FromStr;

    #[test]
    fn validate_div() {
        let unit = Unit::from_str("m").unwrap();
        let other = Unit::from_str("km").unwrap();
        assert_eq!(unit.div(other).to_string().as_str(), "m/km");

        let unit = Unit::from_str("10m").unwrap();
        let other = Unit::from_str("20m").unwrap();
        assert_eq!(unit.div(other).to_string().as_str(), "10m/20m");
    }

    #[test]
    fn validate_mul() {
        let unit = Unit::from_str("m").unwrap();
        let other = Unit::from_str("km").unwrap();
        assert_eq!(unit.mul(other).to_string().as_str(), "m.km");

        let unit = Unit::from_str("10m").unwrap();
        let other = Unit::from_str("20m").unwrap();
        assert_eq!(unit.mul(other).to_string().as_str(), "10m.20m");
    }
}
