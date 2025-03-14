use num_traits::Pow;

use crate::{term::Exponent, Unit};

impl Pow<Exponent> for Unit {
    type Output = Self;

    fn pow(self, rhs: Exponent) -> Self::Output {
        Self::new(
            self.terms
                .iter()
                .map(|term| Pow::pow(term, rhs))
                .collect::<Vec<_>>(),
        )
    }
}

impl<'a> Pow<Exponent> for &'a Unit {
    type Output = Unit;

    fn pow(self, rhs: Exponent) -> Self::Output {
        Unit::new(
            self.terms
                .iter()
                .map(|term| term.clone().pow(rhs))
                .collect::<Vec<_>>(),
        )
    }
}

impl<'a> Pow<Exponent> for &'a mut Unit {
    type Output = Self;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.terms.to_mut().iter_mut().for_each(|term| {
            let _ = Pow::pow(term, rhs);
        });
        self
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn owned_test() {
        let subject = Unit::from_str("L").unwrap();
        assert_eq!(Pow::pow(subject, 2), Unit::from_str("L2").unwrap());
    }

    #[test]
    fn borrowed_test() {
        let subject = Unit::from_str("L").unwrap();
        assert_eq!(Pow::pow(&subject, 2), Unit::from_str("L2").unwrap());
    }

    #[test]
    fn mut_borrowed_test() {
        let mut subject = Unit::from_str("L").unwrap();
        let _ = Pow::pow(&mut subject, 2);
        assert_eq!(subject, Unit::from_str("L2").unwrap());
    }
}
