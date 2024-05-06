mod annotation_composable;
mod composable;
mod display;
mod field_eq;
mod invert;
mod is_compatible_with;
pub(crate) mod num_traits;
mod partial_eq;
mod reducible;
mod ucum_unit;

use std::borrow::Cow;

use crate::{Atom, Prefix};

pub const UNITY: Term = {
    Term {
        atom: None,
        prefix: None,
        factor: Some(1),
        exponent: None,
        annotation: None,
    }
};
pub const UNITY_ARRAY: [Term; 1] = [UNITY];
pub const UNITY_ARRAY_REF: &[Term; 1] = &UNITY_ARRAY;

pub type Factor = u32;
pub type Exponent = i32;

/// A Term makes up an Atom (at its core) along with any Atom modifiers
/// (anything that can change its scalar). It is, however, possible to have an
/// Atom-less Term, which would simple be a Factor (with or without an
/// annotation) (ex. the 10 in "10" or "10/m" would be an Atom-less Term).
///
#[derive(Clone, Debug, Eq, Default)]
pub struct Term {
    pub factor: Option<Factor>,
    pub prefix: Option<Prefix>,
    pub atom: Option<Atom>,
    pub exponent: Option<Exponent>,
    pub annotation: Option<String>,
}

impl Term {
    #[must_use]
    pub const fn new(prefix: Option<Prefix>, atom: Option<Atom>) -> Self {
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
    #[deprecated(since = "1.0.0", note = "Please use term::UNITY instead")]
    #[must_use]
    pub const fn new_unity() -> Self {
        UNITY
    }

    /// A `Term` is a unity `Term` if represents "1", which technically means
    /// here:
    ///
    /// * its `factor` is 1
    /// * it has no `exponent`
    /// * it has no `Atom`
    /// * it has no `Prefix`
    ///
    /// NOTE: This does not check the annotation since that does not effect the
    /// quantity of the Term.
    ///
    #[must_use]
    pub fn is_unity(&self) -> bool {
        self.factor == Some(1_u32)
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
    #[must_use]
    pub const fn has_value(&self) -> bool {
        self.factor.is_some() || self.atom.is_some() || self.annotation.is_some()
    }

    /// If `self` has an `exponent`, it checks if its value is positive; if not, it returns `true`
    /// (since `None` is analogous to an exponent of 1).
    ///
    #[must_use]
    pub const fn exponent_is_positive(&self) -> bool {
        match self.exponent {
            Some(e) => e.is_positive(),
            // None is analogous to an exponent of 1.
            None => true,
        }
    }

    /// If `self` has an `exponent`, it checks if its value is negative; if not, it returns `false`
    /// (since `None` is analogous to an exponent of 1).
    ///
    #[must_use]
    pub const fn exponent_is_negative(&self) -> bool {
        match self.exponent {
            Some(e) => e.is_negative(),
            // None is analogous to an exponent of 1.
            None => false,
        }
    }

    pub fn factor_and_is_not_one<F: FnOnce(Factor)>(&self, f: F) {
        if let Some(factor) = self.factor {
            if factor != 1 {
                f(factor);
            }
        }
    }

    #[must_use]
    pub fn factor_as_u32(&self) -> Factor {
        self.factor.unwrap_or(1)
    }

    /// Depending on the `Term`, its string representation could be anywhere from a `&'static str`
    /// to a combination of all of its fields as a `String`. For those former cases, we want to
    /// allow borrowing the `&'static str` to save on allocations.
    ///
    #[must_use]
    pub fn as_cow_str(&self) -> Cow<'_, str> {
        use crate::UcumSymbol;

        if self.is_unity() && self.annotation.is_none() {
            return Cow::Borrowed("1");
        };

        match (
            self.factor,
            self.prefix,
            self.atom,
            self.exponent,
            self.annotation.as_deref(),
        ) {
            (None, None, None, None, None) => Cow::Borrowed(""),
            (None | Some(1), None, None, None | Some(1), Some(ann)) => {
                Cow::Owned(format!("{{{ann}}}"))
            }
            (None, None, Some(atom), None | Some(1), None) => Cow::Borrowed(atom.primary_code()),
            (None, None, Some(atom), None | Some(1), Some(ann)) => {
                Cow::Owned(format!("{atom}{{{ann}}}"))
            }
            (None, None, Some(atom), Some(exp), None) => Cow::Owned(format!("{atom}{exp}")),
            (None, None, Some(atom), Some(exp), Some(ann)) => {
                Cow::Owned(format!("{atom}{exp}{{{ann}}}"))
            }
            (None, Some(prefix), Some(atom), None | Some(1), None) => {
                Cow::Owned(format!("{prefix}{atom}"))
            }
            (None, Some(prefix), Some(atom), None | Some(1), Some(ann)) => {
                Cow::Owned(format!("{prefix}{atom}{{{ann}}}"))
            }
            (None, Some(prefix), Some(atom), Some(exp), None) => {
                Cow::Owned(format!("{prefix}{atom}{exp}"))
            }
            (None, Some(prefix), Some(atom), Some(exp), Some(ann)) => {
                Cow::Owned(format!("{prefix}{atom}{exp}{{{ann}}}"))
            }
            (Some(factor), None, None, None, None) => Cow::Owned(factor.to_string()),
            (Some(factor), None, None, None, Some(ann)) => Cow::Owned(format!("{factor}{{{ann}}}")),
            (Some(factor), None, Some(atom), None | Some(1), None) => {
                Cow::Owned(format!("{factor}{atom}"))
            }
            (Some(factor), None, Some(atom), None | Some(1), Some(ann)) => {
                Cow::Owned(format!("{factor}{atom}{{{ann}}}"))
            }
            (Some(factor), None, Some(atom), Some(exp), None) => {
                Cow::Owned(format!("{factor}{atom}{exp}"))
            }
            (Some(factor), None, Some(atom), Some(exp), Some(ann)) => {
                Cow::Owned(format!("{factor}{atom}{exp}{{{ann}}}"))
            }
            (Some(factor), Some(prefix), Some(atom), None | Some(1), None) => {
                Cow::Owned(format!("{factor}{prefix}{atom}"))
            }
            (Some(factor), Some(prefix), Some(atom), None | Some(1), Some(ann)) => {
                Cow::Owned(format!("{factor}{prefix}{atom}{{{ann}}}"))
            }
            (Some(factor), Some(prefix), Some(atom), Some(exp), None) => {
                Cow::Owned(format!("{factor}{prefix}{atom}{exp}"))
            }
            (Some(factor), Some(prefix), Some(atom), Some(exp), Some(ann)) => {
                Cow::Owned(format!("{factor}{prefix}{atom}{exp}{{{ann}}}"))
            }
            _ => unreachable!("Invalid Term: {self:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_new_unity() {
        assert_eq!(UNITY.to_string(), "1");
    }

    #[test]
    fn as_str_test() {
        assert_eq!(UNITY.as_cow_str(), "1");

        // None, None, None, None, None
        assert_eq!(Term::default().as_cow_str(), "");

        // None | Some(1), None, None, None, Some(ann)
        assert_eq!(term!(annotation: "hi".to_string()).as_cow_str(), "{hi}");
        assert_eq!(
            term!(factor: 1, annotation: "hi".to_string()).as_cow_str(),
            "{hi}"
        );

        // None, None, Some(atom), None | Some(1), None
        assert_eq!(term!(Meter).as_cow_str(), "m");
        assert_eq!(term!(Meter, exponent: 1).as_cow_str(), "m");

        // None, None, Some(atom), None | Some(1), Some(ann)
        assert_eq!(
            term!(Meter, annotation: "hi".to_string()).as_cow_str(),
            "m{hi}"
        );
        assert_eq!(
            term!(Meter, exponent: 1, annotation: "hi".to_string()).as_cow_str(),
            "m{hi}"
        );

        assert_eq!(term!(Meter, exponent: 2).as_cow_str(), "m2");
        assert_eq!(term!(Meter, exponent: -1).as_cow_str(), "m-1");

        assert_eq!(
            term!(Meter, exponent: 2, annotation: "hi".to_string()).as_cow_str(),
            "m2{hi}"
        );

        assert_eq!(term!(Kilo, Meter).as_cow_str(), "km");

        assert_eq!(
            term!(Kilo, Meter, annotation: "hi".to_string()).as_cow_str(),
            "km{hi}"
        );

        assert_eq!(
            term!(Kilo, Meter, exponent: 1, annotation: "hi".to_string()).as_cow_str(),
            "km{hi}"
        );
        assert_eq!(
            term!(Kilo, Meter, exponent: 2, annotation: "hi".to_string()).as_cow_str(),
            "km2{hi}"
        );
    }
}
