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
