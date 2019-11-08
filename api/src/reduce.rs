#![allow(clippy::doc_markdown)]
/// Defines an interface for reducing `Unit`s and `Measurements`, where "reducing" is on cancelling
/// out `Unit`s that have the same dimension (ex. "[acr_us]" and "har" both have the dimension
/// "L2", so they can effectively be canceled out).
///
pub trait ToReduced {
    type Output;

    fn to_reduced(&self) -> Self::Output;
}

/// Defines an interface similar to `ToReduced`, but which consumes `self`.
///
pub trait IntoReduced: ToReduced {
    fn into_reduced(self) -> <Self as ToReduced>::Output;
}

/// Auto-derive `IntoReduced` for all types that implement `ToReduced`.
///
impl<T: ToReduced> IntoReduced for T {
    fn into_reduced(self) -> <Self as ToReduced>::Output {
        ToReduced::to_reduced(&self)
    }
}
