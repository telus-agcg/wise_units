use std::ops::Neg;

use crate::measurement::Measurement;

// ╭──────────╮
// │ impl Neg │
// ╰──────────╯
impl Neg for Measurement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            value: self.value.neg(),
            unit: self.unit,
        }
    }
}

impl<'a> Neg for &'a Measurement {
    type Output = Measurement;

    fn neg(self) -> Self::Output {
        Measurement {
            value: self.value.neg(),
            unit: self.unit.clone(),
        }
    }
}

impl<'a> Neg for &'a mut Measurement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.value = self.value.neg();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_test() {
        let subject = measurement!(10.0, "L");
        assert_eq!(Neg::neg(subject), measurement!(-10.0, "L"));

        let subject = measurement!(-10.0, "L");
        assert_eq!(Neg::neg(subject), measurement!(10.0, "L"));
    }

    #[test]
    fn borrowed_test() {
        let subject = measurement!(10.0, "L");
        assert_eq!(Neg::neg(&subject), measurement!(-10.0, "L"));

        let subject = measurement!(-10.0, "L");
        assert_eq!(Neg::neg(&subject), measurement!(10.0, "L"));
    }

    #[test]
    fn mut_borrowed_test() {
        let mut subject = measurement!(10.0, "L");
        let _ = Neg::neg(&mut subject);
        assert_eq!(subject, measurement!(-10.0, "L"));

        let mut subject = measurement!(-10.0, "L");
        let _ = Neg::neg(&mut subject);
        assert_eq!(subject, measurement!(10.0, "L"));
    }
}
