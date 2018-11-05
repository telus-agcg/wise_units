mod annotation_composable;
mod composable;
mod display;
mod is_compatible_with;
mod reducible;
mod ucum_unit;

use crate::parser::{Atom, Prefix};

/// A Term makes up an Atom (at its core) along with any Atom modifiers
/// (anything that can change its scalar). It is, however, possible to have an
/// Atom-less Term, which would simple be a Factor (with or without an
/// annotation) (ex. the 10 in "10" or "10/m" would be an Atom-less Term).
///
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Term {
    pub atom: Option<Atom>,
    pub prefix: Option<Prefix>,
    pub factor: Option<u32>,
    pub exponent: Option<i32>,
    pub annotation: Option<String>,
}

impl Term {
    pub fn new(prefix: Option<Prefix>, atom: Option<Atom>) -> Self {
        Self {
            atom,
            prefix,
            factor: None,
            exponent: None,
            annotation: None,
        }
    }

    /// Creates a new `Term` that's equivalent to the unit "1".
    ///
    pub fn new_unity() -> Self {
        Self {
            atom: None,
            prefix: None,
            factor: Some(1),
            exponent: None,
            annotation: None,
        }
    }

    /// A `Term` is a unity `Term` if represents "1", which technically means
    /// here:
    ///
    /// * its `factor` is 1
    /// * it has no `exponent`
    /// * it has no `Atom`
    /// * it has no `Prefix`
    ///
    pub fn is_unity(&self) -> bool {
        self.factor == Some(1u32)
            && self.exponent.is_none()
            && self.atom.is_none()
            && self.prefix.is_none()
    }

    /// A `Term` is valueless if its fields don't contain data that turns the `Term` into something
    /// with value:
    ///
    /// * it has no `factor`
    /// * it has no `Atom`
    /// * it has no `Prefix`
    ///
    /// A `Term` with _only_ an `exponent` doesn't really make any sense. A `Term` with an
    /// `annotation` certainly makes sense, but does not impact the value of anything.
    ///
    pub fn has_value(&self) -> bool {
        self.factor.is_some() || self.atom.is_some() || self.annotation.is_some()
    }

    /// If `self` has an `exponent`, it negates that value; if not, it sets it to `-1` (since
    /// `None` is analogous to an exponent of 1).
    ///
    pub fn invert_exponent(&mut self) {
        self.exponent = match self.exponent {
            Some(exponent) => Some(-exponent),
            // None is analogous to an exponent of 1.
            None => Some(-1),
        };
    }

    /// If `self` has an `exponent`, it checks if its value is positive; if not, it returns `true`
    /// (since `None` is analogous to an exponent of 1).
    ///
    pub fn exponent_is_positive(&self) -> bool {
        match self.exponent {
            Some(e) => e.is_positive(),
            // None is analogous to an exponent of 1.
            None => true,
        }
    }

    /// If `self` has an `exponent`, it checks if its value is negative; if not, it returns `false`
    /// (since `None` is analogous to an exponent of 1).
    ///
    pub fn exponent_is_negative(&self) -> bool {
        match self.exponent {
            Some(e) => e.is_negative(),
            // None is analogous to an exponent of 1.
            None => false,
        }
    }

    pub fn factor_and_is_not_one<F: FnOnce(u32)>(&self, f: F) {
        if let Some(factor) = self.factor {
            if factor != 1 {
                f(factor)
            }
        }
    }

    pub fn factor_as_u32(&self) -> u32 {
        match self.factor {
            Some(f) => f,
            None => 1,
        }
    }
}

impl ::std::default::Default for Term {
    fn default() -> Self {
        Self {
            prefix: None,
            atom: None,
            factor: None,
            exponent: None,
            annotation: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Term;

    #[test]
    fn validate_new_unity() {
        let term = Term::new_unity();
        assert_eq!(term.to_string(), "1");
    }

    #[cfg(feature = "with_serde")]
    mod with_serde {
        use super::super::Term;
        use parser::{Atom, Prefix};
        use serde_json;

        #[test]
        fn validate_serialization_empty_term() {
            let term = term!();

            let expected_json = r#"{
                                    "atom": null,
                                    "prefix": null,
                                    "factor": null,
                                    "exponent": null,
                                    "annotation": null
                                   }"#.replace("\n", "")
            .replace(" ", "");

            let j = serde_json::to_string(&term).expect("Couldn't convert Term to JSON String");

            assert_eq!(expected_json.as_str(), j);
        }

        #[test]
        fn validate_serialization_full_term() {
            let mut term = term!(Kilo, Meter, factor: 123, exponent: -456);
            term.annotation = Some("stuff".to_string());

            let expected_json = r#"{
                                    "atom": "Meter",
                                    "prefix": "Kilo",
                                    "factor": 123,
                                    "exponent": -456,
                                    "annotation": "stuff"
                                   }"#.replace("\n", "")
            .replace(" ", "");

            let j = serde_json::to_string(&term).expect("Couldn't convert Term to JSON String");

            assert_eq!(expected_json.as_str(), j);
        }

        #[test]
        fn validate_deserialization_empty_term() {
            let json = r#"{
                            "atom": null,
                            "prefix": null,
                            "factor": null,
                            "exponent": null,
                            "annotation": null
                           }"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Term");

            let expected_term = term!();

            assert_eq!(expected_term, k);
        }

        #[test]
        fn validate_deserialization_full_term() {
            let json = r#"{
                            "atom": "Meter",
                            "prefix": "Kilo",
                            "factor": 123,
                            "exponent": -456,
                            "annotation": "stuff"
                           }"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Term");

            let mut expected_term = term!(Kilo, Meter, factor: 123, exponent: -456);
            expected_term.annotation = Some("stuff".to_string());

            assert_eq!(expected_term, k);
        }
    }
}
