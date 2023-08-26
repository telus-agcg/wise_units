//// NOTE: The differences with this trait are
//// 1. It doesn't require `rhs` to be a reference
//// 2. It doesn't default `T` to `Self`
////
//pub trait IsCompatibleWith<T: ?Sized> {
//    fn is_compatible_with(&self, rhs: &T) -> bool;
//}

/// This trait is just a carryover from the old traits, where the old trait's use of lifetimes made
/// it awkward to implement; this fixes that awkwardness, but this trait should go away. It was
/// introduced to distinguish `PartialEq` and `FieldEq`, where the implementation of `PartialEq` was
/// done to allow for, say `1 kilometer == 1000 meters`, but this implementation prevents for a
/// proper `Hash` implementation, where we want different hashes for `1 kilometer` vs `1000 meters`,
/// but the [`Hash` and `PartialEq` implementations need to reflect each
/// other](https://rust-lang.github.io/rust-clippy/stable/index.html#derived_hash_with_manual_eq).
///
/// All that to say, the next major release of `wise_units` will change the behavior of `PartialEq`
/// to mimic that of what `FieldEq` implementations do, and rely on `IsCommensurableWith` to see
/// that `1 kilometer == 1000 meters`.
///
/// See the [Semantics](https://ucum.org/ucum#section-Semantics) section of the UCUM spec.
///
pub trait FieldEq<T> {
    fn field_eq(&self, rhs: &T) -> bool;
}

/// Allows for checking that `1 kilometer == 1000 meters`.
///
/// See the [Semantics](https://ucum.org/ucum#section-Semantics) section of the UCUM spec.
///
pub trait IsCommensurableWith<Rhs = Self> {
    fn is_commensurable_with(&self, rhs: &Rhs) -> bool;
}

/// `std::ops::Add::add()` takes `self`, which makes sense for regular numbers
/// (which are `Copy`), but not for `Measurement` and `Unit` (which are not `Copy`).
/// This is more ergonomic for our types.
///
pub trait RefAdd<Rhs = Self, O = Self> {
    fn ref_add(&self, rhs: &Rhs) -> O;
}

pub trait TryAddRef<'a, Rhs = Self, O = Self> {
    type Error;

    /// # Errors
    ///
    /// In order for Measurements to be addable, they need to of the same dimension; any errors here
    /// are most likely the result of trying to convert `rhs` to be of the same dimension as `self`.
    ///
    fn try_add_ref(&'a self, rhs: &'a Rhs) -> Result<O, Self::Error>;
}

/// `std::ops::Sub::sub()` takes `self`, which makes sense for regular numbers
/// (which are `Copy`), but not for `Measurement` and `Unit` (which are not `Copy`).
/// This is more ergonomic for our types.
///
pub trait SubRef<Rhs = Self, O = Self> {
    fn ref_sub(&self, rhs: &Rhs) -> O;
}

pub trait TrySubRef<'a, Rhs = Self, O = Self> {
    type Error;

    /// # Errors
    ///
    /// In order for Measurements to be subtractable, they need to of the same dimension;
    /// any errors here are most likely the result of trying to convert `rhs` to be of the
    /// same dimension as `self`.
    ///
    fn try_sub_ref(&'a self, rhs: &'a Rhs) -> Result<O, Self::Error>;
}

/// `std::ops::Div::div()` takes `self`, which makes sense for regular numbers
/// (which are `Copy`), but not for `Measurement` and `Unit` (which are not `Copy`).
/// This is more ergonomic for our types.
///
pub trait DivRef<Rhs = Self, O = Self> {
    fn div_ref(&self, rhs: &Rhs) -> O;
}

pub trait CheckedDivRef<Rhs = Self, O = Self>: Sized {
    fn checked_div_ref(&self, rhs: &Rhs) -> Option<O>;
}

/// `std::ops::Mul::mul()` takes `self`, which makes sense for regular numbers
/// (which are `Copy`), but not for `Measurement` and `Unit` (which are not `Copy`).
/// This is more ergonomic for our types.
///
pub trait MulRef<Rhs = Self, O = Self> {
    fn mul_ref(&self, rhs: &Rhs) -> O;
}

pub trait CheckedMulRef<Rhs = Self, O = Self>: Sized {
    fn checked_mul_ref(&self, rhs: &Rhs) -> Option<O>;
}
