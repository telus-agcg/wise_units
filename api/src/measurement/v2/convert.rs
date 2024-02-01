use crate::{v2::behavior_traits::convert, Measurement, Unit};

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

impl<'a> convert::TryConvertTo<'a, Unit> for Measurement {
    type Error = crate::Error;

    fn try_convert_to(&'a self, rhs: &'a Unit) -> Result<Self, Self::Error> {
        // Just delegate to the old trait impl for now.
        crate::Convertible::convert_to(self, rhs)
    }
}

impl<'a> convert::TryConvertTo<'a, str> for Measurement {
    type Error = crate::Error;

    fn try_convert_to(&'a self, rhs: &'a str) -> Result<Self, Self::Error> {
        // Just delegate to the old trait impl for now.
        crate::Convertible::convert_to(self, rhs)
    }
}

impl convert::ToReduced for Measurement {
    fn to_reduced(&self) -> Self {
        // Just delegate to the old trait impl for now.
        crate::reduce::ToReduced::to_reduced(self).unwrap()
    }
}
