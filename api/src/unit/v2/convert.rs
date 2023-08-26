// use crate::{v2::behavior_traits::convert, v2::type_traits::term_unit::TermUnit, Unit};

// impl convert::ToFraction for Unit {
//     fn to_fraction(&self) -> (Option<Self>, Option<Self>) {
//         (
//             crate::v2::traits::convert::ToFraction::numerator(self),
//             crate::v2::traits::convert::ToFraction::denominator(self),
//         )
//     }

//     fn numerator(&self) -> Option<Self> {
//         // Just delegate to the old trait impl for now.
//         crate::as_fraction::AsFraction::numerator(self)
//     }

//     fn denominator(&self) -> Option<Self> {
//         // Just delegate to the old trait impl for now.
//         crate::as_fraction::AsFraction::denominator(self)
//     }
// }

// impl convert::Invert for Unit {
//     fn invert(&mut self) {
//         convert::Invert::invert(&mut self.terms);
//     }
// }

// impl convert::ToInverse for Unit {
//     fn to_inverse(&self) -> Self {
//         Self::new(crate::v2::traits::convert::ToInverse::to_inverse(
//             &self.terms,
//         ))
//     }
// }

// impl convert::ToScalar<f64> for Unit {
//     fn to_scalar(&self) -> f64 {
//         // Just delegate to the old trait impl for now.
//         crate::UcumUnit::scalar(self)
//     }
// }

// impl convert::ToMagnitude<f64> for Unit {
//     fn to_magnitude(&self) -> f64 {
//         // Just delegate to the old trait impl for now.
//         crate::UcumUnit::magnitude(self)
//     }
// }
