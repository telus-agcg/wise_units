use crate::{v2::behavior_traits::convert, Unit};

impl convert::ToFraction for Unit {
    fn to_fraction(&self) -> (Option<Self>, Option<Self>) {
        (
            convert::ToFraction::numerator(self),
            convert::ToFraction::denominator(self),
        )
    }

    fn numerator(&self) -> Option<Self> {
        // Just delegate to the old trait impl for now.
        crate::as_fraction::AsFraction::numerator(self)
    }

    fn denominator(&self) -> Option<Self> {
        // Just delegate to the old trait impl for now.
        crate::as_fraction::AsFraction::denominator(self)
    }
}

impl convert::ToScalar<f64> for Unit {
    fn to_scalar(&self) -> f64 {
        // Just delegate to the old trait impl for now.
        crate::UcumUnit::scalar(self)
    }
}

impl convert::ToMagnitude<f64> for Unit {
    fn to_magnitude(&self) -> f64 {
        // Just delegate to the old trait impl for now.
        crate::UcumUnit::magnitude(self)
    }
}

impl convert::ToReduced for Unit {
    fn to_reduced(&self) -> Self {
        // Just delegate to the old trait impl for now.
        crate::reduce::ToReduced::to_reduced(self)
    }
}
