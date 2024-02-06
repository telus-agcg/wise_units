use num_traits::One;

use crate::Measurement;

impl One for Measurement {
    fn one() -> Self {
        Self {
            value: One::one(),
            unit: crate::unit::UNITY,
        }
    }

    fn is_one(&self) -> bool {
        self.value.is_one()
    }
}

#[cfg(test)]
mod tests {
    use crate::unit;

    use super::*;

    #[test]
    fn one_test() {
        assert_eq!(Measurement::new(1.0, unit::UNITY), Measurement::one());
    }
}
