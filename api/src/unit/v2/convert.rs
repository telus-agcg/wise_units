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

impl convert::Invert for Unit {
    fn invert(&mut self) {
        convert::Invert::invert(&mut self.terms);
    }
}

impl convert::ToInverse for Unit {
    fn to_inverse(&self) -> Self {
        Self::new(convert::ToInverse::to_inverse(&self.terms))
    }
}

impl convert::ToScalar<'_, f64> for Unit {
    fn to_scalar(&self) -> f64 {
        // Just delegate to the old trait impl for now.
        crate::UcumUnit::scalar(self)
    }
}

impl convert::ToMagnitude<'_, f64> for Unit {
    fn to_magnitude(&self) -> f64 {
        // Just delegate to the old trait impl for now.
        crate::UcumUnit::magnitude(self)
    }
}
