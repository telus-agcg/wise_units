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

/// `std::ops::Add::add()` takes `self`, which makes sense for regular numbers
/// (which are `Copy`), but not for `Measurement` and `Unit` (which are not `Copy`).
/// This is more ergonomic for our types.
///
pub trait RefAdd<Rhs = Self, O = Self> {
    fn ref_add(&self, rhs: &Rhs) -> O;
}

pub trait TryRefAdd<'a, Rhs = Self, O = Self> {
    type Error;

    /// # Errors
    ///
    /// In order for Measurements to be addable, they need to of the same dimension; any errors here
    /// are most likely the result of trying to convert `rhs` to be of the same dimension as `self`.
    ///
    fn try_ref_add(&'a self, rhs: &'a Rhs) -> Result<O, Self::Error>;
}

/// `std::ops::Sub::sub()` takes `self`, which makes sense for regular numbers
/// (which are `Copy`), but not for `Measurement` and `Unit` (which are not `Copy`).
/// This is more ergonomic for our types.
///
pub trait RefSub<Rhs = Self, O = Self> {
    fn ref_sub(&self, rhs: &Rhs) -> O;
}

pub trait TryRefSub<'a, Rhs = Self, O = Self> {
    type Error;

    /// # Errors
    ///
    /// In order for Measurements to be subtractable, they need to of the same dimension;
    /// any errors here are most likely the result of trying to convert `rhs` to be of the
    /// same dimension as `self`.
    ///
    fn try_ref_sub(&'a self, rhs: &'a Rhs) -> Result<O, Self::Error>;
}

/// `std::ops::Div::div()` takes `self`, which makes sense for regular numbers
/// (which are `Copy`), but not for `Measurement` and `Unit` (which are not `Copy`).
/// This is more ergonomic for our types.
///
pub trait RefDiv<Rhs = Self, O = Self> {
    fn ref_div(&self, rhs: &Rhs) -> O;
}

pub trait CheckedRefDiv<Rhs = Self, O = Self>: Sized {
    fn checked_ref_div(&self, rhs: &Rhs) -> Option<O>;
}

/// `std::ops::Mul::mul()` takes `self`, which makes sense for regular numbers
/// (which are `Copy`), but not for `Measurement` and `Unit` (which are not `Copy`).
/// This is more ergonomic for our types.
///
pub trait RefMul<Rhs = Self, O = Self> {
    fn ref_mul(&self, rhs: &Rhs) -> O;
}

pub trait CheckedRefMul<Rhs = Self, O = Self>: Sized {
    fn checked_ref_mul(&self, rhs: &Rhs) -> Option<O>;
}
