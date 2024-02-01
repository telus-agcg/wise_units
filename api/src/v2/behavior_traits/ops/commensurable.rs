use std::cmp::Ordering;

use crate::v2::behavior_traits::{convert::TryToScalar, dim::Dimensionable};

/// Trait to determine if two, typically `Measurement`s are the same quantity.
///
/// Note that while the implementation for most cases should involve:
/// 1. Checking if compared items are in the same dimension; if not, they cannot
///    be compared.
/// 2. Checking if the scalar values of compared items are equal; if so, the
///    items are commensurable.
///
/// We explicitly don't provide generic implementations for any value type `V`,
/// since the manner of checking the equality of `V` can differ depending on `V`.
/// For example, we _do_ provide an implementation for `V` as `f64`, and in that
/// case, we use the [Unit in the last place (ULP)](https://en.wikipedia.org/wiki/Unit_in_the_last_place)
/// method for comparison. Comparing other value types may merit other methods.
///
/// ```
/// use wise_units::Measurement;
/// use wise_units::v2::behavior_traits::ops::commensurable::IsCommensurableWith;
///
/// let one_km = Measurement::try_new(1.0, "km").unwrap();
/// let two_km = Measurement::try_new(2.0, "km").unwrap();
/// let thousand_m = Measurement::try_new(1000.0, "m").unwrap();
///
/// assert!(one_km.is_commensurable_with(&one_km).unwrap());
/// assert!(one_km.is_commensurable_with(&thousand_m).unwrap());
/// assert!(!one_km.is_commensurable_with(&two_km).unwrap());
///
/// ```
pub trait IsCommensurableWith<'a, V, Rhs = Self>: Dimensionable {
    fn is_commensurable_with(&'a self, rhs: &'a Rhs) -> Option<bool>;
}

impl<'a, T> IsCommensurableWith<'a, f64> for T
where
    T: Dimensionable + TryToScalar<f64> + 'a,
{
    fn is_commensurable_with(&'a self, rhs: &'a Self) -> Option<bool> {
        if !self.dim_eq(rhs) {
            return None;
        }

        match (self.try_to_scalar(), rhs.try_to_scalar()) {
            (Ok(lhs), Ok(rhs)) => Some(approx::ulps_eq!(lhs, rhs)),
            (_, _) => None,
        }
    }
}

/// This trait exists to allow for `PartialOrd` implementations that check for
/// actual object equality, not semantic equality, which thus allows for proper
/// behavior of `Hash` (and thus, say, `HashMap`) for those objects. We can then
/// determine ordering for commensurable things using this trait's implementation.
///
pub trait CommensurableOrd<Rhs = Self>: Dimensionable {
    fn commensurable_ord(&self, rhs: &Rhs) -> Option<Ordering>;
}

impl<T> CommensurableOrd for T
where
    T: Dimensionable + TryToScalar<f64>,
{
    fn commensurable_ord(&self, rhs: &Self) -> Option<Ordering> {
        if !self.dim_eq(rhs) {
            return None;
        }

        match (self.try_to_scalar(), rhs.try_to_scalar()) {
            (Ok(lhs), Ok(ref rhs)) => lhs.partial_cmp(rhs),
            (_, _) => None,
        }
    }
}

pub trait CommensurableAdd<'a, O = Self, Rhs = Self>: Dimensionable {
    fn commensurable_add(&'a self, rhs: &'a Rhs) -> Option<O>;
}

// pub trait CommensurableAdd<'a, V, O = Self, Rhs = Self>: Dimensionable<Rhs> {
//     // fn commensurable_add(&'a self, rhs: &'a Rhs) -> Option<O> {
//     //     if !self.dim_eq(rhs) {
//     //         return None;
//     //     }
//     //
//     //     let rhs_converted = rhs.convert_to(&self)?;
//     //     // let new_value = lhs.value + rhs_converted.value;
//     //
//     //     match (self.try_to_scalar(), rhs_converted.try_to_scalar()) {
//     //         (Ok(lhs), Ok(rhs)) => {
//     //             todo!()
//     //         }
//     //         (_, _) => None,
//     //     }
//     // }
// }

// impl<'a, T, E> CommensurableAdd<'a> for T
// where
//     T: 'a,
//     &'a T: Add<Output = Result<T, E>>,
// {
//     type Error = E;
//
//     fn commensurable_add(&'a self, rhs: &'a Self) -> Result<T, Self::Error> {
//         Add::add(self, rhs)
//     }
// }

// /// `std::ops::Sub::sub()` takes `self`, which makes sense for regular numbers
// /// (which are `Copy`), but not for `Measurement` and `Unit` (which are not `Copy`).
// /// This is more ergonomic for our types.
// ///
// pub trait SubRef<'a, O = Self, Rhs = Self> {
//     fn sub_ref(&'a self, rhs: &'a Rhs) -> O;
// }
//
// // impl<'a, T, O> SubRef<'a, O> for T
// // where
// //     T: TrySubRef<'a, O, Error = Infallible>,
// // {
// //     fn sub_ref(&'a self, rhs: &'a Self) -> O {
// //         <Self as TrySubRef>::try_sub_ref(self, rhs).expect("infallible")
// //     }
// // }
//
// pub trait TrySubRef<'a, O = Self, Rhs = Self> {
//     type Error;
//
//     /// # Errors
//     ///
//     /// In order for Measurements to be subtractable, they need to of the same dimension;
//     /// any errors here are most likely the result of trying to convert `rhs` to be of the
//     /// same dimension as `self`.
//     ///
//     fn try_sub_ref(&'a self, rhs: &'a Rhs) -> Result<O, Self::Error>;
// }
//
// impl<'a, T, E> TrySubRef<'a> for T
// where
//     T: 'a,
//     &'a T: Sub<Output = Result<T, E>>,
// {
//     type Error = E;
//
//     fn try_sub_ref(&'a self, rhs: &'a Self) -> Result<T, Self::Error> {
//         Sub::sub(self, rhs)
//     }
// }
//
// /// `std::ops::Div::div()` takes `self`, which makes sense for regular numbers
// /// (which are `Copy`), but not for `Measurement` and `Unit` (which are not `Copy`).
// /// This is more ergonomic for our types.
// ///
// /// Also note that this operation *can't* fail.
// ///
// pub trait DivRef<'a, O = Self, Rhs = Self> {
//     fn div_ref(&'a self, rhs: &'a Rhs) -> O;
// }
//
// // impl<'a, T, O> DivRef<'a, O> for T
// // where
// //     T: 'a,
// //     &'a T: Div<Output = O>,
// // {
// //     fn div_ref(&'a self, rhs: &'a Self) -> O {
// //         Div::div(self, rhs)
// //     }
// // }
//
// pub trait TryDivRef<'a, O = Self, Rhs = Self> {
//     type Error;
//
//     /// # Errors
//     ///
//     /// This crate's `Measurement` can always be divided by an object of its own type and return
//     /// an object of its own type, but other implementations may not. For example, if one created a
//     /// measurement type `Area`, dividing an object of such type by another object of the same type
//     /// would result in a measurement without dimension, not another `Area`. ...thus it might make
//     /// sense to implement this trait for that case such that it always errors.
//     ///
//     /// This trait mainly exists to provide a trait bounds for other traits like
//     /// `v2::type_traits::Measurement`, where you _should_ have some sdfasdlfkjas;dlfkjas;dflj
//     ///
//     fn try_div_ref(&'a self, rhs: &'a Rhs) -> Result<O, Self::Error>;
// }
//
// // impl<'a, T, O> TryDivRef<'a, O> for T
// // where
// //     T: DivRef<'a, O>,
// // {
// //     type Error = Infallible;
// //
// //     fn try_div_ref(&'a self, rhs: &'a Self) -> Result<O, Self::Error> {
// //         Ok(DivRef::div_ref(self, rhs))
// //     }
// // }
//
// /// `std::ops::Mul::mul()` takes `self`, which makes sense for regular numbers
// /// (which are `Copy`), but not for `Measurement` and `Unit` (which are not `Copy`).
// /// This is more ergonomic for our types.
// ///
// /// Also note that this operation *can't* fail.
// ///
// pub trait MulRef<'a, O = Self, Rhs = Self> {
//     fn mul_ref(&'a self, rhs: &'a Rhs) -> O;
// }
//
// // impl<'a, T, O> MulRef<'a, O> for T
// // where
// //     T: 'a,
// //     &'a T: Mul<Output = O>,
// // {
// //     fn mul_ref(&'a self, rhs: &'a Self) -> O {
// //         Mul::mul(self, rhs)
// //     }
// // }
// //
// pub trait TryMulRef<'a, O = Self, Rhs = Self> {
//     type Error;
//
//     /// # Errors
//     ///
//     /// This crate's `Measurement` can always be multiplied by an object of its own type and return
//     /// an object of its own type, but other implementations may not. For example, if one created a
//     /// measurement type `Area`, multiplying an object of such type by another object of the same type
//     /// would result in a measurement without dimension, not another `Area`. ...thus it might make
//     /// sense to implement this trait for that case such that it always errors.
//     ///
//     /// This trait mainly exists to provide a trait bounds for other traits like
//     /// `v2::type_traits::Measurement`, where you _should_ have some an implementation
//     ///
//     fn try_mul_ref(&'a self, rhs: &'a Rhs) -> Result<O, Self::Error>;
// }
//
// // impl<'a, T, O> TryMulRef<'a, O> for T
// // where
// //     T: MulRef<'a, O>,
// // {
// //     type Error = Infallible;
// //
// //     fn try_mul_ref(&'a self, rhs: &'a Self) -> Result<O, Self::Error> {
// //         Ok(MulRef::mul_ref(self, rhs))
// //     }
// // }
