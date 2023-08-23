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

// NOTE: The difference with this trait is that it's generic over `T`, allowing
// for multiple implementations.
//
pub trait ToReduced<T = Self> {
    fn to_reduced(&self) -> T;
}

// impl ToReduced for crate::Unit {
//     fn to_reduced(&self) -> Self {
//         crate::reduce::ToReduced::to_reduced(self)
//     }
// }

///// In reality, this probably shouldn't be needed. Reducing can involve converting, but shouldn't
///// ever mean converting to units of a different dimension. Until we can capture dimensions as
///// types, though, converting has to technically use the `TryConvertTo` trait, which allows
///// conversion to fail.
/////
//pub trait TryToReduced<T = Self> {
//    type Error;

//    /// # Errors
//    ///
//    /// In reality, this should never fail, but because reducing can rely on conversion calls, which
//    /// are fallible,
//    fn try_to_reduced(&self) -> Result<T, Self::Error>;
//}

pub trait ToScalar<T> {
    fn to_scalar(&self) -> T;
}

// impl<T> ToScalar<f64> for T
// where
//     T: crate::UcumUnit,
// {
//     fn to_scalar(&self) -> f64 {
//         crate::UcumUnit::scalar(self)
//     }
// }

pub trait TryToScalar<T> {
    type Error;

    fn try_to_scalar(&self) -> Result<T, Self::Error>;
}

pub trait ToMagnitude<T> {
    fn to_magnitude(&self) -> T;
}

// impl<T> ToMagnitude<f64> for T
// where
//     T: crate::UcumUnit,
// {
//     fn to_magnitude(&self) -> f64 {
//         crate::UcumUnit::magnitude(self)
//     }
// }

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
