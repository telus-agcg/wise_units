pub mod as_fraction;
pub mod composable;
pub mod deref;
pub mod display;
pub mod field_eq;
pub mod from;
pub mod from_str;
pub mod is_compatible_with;
pub mod ops;
pub mod partial_eq;
pub mod partial_ord;
pub mod reducible;
mod term_reducing;
pub mod ucum_unit;

use crate::decomposer::{Decomposable, SimpleDecomposer};
use crate::parser::Term;

#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Unit {
    pub terms: Vec<Term>,
}

/// A `Unit` is the piece of data that represents a *valid* UCUM unit or
/// custom-defined unit. A `Unit` is defined as a number of `Term`s and thus
/// all methods defined on `Unit` rely on values from its `Terms`.
///
/// The easiest way to create a new `Unit` is via its implementation of
/// `std::str::FromStr`. This parses the given `&str` and returns a
/// `wise_units::parser::Error` if it fails to parse:
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
    /// Creates a new `Unit` that's equivalent to "1".
    ///
    pub fn new_unity() -> Self {
        let unity_term = Term::new_unity();

        Unit {
            terms: vec![unity_term],
        }
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
    pub fn is_unity(&self) -> bool {
        self.terms.len() == 1 && self.terms[0].is_unity()
    }

    /// Reduces `self`'s `Term`s into a new `Unit`, consuming `self`.
    ///
    /// ```
    /// use std::str::FromStr;
    /// use wise_units::Unit;
    ///
    /// // "m2" doesn't reduce down...
    /// let m1 = Unit::from_str("m2").unwrap();
    /// let m2 = Unit::from_str("m2").unwrap();
    /// assert_eq!(m1.into_reduced(), m2);
    ///
    /// // ...but "m4/m2" does.
    /// let m1 = Unit::from_str("m4/m2").unwrap();
    /// let m2 = Unit::from_str("m2").unwrap();
    /// assert_eq!(m1.into_reduced(), m2);
    /// ```
    ///
    #[inline]
    pub fn into_reduced(self) -> Unit {
        Unit {
            terms: term_reducing::reduce_terms(&self.terms),
        }
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
    /// assert_eq!(u.expression().as_str(), "[acr_us].[in_i]/[acr_us]");
    /// ```
    ///
    #[inline]
    pub fn expression(&self) -> String {
        let sd = SimpleDecomposer;

        sd.decompose(&self.terms)
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
    /// assert_eq!(u.expression_reduced().as_str(), "[in_i]");
    /// ```
    ///
    #[inline]
    pub fn expression_reduced(&self) -> String {
        let reduced = term_reducing::reduce_terms(&self.terms);
        let sd = SimpleDecomposer;

        sd.decompose(&reduced)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn validate_is_unity() {
        let unit = Unit::new_unity();
        assert!(unit.is_unity());

        let unit = Unit { terms: Vec::new() };
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
        assert_eq!(unit.expression_reduced().as_str(), "m");

        let unit = Unit::from_str("M").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "m");

        let unit = Unit::from_str("km/10m").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "km/10m");

        let unit = Unit::from_str("m-1").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "/m");

        let unit = Unit::from_str("10m").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "10m");

        let unit = Unit::from_str("10km").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "10km");

        let unit = Unit::from_str("10km-1").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "/10km");

        let unit = Unit::from_str("km-1/m2").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "/m2.km");

        let unit = Unit::from_str("km/m2.cm").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "km/m2.cm");

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "/m2.cm.km");

        let unit = Unit::from_str("m/s2").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "m/s2");

        let unit = Unit::from_str("km3/nm2").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "km3/nm2");
    }

    #[cfg(feature = "with_serde")]
    mod with_serde {
        use super::super::Unit;
        use crate::parser::{Atom, Prefix, Term};
        use serde_json;

        #[test]
        fn validate_serialization_empty_terms() {
            let unit = Unit { terms: vec![] };
            let expected_json = r#"{"terms":[]}"#;

            let j = serde_json::to_string(&unit).expect("Couldn't convert Unit to JSON String");

            assert_eq!(expected_json, j);
        }

        #[test]
        fn validate_serialization_full_terms() {
            let expected_json = r#"{
                "terms":[{
                    "atom": "Meter",
                    "prefix": "Centi",
                    "factor": 100,
                    "exponent": 456,
                    "annotation": "stuff"
                }, {
                    "atom": "Gram",
                    "prefix": null,
                    "factor": 1,
                    "exponent": -4,
                    "annotation": null
                }]
        }"#.replace("\n", "")
            .replace(" ", "");

            let term1 =
                term!(Centi, Meter, factor: 100, exponent: 456, annotation: "stuff".to_string());
            let term2 = term!(Gram, factor: 1, exponent: -4);

            let unit = Unit {
                terms: vec![term1, term2],
            };

            let j = serde_json::to_string(&unit).expect("Couldn't convert Unit to JSON String");

            assert_eq!(expected_json, j);
        }

        #[test]
        fn validate_deserialization_empty_terms() {
            let json = r#"{"terms": []}"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Unit");

            let expected_unit = Unit { terms: vec![] };

            assert_eq!(expected_unit, k);
        }

        #[test]
        fn validate_deserialization_full_terms() {
            let json = r#"{
                "terms":[{
                    "atom": "Meter",
                    "prefix": "Centi",
                    "factor": 100,
                    "exponent": 456,
                    "annotation": "stuff"
                }, {
                    "atom": "Gram",
                    "prefix": null,
                    "factor": 1,
                    "exponent": -4,
                    "annotation": null
                }]
            }"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Unit");

            let term1 =
                term!(Centi, Meter, factor: 100, exponent: 456, annotation: "stuff".to_string());
            let term2 = term!(Gram, exponent: -4);

            let expected_unit = Unit {
                terms: vec![term1, term2],
            };

            assert_eq!(expected_unit, k);
        }
    }

    #[test]
    fn into_reduced() {
        fn validate(input: &str, expected: &str) {
            let unit = Unit::from_str(input).unwrap();
            let actual = unit.into_reduced();
            let expected = Unit::from_str(expected).unwrap();

            assert_eq!(&actual, &expected);
            assert_eq!(actual.expression(), expected.expression());
        }

        validate("m", "m");
        validate("m2/m", "m");
        validate("100m2/m", "100m2/m");
        validate("m2.m2", "m4");
        validate("m2.m2/s.s", "m4/s2");
        validate("m2.s/s.m2", "1");
    }
}
