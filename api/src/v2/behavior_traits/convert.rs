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

pub trait ToMagnitude<T> {
    fn to_magnitude(&self) -> T;
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
