use std::convert::Infallible;

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ConversionError<T> {
    #[error(transparent)]
    Unit(#[from] UnitConversionError),

    #[error(transparent)]
    Scalar(#[from] ScalarConversionError<T>),
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
#[error("Unable to convert: {source_unit:?} to {destination_unit:?}")]
pub struct UnitConversionError {
    source_unit: String,
    destination_unit: String,
}

impl UnitConversionError {
    pub fn new<'a, S, D>(source_unit: &'a S, destination_unit: &'a D) -> Self
    where
        &'a S: ToString,
        &'a D: ToString,
    {
        Self {
            source_unit: source_unit.to_string(),
            destination_unit: destination_unit.to_string(),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
#[error("Unable to convert: {source_unit:?} to scalar value")]
pub struct ScalarConversionError<T> {
    source_unit: String,
    #[source]
    source: T,
}

impl<T> ScalarConversionError<T> {
    pub fn new<'a, S>(source_unit: &'a S, source: T) -> Self
    where
        &'a S: ToString,
    {
        Self {
            source_unit: source_unit.to_string(),
            source,
        }
    }
}

// NOTE: The difference with this trait is that it takes a `&mut self` instead of `self`, allowing
// it to be implemented a bit more conventionally on types: ex. `impl Invert on Term` instead of
// `impl Invert on &mut Term`.
//
pub trait Invert {
    fn invert(&mut self);
}

/// Similar to `Invert`, but allows for checking for divide-by-0 errors before converting; should
/// return `None` in that case.
///
pub trait CheckedInvert {
    fn checked_invert(&mut self) -> Option<()>;
}

// NOTE: The difference with this trait is that it's generic over `T`, allowing
// for multiple implementations.
//
pub trait ToInverse<T = Self> {
    fn to_inverse(&self) -> T;
}

// NOTE: The difference with this trait is that it's generic over `T`, allowing
// for multiple implementations.
//
pub trait CheckedToInverse<T = Self> {
    fn checked_to_inverse(&self) -> Option<T>;
}

pub trait ToScalar<V> {
    fn to_scalar(&self) -> V;
}

pub trait TryToScalar<V> {
    type Error;

    /// # Errors
    ///
    /// An error returned here should indicate that `self` could not be coerced to a `V`. One
    /// example of this would be converting a `f64` to a `num_rational::Rational`, where
    /// `f64::INFINITY`, `f64::NEG_INFINITY`, and `f64::NAN` can't be converted.
    ///
    fn try_to_scalar(&self) -> Result<V, Self::Error>;
}

impl<T, V> TryToScalar<V> for T
where
    T: ToScalar<V>,
{
    type Error = Infallible;

    fn try_to_scalar(&self) -> Result<V, Self::Error> {
        Ok(self.to_scalar())
    }
}

pub trait ToMagnitude<V> {
    fn to_magnitude(&self) -> V;
}

pub trait TryToMagnitude<V> {
    type Error;

    /// # Errors
    ///
    /// An error returned here should indicate that `self` could not be coerced to a `V`. One
    /// example of this would be converting a `f64` to a `num_rational::Rational`, where
    /// `f64::INFINITY`, `f64::NEG_INFINITY`, and `f64::NAN` can't be converted.
    ///
    fn try_to_magnitude(&self) -> Result<V, Self::Error>;
}

impl<T, V> TryToMagnitude<V> for T
where
    T: ToMagnitude<V>,
{
    type Error = Infallible;

    fn try_to_magnitude(&self) -> Result<V, Self::Error> {
        Ok(self.to_magnitude())
    }
}

// NOTE: This is the next version of `AsFraction`, which was incorrectly named, according to Rust
// API guidelines. The difference with this trait is that a) you can specify the output type for
// the `to_fraction()` call, letting wrapper crates use this trait (since other types may not
// easily be able to convert from `(Self::Numerator, Self::Denominator)`).
//
pub trait ToFraction<N = Option<Self>, D = Option<Self>, F = (N, D)> {
    fn to_fraction(&self) -> F;

    fn numerator(&self) -> N;
    fn denominator(&self) -> D;
}

/// Trait for infallible conversion.
///
// NOTE: The difference with this trait is that it doesn't require the output to be a `Result` like
// the original does. This allows for implementing for types that can guarantee a conversion.
#[allow(clippy::module_name_repetitions)]
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
