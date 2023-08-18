// NOTE: The differences with this trait are
// 1. It doesn't require `rhs` to be a reference
// 2. It doesn't default `T` to `Self`
//
pub trait IsCompatibleWith<T> {
    fn is_compatible_with(&self, rhs: T) -> bool;
}

pub trait FieldEq<T> {
    fn field_eq(&self, rhs: T) -> bool;
}

/// `std::ops::Div::div()` takes `self`, which makes sense for regular numbers
/// (which are `Copy`), but not for `Measurement` and `Unit` (which are not `Copy`).
/// This is more ergonomic for our types.
//
pub trait RefDiv<Rhs = Self> {
    type Output;

    fn ref_div(&self, rhs: &Rhs) -> Self::Output;
}

/// `std::ops::Mul::mul()` takes `self`, which makes sense for regular numbers
/// (which are `Copy`), but not for `Measurement` and `Unit` (which are not `Copy`).
/// This is more ergonomic for our types.
//
pub trait RefMul<Rhs = Self> {
    type Output;

    fn ref_mul(&self, rhs: &Rhs) -> Self::Output;
}
