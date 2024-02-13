use num_traits::Pow;

use crate::Term;

impl Pow<i32> for Term {
    type Output = Self;

    fn pow(self, rhs: i32) -> Self::Output {
        let exponent = self.exponent.map_or(rhs, |exp| exp * rhs);

        Self {
            factor: self.factor,
            prefix: self.prefix,
            atom: self.atom,
            exponent: Some(exponent),
            annotation: self.annotation,
        }
    }
}

impl<'a> Pow<i32> for &'a Term {
    type Output = Term;

    fn pow(self, rhs: i32) -> Self::Output {
        let exponent = self.exponent.map_or(rhs, |exp| exp * rhs);

        Term {
            factor: self.factor,
            prefix: self.prefix,
            atom: self.atom,
            exponent: Some(exponent),
            annotation: self.annotation.clone(),
        }
    }
}

impl<'a> Pow<i32> for &'a mut Term {
    type Output = Self;

    fn pow(self, rhs: i32) -> Self::Output {
        self.exponent = Some(self.exponent.map_or(rhs, |exp| exp * rhs));

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_no_exponent_test() {
        let subject = term!(Liter);
        assert_eq!(Pow::pow(subject, 2), term!(Liter, exponent: 2));
    }

    #[test]
    fn owned_negative_exponent_test() {
        let subject = term!(Liter, exponent: -1);
        assert_eq!(Pow::pow(subject, 2), term!(Liter, exponent: -2));
    }

    #[test]
    fn owned_positive_exponent_test() {
        let subject = term!(Liter, exponent: 1);
        assert_eq!(Pow::pow(subject, 2), term!(Liter, exponent: 2));
    }

    #[test]
    fn borrowed_no_exponent_test() {
        let subject = term!(Liter);
        assert_eq!(Pow::pow(&subject, 2), term!(Liter, exponent: 2));
    }

    #[test]
    fn borrowed_negative_exponent_test() {
        let subject = term!(Liter, exponent: -1);
        assert_eq!(Pow::pow(&subject, 2), term!(Liter, exponent: -2));
    }

    #[test]
    fn borrowed_positive_exponent_test() {
        let subject = term!(Liter, exponent: 1);
        assert_eq!(Pow::pow(&subject, 2), term!(Liter, exponent: 2));
    }

    #[test]
    fn mut_borrowed_no_exponent_test() {
        let mut subject = term!(Liter);
        let _ = Pow::pow(&mut subject, 2);
        assert_eq!(subject, term!(Liter, exponent: 2));
    }

    #[test]
    fn mut_borrowed_negative_exponent_test() {
        let mut subject = term!(Liter, exponent: -1);
        let _ = Pow::pow(&mut subject, 2);
        assert_eq!(subject, term!(Liter, exponent: -2));
    }

    #[test]
    fn mut_borrowed_positive_exponent_test() {
        let mut subject = term!(Liter, exponent: 1);
        let _ = Pow::pow(&mut subject, 2);
        assert_eq!(subject, term!(Liter, exponent: 2));
    }
}
