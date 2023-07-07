//! This module contains (re)definitions of standard `wise_units` traits, but potentially as they
//! should've been defined. Defining them here gives an opportunity to use them in other crates that
//! build on `wise_units`, and if that goes well, then these traits can then replace the old ones.

pub mod ucum_unit;

pub trait Composable {
    type Composition;

    fn composition(&self) -> Self::Composition;
}

// NOTE: The difference with this trait is that it doesn't require the output to be a `Result` like
// the original does. This allows for implementing for types that can guarantee a conversion.
pub trait Convertible<RHS> {
    type Output;

    /// _The_ method for doing the conversion.
    ///
    fn convert_to(&self, unit: RHS) -> Self::Output;
}

pub trait FieldEq<T> {
    fn field_eq(&self, rhs: T) -> bool;
}
