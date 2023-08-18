// NOTE: The difference with this trait is that it doesn't require the output to be a `Result` like
// the original does. This allows for implementing for types that can guarantee a conversion.
pub trait ConvertTo<T> {
    type Output;

    /// _The_ method for doing the conversion.
    ///
    fn convert_to(&self, rhs: T) -> Self::Output;
}

// NOTE: The difference with this trait is that it takes a `&mut self` instead of `self`, allowing
// it to be implemented a bit more conventionally on types: ex. `impl Invert on Term` instead of
// `impl Invert on &mut Term`.
//
pub trait Invert {
    fn invert(&mut self);
}

/// Similar to `Invert`, but allows for implementing for
pub trait TryInvert {
    type Error;

    fn try_invert(&mut self) -> Result<(), Self::Error>;
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
pub trait TryToInverse<T = Self> {
    type Error;

    fn try_to_inverse(&self) -> Result<T, Self::Error>;
}

// NOTE: The difference with this trait is that it's generic over `T`, allowing
// for multiple implementations.
//
pub trait ToReduced<T = Self> {
    type Output;

    fn to_reduced(&self) -> Self::Output;
}

impl ToReduced for crate::Unit {
    type Output = Self;

    fn to_reduced(&self) -> Self {
        crate::reduce::ToReduced::to_reduced(self)
    }
}

pub trait ToScalar<T> {
    fn to_scalar(&self) -> T;
}

impl<T> ToScalar<f64> for T
where
    T: crate::UcumUnit,
{
    fn to_scalar(&self) -> f64 {
        crate::UcumUnit::scalar(self)
    }
}

pub trait TryToScalar<T> {
    type Error;

    fn try_to_scalar(&self) -> Result<T, Self::Error>;
}

pub trait ToMagnitude<T> {
    fn to_magnitude(&self) -> T;
}

impl<T> ToMagnitude<f64> for T
where
    T: crate::UcumUnit,
{
    fn to_magnitude(&self) -> f64 {
        crate::UcumUnit::magnitude(self)
    }
}

pub trait TryToMagnitude<T> {
    type Error;

    fn try_to_magnitude(&self) -> Result<T, Self::Error>;
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
