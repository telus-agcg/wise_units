use num_traits::Pow;

use crate::Measurement;

impl Pow<i32> for Measurement {
    type Output = Self;

    fn pow(self, rhs: i32) -> Self::Output {
        Self {
            value: self.value.pow(rhs),
            unit: self.unit.pow(rhs),
        }
    }
}

impl<'a> Pow<i32> for &'a Measurement {
    type Output = Measurement;

    fn pow(self, rhs: i32) -> Self::Output {
        Measurement {
            value: self.value.pow(rhs),
            unit: self.unit.clone().pow(rhs),
        }
    }
}

impl<'a> Pow<i32> for &'a mut Measurement {
    type Output = Self;

    fn pow(self, rhs: i32) -> Self::Output {
        self.value = self.value.pow(rhs);
        let _ = Pow::pow(&mut self.unit, rhs);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_test() {
        let subject = measurement!(10.0, "L");
        assert_eq!(Pow::pow(subject, 2), measurement!(100.0, "L2"));
    }

    #[test]
    fn borrowed_test() {
        let subject = measurement!(10.0, "L");
        assert_eq!(Pow::pow(&subject, 2), measurement!(100.0, "L2"));
    }

    #[test]
    fn mut_borrowed_test() {
        let mut subject = measurement!(10.0, "L");
        let _ = Pow::pow(&mut subject, 2);
        assert_eq!(subject, measurement!(100.0, "L2"));
    }
}
