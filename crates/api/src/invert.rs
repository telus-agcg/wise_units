// TODO: Remove in 1.0.0 release.
#![allow(deprecated)]

//! Defines a basic interface for inverting types, similar to that of a fraction.
//!

#[deprecated(since = "1.0.0", note = "Please use num_traits::Inv")]
pub trait Invert {
    /// Updates `self` to be inverted.
    ///
    fn invert(self);
}

#[deprecated(since = "1.0.0", note = "Please use num_traits::Inv")]
pub trait ToInverse {
    type Output;

    /// Builds a new `Self` that is inverted.
    ///
    fn to_inverse(&self) -> Self::Output;
}

#[deprecated(since = "1.0.0", note = "Please use num_traits::Inv")]
pub trait IntoInverse: ToInverse {
    /// Builds a new `Self` that is inverted.
    ///
    fn into_inverse(self) -> <Self as ToInverse>::Output;
}

/// Auto-derive `IntoInverse` for all types that implement `ToInverse`.
///
impl<T: ToInverse> IntoInverse for T {
    fn into_inverse(self) -> <Self as ToInverse>::Output {
        ToInverse::to_inverse(&self)
    }
}
