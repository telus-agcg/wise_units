use crate::{v2::behavior_traits::ops::inv, Measurement, Unit};

impl inv::InvertMut for Measurement {
    fn invert_mut(&mut self) {
        self.value = 1.0 / self.value;
        inv::InvertMut::invert_mut(&mut self.unit);
    }
}

impl inv::CheckedInvert for Measurement {
    fn checked_invert(&mut self) -> Option<()> {
        if self.value == 0.0 {
            return None;
        }

        inv::InvertMut::invert_mut(self);

        Some(())
    }
}

impl inv::ToInverse for Measurement {
    fn to_inverse(&self) -> Self {
        Self {
            value: 1.0 / self.value,
            unit: inv::ToInverse::to_inverse(&self.unit),
        }
    }
}

impl inv::CheckedToInverse for Measurement {
    fn checked_to_inverse(&self) -> Option<Self> {
        let new_value = 1.0 / self.value;

        if new_value.is_infinite() {
            return None;
        }

        Some(Self {
            value: new_value,
            unit: inv::ToInverse::to_inverse(&self.unit),
        })
    }
}
