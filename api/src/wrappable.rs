//! Traits in here are intended for building wrappers in other languages around
//! `wise_units` (v1).
//!
pub mod atom;
pub mod measurement;
pub mod prefix;
pub mod property;
pub mod term;
pub mod unit;

pub use self::atom::WrapperAtom;
pub use self::measurement::WrapperMeasurement;
pub use self::prefix::WrapperPrefix;
pub use self::property::WrapperProperty;
pub use self::term::WrapperTerm;
pub use self::unit::WrapperUnit;

pub trait Ucum {
    type Composition;

    fn dim(&self) -> Self::Composition;
    fn is_special(&self) -> bool;
    fn is_metric(&self) -> bool;
    fn is_arbitrary(&self) -> bool;
}

pub trait TryToNumber {
    type Number;
    type ConversionError;

    /// Convert the Atom to a scalar as a `Self::Number`.
    ///
    /// # Errors
    ///
    /// Depending on `Self::Number`, a safe conversion may not be possible; an
    /// error here indicates that.
    ///
    fn try_to_scalar(&self) -> Result<Self::Number, Self::ConversionError>;

    /// Convert the Atom to a magnitude as a `Self::Number`.
    ///
    /// # Errors
    ///
    /// Depending on `Self::Number`, a safe conversion may not be possible; an
    /// error here indicates that.
    ///
    fn try_to_magnitude(&self) -> Result<Self::Number, Self::ConversionError>;
}

pub trait Commensurability {
    type OpRhs;

    fn is_compatible_with(&self, rhs: &Self::OpRhs) -> bool;
}

pub trait Compare {
    type OpRhs;
    type Ordering;

    fn coerced_eq(&self, rhs: &Self::OpRhs) -> bool;
    fn hash_eq(&self, rhs: &Self::OpRhs) -> bool;
    fn semantic_ord(&self, rhs: &Self::OpRhs) -> Self::Ordering;
    fn hash_ord(&self, rhs: &Self::OpRhs) -> Self::Ordering;
}

pub trait AddSub {
    type OpRhs;
    type Output;

    fn add(&self, rhs: Self::OpRhs) -> Self::Output;
    fn sub(&self, rhs: Self::OpRhs) -> Self::Output;
}

pub trait MulDiv {
    type OpRhs;
    type Output;

    fn mul(&self, rhs: Self::OpRhs) -> Self::Output;
    fn div(&self, rhs: Self::OpRhs) -> Self::Output;
}
