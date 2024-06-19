mod as_fraction;
mod composable;
mod deref;
mod display;
mod field_eq;
mod from_str;
mod invert;
mod is_compatible_with;
mod num_traits;
mod ops;
mod parser;
mod partial_eq;
mod partial_ord;
mod reducible;
mod term_reducing;
mod to_reduced;
#[cfg(feature = "v2")]
mod v2;

#[allow(clippy::module_name_repetitions)]
mod ucum_unit;

#[cfg(feature = "cffi")]
pub mod custom_ffi;

#[cfg(feature = "serde")]
mod serde;

use std::{borrow::Cow, str::FromStr};

#[cfg(feature = "cffi")]
use ffi_common::derive::FFI;

pub(crate) use self::parser::Error as ParserError;

use crate::{term, Error, Term};

pub const UNITY: Unit = Unit {
    terms: Cow::Borrowed(term::UNITY_ARRAY_REF),
};

#[cfg_attr(
    feature = "cffi",
    derive(FFI),
    ffi(
        custom = "src/unit/custom_ffi.rs",
        failable_init,
        failable_fns(custom_ffi::get_unit_expression)
    )
)]
#[derive(Clone, Debug)]
pub struct Unit {
    pub(crate) terms: Cow<'static, [Term]>,
}

/// A `Unit` is the piece of data that represents a *valid* UCUM unit or
/// custom-defined unit. A `Unit` is defined as a number of `Term`s and thus
/// all methods defined on `Unit` rely on values from its `Terms`.
///
/// The easiest way to create a new `Unit` is via its implementation of
/// `std::str::FromStr`. This parses the given `&str` and returns a
/// `wise_units::error::Error` if it fails to parse:
///
/// ```rust
/// use std::str::FromStr;
/// use wise_units::{UcumUnit, Unit};
///
/// let m = Unit::from_str("m2").unwrap();
/// assert_eq!(m.scalar(), 1.0);
///
/// let bad_unit = Unit::from_str("not_a_unit");
/// assert!(bad_unit.is_err());
/// ```
///
impl Unit {
    /// ```
    /// use wise_units::{Unit, Term, Prefix, Atom};
    ///
    /// let km_term = Term::new(Some(Prefix::Kilo), Some(Atom::Meter));
    /// let km_unit = Unit::new(vec![km_term]);
    ///
    /// assert_eq!(km_unit.to_string(), "km");
    /// ```
    ///
    #[must_use]
    pub fn new<T>(terms: T) -> Self
    where
        Cow<'static, [Term]>: From<T>,
    {
        Self {
            terms: Cow::from(terms),
        }
    }

    /// Creates a new `Unit` that's equivalent to "1".
    ///
    #[deprecated(since = "1.0.0", note = "Please use unit::UNITY instead")]
    #[must_use]
    pub const fn new_unity() -> Self {
        UNITY
    }

    /// Accessor for the `Term`s used that make up this `Unit`.
    ///
    #[must_use]
    pub const fn terms(&self) -> &Cow<'static, [Term]> {
        &self.terms
    }

    #[must_use]
    pub fn into_terms(self) -> Cow<'static, [Term]> {
        self.terms
    }

    /// A `Unit` is a unity `Unit` if represents "1", which technically means
    /// here:
    ///
    /// * it has 1 `Term`...
    ///     * whose `factor` is 1
    ///     * has no `exponent`
    ///     * has no `Atom`
    ///     * has no `Prefix`
    ///
    #[must_use]
    pub fn is_unity(&self) -> bool {
        &*self.terms == term::UNITY_ARRAY_REF
    }

    /// Turns the Unit's Terms into Strings and combines them accordingly.
    /// This always returns a String that is parsable back into the same Unit.
    ///
    /// Ex. terms that would normally render `[acr_us].[in_i]/[acr_us]` would
    /// render the same result.
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use wise_units::Unit;
    ///
    /// let u = Unit::from_str("[acr_us].[in_i]/[acr_us]").unwrap();
    /// assert_eq!(u.expression(), "[acr_us].[in_i]/[acr_us]");
    /// ```
    ///
    #[inline]
    #[must_use]
    pub fn expression(&self) -> String {
        self.to_string()
    }

    /// If the unit terms are a fraction and can be reduced, this returns those
    /// as a string. Ex. terms that would normally render
    /// `[acr_us].[in_i]/[acr_us]` would simply render `[in_i]`.
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use wise_units::Unit;
    ///
    /// let u = Unit::from_str("[acr_us].[in_i]/[acr_us]").unwrap();
    /// assert_eq!(u.expression_reduced(), "[in_i]");
    /// ```
    ///
    #[inline]
    #[must_use]
    pub fn expression_reduced(&self) -> String {
        let reduced = term_reducing::reduce_terms(self.terms.to_vec());

        Self::new(reduced).to_string()
    }
}

// TODO: This is silly; remove.
//
impl AsRef<Self> for Unit {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'a> TryFrom<&'a str> for Unit {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn validate_is_unity() {
        let unit = UNITY;
        assert!(unit.is_unity());

        let unit = Unit::new(Vec::new());
        assert!(!unit.is_unity());

        let unit = Unit::from_str("1").unwrap();
        assert!(unit.is_unity());

        let unit = Unit::from_str("m").unwrap();
        assert!(!unit.is_unity());

        let unit = Unit::from_str("m/m").unwrap();
        assert!(!unit.is_unity());
    }

    #[test]
    fn validate_expression_reduced() {
        let unit = Unit::from_str("m").unwrap();
        assert_eq!(unit.expression_reduced(), "m");

        let unit = Unit::from_str("M").unwrap();
        assert_eq!(unit.expression_reduced(), "m");

        let unit = Unit::from_str("km/10m").unwrap();
        assert_eq!(unit.expression_reduced(), "km/10m");

        let unit = Unit::from_str("m-1").unwrap();
        assert_eq!(unit.expression_reduced(), "/m");

        let unit = Unit::from_str("10m").unwrap();
        assert_eq!(unit.expression_reduced(), "10m");

        let unit = Unit::from_str("10km").unwrap();
        assert_eq!(unit.expression_reduced(), "10km");

        let unit = Unit::from_str("10km-1").unwrap();
        assert_eq!(unit.expression_reduced(), "/10km");

        let unit = Unit::from_str("km-1/m2").unwrap();
        assert_eq!(unit.expression_reduced(), "/m2.km");

        let unit = Unit::from_str("km/m2.cm").unwrap();
        assert_eq!(unit.expression_reduced(), "km/m2.cm");

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        assert_eq!(unit.expression_reduced(), "/m2.cm.km");

        let unit = Unit::from_str("m/s2").unwrap();
        assert_eq!(unit.expression_reduced(), "m/s2");

        let unit = Unit::from_str("km3/nm2").unwrap();
        assert_eq!(unit.expression_reduced(), "km3/nm2");

        let unit = Unit::from_str("Kibit").unwrap();
        assert_eq!(unit.expression_reduced(), "Kibit");

        let unit = Unit::from_str("KiBy").unwrap();
        assert_eq!(unit.expression_reduced(), "KiBy");

        let unit = Unit::from_str("MiBy").unwrap();
        assert_eq!(unit.expression_reduced(), "MiBy");

        let unit = Unit::from_str("GiBy").unwrap();
        assert_eq!(unit.expression_reduced(), "GiBy");

        let unit = Unit::from_str("TiBy").unwrap();
        assert_eq!(unit.expression_reduced(), "TiBy");
    }

    #[cfg(feature = "cffi")]
    mod cffi {
        use super::*;
        use ffi_common::core;

        #[test]
        fn test_custom_and_derived_ffi() {
            let expression = "kg/[lb_av]";
            let unit = unsafe { custom_ffi::unit_init(core::ffi_string!(expression)) };
            let c_expression = unsafe { custom_ffi::get_unit_expression(unit) };
            assert_eq!(expression, unsafe {
                core::string::string_from_c(c_expression)
            });
            unsafe { unit_ffi::unit_free(unit) };
        }
    }
}
