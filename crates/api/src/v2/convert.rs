/// This is the next version of `AsFraction`, which was incorrectly named, according to Rust
/// API guidelines. The difference with this trait is that a) you can specify the output type for
/// the `to_fraction()` call, letting wrapper crates use this trait (since other types may not
/// easily be able to convert from `(Self::Numerator, Self::Denominator)`).
///
pub trait ToFraction<N = Option<Self>, D = Option<Self>, F = (N, D)> {
    fn to_fraction(&self) -> F;

    fn to_numerator(&self) -> N;
    fn to_denominator(&self) -> D;
}

/// Trait for infallible conversion.
///
/// The differences with this trait compared to `Convertible<RHS>` are:
/// 1. it doesn't require the output to be a `Result` like the original does. This allows for
/// implementing for types that can guarantee a conversion.
/// 2. it can be implemented for multiple output types.
///
#[allow(clippy::module_name_repetitions)]
pub trait ConvertTo<U, O = Self> {
    /// _The_ method for doing the conversion.
    ///
    fn convert_to(&self, rhs: U) -> O;
}

/// Trait for fallible conversion.
///
/// The difference with this trait compared to `Convertible<RHS>` is that it can be implemented for
/// multiple output types.
///
pub trait TryConvertTo<U, O = Self> {
    type Error;

    /// _The_ method for doing the conversion.
    ///
    /// # Errors
    ///
    /// This should fail if `self` couldn't be converted to `O`.
    ///
    fn try_convert_to(&self, rhs: U) -> Result<O, Self::Error>;
}
