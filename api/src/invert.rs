/// Defines a basic interface for inverting types, similar to that of a fraction.
///
pub trait Invert {
    /// Updates `self` to be inverted.
    ///
    fn invert(self);
}

pub trait IntoInverse {
    /// Builds a new `Self` that is inverted.
    ///
    fn into_inverse(&self) -> Self;
}
