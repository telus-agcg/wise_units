use std::{
    cmp::Ordering,
    ops::{Div, Mul},
};

use super::{convert::ToScalar, ucum::Dimensionable};

/// Trait for determining if two things are dimensionally equal.
///
pub trait DimEq<Rhs = Self> {
    fn dim_eq(&self, rhs: &Rhs) -> bool;
}

impl<T> DimEq for T
where
    T: Dimensionable,
{
    fn dim_eq(&self, rhs: &Self) -> bool {
        self.dim() == rhs.dim()
    }
}

pub trait IsCommensurableWith<'a, Rhs = Self>: DimEq<Rhs> {
    fn is_commensurable_with(&'a self, rhs: &'a Rhs) -> Option<bool>;
}

impl<'a, T> IsCommensurableWith<'a> for T
where
    T: Dimensionable + ToScalar<f64> + 'a,
{
    fn is_commensurable_with(&'a self, rhs: &'a Self) -> Option<bool> {
        if !self.dim_eq(rhs) {
            return None;
        }

        Some(approx::ulps_eq!(&self.to_scalar(), &rhs.to_scalar()))
    }
}

pub trait CommensurableOrd<Rhs = Self>: DimEq<Rhs> {
    fn commensurable_ord(&self, rhs: &Rhs) -> Option<Ordering>;
}

impl<T> CommensurableOrd for T
where
    T: Dimensionable + ToScalar<f64>,
{
    fn commensurable_ord(&self, rhs: &Self) -> Option<Ordering> {
        if !self.dim_eq(rhs) {
            return None;
        }

        self.to_scalar().partial_cmp(&rhs.to_scalar())
    }
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
/// Also note that this operation *can't* fail.
///
pub trait DivRef<Rhs = Self, O = Self> {
    fn div_ref(&self, rhs: &Rhs) -> O;
}

pub trait TryDivRef<'a, Rhs = Self, O = Self>: Sized + Div<&'a Self, Output = O> + 'a {
    type Error;

    /// # Errors
    ///
    /// This crate's `Measurement` can always be divided by an object of its own type and return
    /// an object of its own type, but other implementations may not. For example, if one created a
    /// measurement type `Area`, dividing an object of such type by another object of the same type
    /// would result in a measurement without dimension, not another `Area`. ...thus it might make
    /// sense to implement this trait for that case such that it always errors.
    ///
    /// This trait mainly exists to provide a trait bounds for other traits like
    /// `v2::type_traits::Measurement`, where you _should_ have some sdfasdlfkjas;dlfkjas;dflj
    ///
    fn try_div_ref(&'a self, rhs: &'a Rhs) -> Result<O, Self::Error>;
}

pub trait CheckedDivRef<Rhs = Self, O = Self>: Sized {
    fn checked_div_ref(&self, rhs: &Rhs) -> Option<O>;
}

/// `std::ops::Mul::mul()` takes `self`, which makes sense for regular numbers
/// (which are `Copy`), but not for `Measurement` and `Unit` (which are not `Copy`).
/// This is more ergonomic for our types.
///
/// Also note that this operation *can't* fail.
///
pub trait MulRef<Rhs = Self, O = Self> {
    fn mul_ref(&self, rhs: &Rhs) -> O;
}

pub trait TryMulRef<'a, Rhs = Self, O = Self>: Sized + Mul<&'a Self, Output = O> + 'a {
    type Error;

    /// # Errors
    ///
    /// This crate's `Measurement` can always be multiplied by an object of its own type and return
    /// an object of its own type, but other implementations may not. For example, if one created a
    /// measurement type `Area`, multiplying an object of such type by another object of the same type
    /// would result in a measurement without dimension, not another `Area`. ...thus it might make
    /// sense to implement this trait for that case such that it always errors.
    ///
    /// This trait mainly exists to provide a trait bounds for other traits like
    /// `v2::type_traits::Measurement`, where you _should_ have some an implementation
    ///
    fn try_mul_ref(&'a self, rhs: &'a Rhs) -> Result<O, Self::Error>;
}

pub trait CheckedMulRef<Rhs = Self, O = Self>: Sized {
    fn checked_mul_ref(&self, rhs: &Rhs) -> Option<O>;
}
