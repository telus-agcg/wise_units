//!
//! These types only pertain to `Measurement`s since converting requires both a value and a
//! unit, not just a unit alone.
//!
use std::borrow::Cow;

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
#[error("Unable to convert: {source:?} using {factor:?}")]
pub struct ConversionError<'a, S, F>
where
    S: Clone,
    F: Clone,
{
    source: Cow<'a, S>,
    factor: Cow<'a, F>,
}

impl<'a, S, F> ConversionError<'a, S, F>
where
    S: Clone,
    F: Clone,
{
    pub const fn new_borrowed(source: &'a S, factor: &'a F) -> Self {
        Self {
            source: Cow::Borrowed(source),
            factor: Cow::Borrowed(factor),
        }
    }
}

/// Trait for infallible conversion.
///
// NOTE: The difference with this trait is that it doesn't require the output to be a `Result` like
// the original does. This allows for implementing for types that can guarantee a conversion.
pub trait ConvertTo<U: ?Sized, O = Self> {
    /// _The_ method for doing the conversion.
    ///
    fn convert_to(&self, rhs: &U) -> O;
}

pub trait TryConvertTo<'a, U: ?Sized, O = Self>: Sized {
    type Error;

    /// _The_ method for doing the conversion.
    ///
    /// # Errors
    ///
    /// This should fail if `self` couldn't be converted to `O`.
    ///
    fn try_convert_to(&'a self, rhs: &'a U) -> Result<O, Self::Error>;
}

// NOTE: The difference with this trait is that it's generic over `T`, allowing
// for multiple implementations.
//
pub trait ToReduced<T = Self> {
    fn to_reduced(&self) -> T;
}
