//!
//! These types only pertain to `Measurement`s since converting requires both a value and a
//! unit, not just a unit alone.
//!
use std::borrow::Cow;

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
#[error("Unable to convert: {source:?}, {dest:?}")]
pub struct ConversionError<'a, T, U>
where
    T: Clone,
    U: Clone,
{
    source: Cow<'a, T>,
    dest: Cow<'a, U>,
}

impl<'a, T, U> ConversionError<'a, T, U>
where
    T: Clone,
    U: Clone,
{
    pub const fn new_borrowed(source: &'a T, dest: &'a U) -> Self {
        Self {
            source: Cow::Borrowed(source),
            dest: Cow::Borrowed(dest),
        }
    }
}

// NOTE: The difference with this trait is that it doesn't require the output to be a `Result` like
// the original does. This allows for implementing for types that can guarantee a conversion.
pub trait ConvertTo<U: ?Sized, O = Self> {
    /// _The_ method for doing the conversion.
    ///
    fn convert_to(&self, rhs: &U) -> O;
}

pub trait TryConvertTo<U: ?Sized, O = Self>: Sized {
    type Error;

    /// _The_ method for doing the conversion.
    ///
    /// # Errors
    ///
    /// This should fail if `self` couldn't be converted to `O`.
    ///
    fn try_convert_to(&self, rhs: &U) -> Result<O, Self::Error>;
}

// NOTE: The difference with this trait is that it's generic over `T`, allowing
// for multiple implementations.
//
pub trait ToReduced<T = Self> {
    fn to_reduced(&self) -> T;
}
