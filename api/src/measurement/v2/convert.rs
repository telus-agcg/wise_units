use crate::{v2::traits::convert, Measurement};

impl convert::Invert for Measurement {
    fn invert(&mut self) {
        self.value = 1.0 / self.value;
        convert::Invert::invert(&mut self.unit);
    }
}

impl convert::CheckedInvert for Measurement {
    fn checked_invert(&mut self) -> Option<()> {
        if self.value == 0.0 {
            return None;
        }

        convert::Invert::invert(self);

        Some(())
    }
}

impl convert::ToInverse for Measurement {
    fn to_inverse(&self) -> Self {
        Self {
            value: 1.0 / self.value,
            unit: convert::ToInverse::to_inverse(&self.unit),
        }
    }
}

impl convert::CheckedToInverse for Measurement {
    fn checked_to_inverse(&self) -> Option<Self> {
        let new_value = 1.0 / self.value;

        if new_value.is_infinite() {
            return None;
        }

        Some(Self {
            value: new_value,
            unit: convert::ToInverse::to_inverse(&self.unit),
        })
    }
}

impl convert::ToScalar<f64> for Measurement {
    fn to_scalar(&self) -> f64 {
        // Just delegate to the old trait impl for now.
        crate::UcumUnit::scalar(self)
    }
}

impl convert::ToMagnitude<f64> for Measurement {
    fn to_magnitude(&self) -> f64 {
        // Just delegate to the old trait impl for now.
        crate::UcumUnit::magnitude(self)
    }
}
