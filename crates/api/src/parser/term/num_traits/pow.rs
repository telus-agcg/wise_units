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
