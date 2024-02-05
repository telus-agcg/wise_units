//! Defines a basic interface for inverting types, similar to that of a fraction.
//!

pub trait Invert {
    /// Updates `self` to be inverted.
    ///
    fn invert(self);
}

pub trait ToInverse {
    type Output;

    /// Builds a new `Self` that is inverted.
    ///
    fn to_inverse(&self) -> Self::Output;
}

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
