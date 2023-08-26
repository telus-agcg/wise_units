use crate::{v2::behavior_traits::unit_conversion, Measurement, Unit};

impl unit_conversion::TryConvertTo<Unit> for Measurement {
    type Error = crate::Error;

    fn try_convert_to(&self, rhs: &Unit) -> Result<Self, Self::Error> {
        // Just delegate to the old trait impl for now.
        crate::Convertible::convert_to(self, rhs)
    }
}

impl unit_conversion::TryConvertTo<str> for Measurement {
    type Error = crate::Error;

    fn try_convert_to(&self, rhs: &str) -> Result<Self, Self::Error> {
        // Just delegate to the old trait impl for now.
        crate::Convertible::convert_to(self, rhs)
    }
}

impl unit_conversion::ToReduced for Measurement {
    fn to_reduced(&self) -> Self {
        // Just delegate to the old trait impl for now.
        crate::reduce::ToReduced::to_reduced(self).unwrap()
    }
}
