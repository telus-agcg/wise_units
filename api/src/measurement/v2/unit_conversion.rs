use crate::{v2::behavior_traits::unit_conversion, Measurement, Unit};

impl<'a> unit_conversion::TryConvertTo<'a, Unit> for Measurement {
    type Error = crate::Error;

    fn try_convert_to(&'a self, rhs: &'a Unit) -> Result<Self, Self::Error> {
        // Just delegate to the old trait impl for now.
        crate::Convertible::convert_to(self, rhs)
    }
}

impl<'a> unit_conversion::TryConvertTo<'a, str> for Measurement {
    type Error = crate::Error;

    fn try_convert_to(&'a self, rhs: &'a str) -> Result<Self, Self::Error> {
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
