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
