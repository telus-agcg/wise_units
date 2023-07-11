//! This module contains (re)definitions of standard `wise_units` traits, but potentially as they
//! should've been defined. Defining them here gives an opportunity to use them in other crates that
//! build on `wise_units`, and if that goes well, then these traits can then replace the old ones.

pub mod ucum_symbol;
pub mod ucum_unit;

// NOTE: The difference with this trait is that a) you can specify the output type for the
// `as_fraction()` call, letting wrapper crates use this trait (since other types may not easily be
// able to convert from `(Self::Numerator, Self::Denominator)`).
//
pub trait AsFraction {
    type Denominator;
    type Numerator;
    type Fraction;

    fn as_fraction(&self) -> Self::Fraction;
    fn numerator(&self) -> Self::Numerator;
    fn denominator(&self) -> Self::Denominator;
}

pub trait Composable {
    type Composition;

    fn composition(&self) -> Self::Composition;
}

impl<T> Composable for T
where
    T: crate::Composable,
{
    type Composition = crate::Composition;

    fn composition(&self) -> Self::Composition {
        crate::Composable::composition(self)
    }
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

// NOTE: The difference with this trait is that it takes a `&mut self` instead of `self`, allowing
// it to be implemented a bit more conventionally on types: ex. `impl Invert on Term` instead of
// `impl Invert on &mut Term`.
//
pub trait Invert {
    type Error;

    fn invert(&mut self) -> Result<(), Self::Error>;
}

// NOTE: The difference with this trait is that it's generic over `T`, allowing
// for multiple implementations.
//
pub trait ToInverse<T = Self> {
    type Error;

    fn to_inverse(&self) -> Result<T, Self::Error>;
}

// NOTE: The difference with this trait is that it's generic over `T`, allowing
// for multiple implementations.
//
pub trait ToReduced<T = Self> {
    type Error;

    fn to_reduced(&self) -> Result<T, Self::Error>;
}
