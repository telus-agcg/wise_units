mod annotation_composable;
mod composable;
mod display;
mod field_eq;
mod invert;
mod is_compatible_with;
pub(crate) mod num_traits;
mod partial_eq;
mod partial_ord;
mod reducible;
mod ucum_unit;
#[cfg(feature = "v2")]
mod v2;
pub(crate) mod variants;

use std::borrow::Cow;

use crate::{Annotation, Atom, Prefix};

use self::variants::*;

pub const UNITY: Term = Term::Factor(1);
pub const UNITY_ARRAY: [Term; 1] = [UNITY];
pub const UNITY_ARRAY_REF: &[Term; 1] = &UNITY_ARRAY;

pub type Factor = u32;
pub type Exponent = i32;

impl From<Factor> for Term {
    fn from(value: Factor) -> Self {
        Self::Factor(value)
    }
}

impl From<Atom> for Term {
    fn from(value: Atom) -> Self {
        Self::Atom(value)
    }
}

/// A Term makes up an Atom (at its core) along with any Atom modifiers
/// (anything that can change its scalar). It is, however, possible to have an
/// Atom-less Term, which would simple be a Factor (with or without an
/// annotation) (ex. the 10 in "10" or "10/m" would be an Atom-less Term).
///
/// Interesting notes from the spec:
/// - It's valid to have a unit string with just a factor and exponent: `2+10` or `2-10`
///   (see [the UCUM spec](https://unitsofmeasure.org/ucum#section-Syntax-Rules), under §9 exponents)
/// - Since the above is valid, and an annotation without any symbol implies the unity unit, it's
///   possible to raise a Term such as this to a power. ex. `{foo}` raised to the 2nd power becomes
///   `1+2{foo}`; raised to the -2nd power becomes `1-2{foo}`. Notice that a factor, exponent and
///   annotation are all required to represent that.
///
#[derive(Clone, Debug, Eq)]
pub enum Term {
    /// Ex. "{tree}", where the inner `String` would be the contents inside the curly braces.
    ///
    Annotation(Annotation),

    /// Ex. `[ft_i]`
    ///
    Atom(Atom),

    /// Ex. `g{sucrose}`
    ///
    AtomAnnotation(AtomAnnotation),

    /// Ex. `m2`
    ///
    AtomExponent(AtomExponent),

    /// Ex. `m2{peaches}`
    ///
    AtomExponentAnnotation(AtomExponentAnnotation),

    /// Ex. `km`
    ///
    PrefixAtom(PrefixAtom),

    /// Ex. `km{road}`
    ///
    PrefixAtomAnnotation(PrefixAtomAnnotation),

    /// Ex. `cm3`
    ///
    PrefixAtomExponent(PrefixAtomExponent),

    /// Ex. `cm3{water}`
    ///
    PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation),

    /// Ex. `100`
    ///
    Factor(Factor),

    /// Ex. `100{tree}`
    ///
    FactorAnnotation(FactorAnnotation),

    /// Ex. `2+5` or `2-5`
    ///
    FactorExponent(FactorExponent),

    /// Ex. `2+5{box}`
    ///
    FactorExponentAnnotation(FactorExponentAnnotation),

    /// Ex. `10L`
    ///
    FactorAtom(FactorAtom),

    /// Ex. `10L{gas}`
    ///
    FactorAtomAnnotation(FactorAtomAnnotation),

    /// Ex. `10[in_i]3`
    ///
    FactorAtomExponent(FactorAtomExponent),

    /// Ex. `10[in_i]3{oil}`
    ///
    FactorAtomExponentAnnotation(FactorAtomExponentAnnotation),

    /// Ex. `10cm`
    ///
    FactorPrefixAtom(FactorPrefixAtom),

    /// Ex. `10cm{wire}`
    ///
    FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation),

    /// Ex. `10cm3`
    ///
    FactorPrefixAtomExponent(FactorPrefixAtomExponent),

    /// Ex. `10cm3{solution}`
    ///
    FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation),
}

impl Term {
    /// # Panics
    ///
    /// Since it's invalid, in practice, this panics if you pass only a `prefix`.
    ///
    #[must_use]
    pub fn new(prefix: Option<Prefix>, atom: Option<Atom>) -> Self {
        match (prefix, atom) {
            (None, None) => Self::Factor(1),
            (None, Some(a)) => Self::Atom(a),
            (Some(p), None) => panic!("Can't instantiate Term with only a Prefix: {p}"),
            (Some(p), Some(a)) => Self::PrefixAtom(PrefixAtom::new(p, a)),
        }
    }

    /// Creates a new `Term` that's equivalent to the unit "1".
    ///
    #[deprecated(since = "1.0.0", note = "Please use term::UNITY instead")]
    #[must_use]
    pub const fn new_unity() -> Self {
        UNITY
    }

    #[must_use]
    pub const fn factor(&self) -> Option<Factor> {
        match self {
            Self::Annotation(_)
            | Self::Atom(_)
            | Self::AtomAnnotation { .. }
            | Self::AtomExponent { .. }
            | Self::AtomExponentAnnotation { .. }
            | Self::PrefixAtom { .. }
            | Self::PrefixAtomAnnotation { .. }
            | Self::PrefixAtomExponent { .. }
            | Self::PrefixAtomExponentAnnotation { .. } => None,
            Self::Factor(factor)
            | Self::FactorAnnotation(FactorAnnotation { factor, .. })
            | Self::FactorExponentAnnotation(FactorExponentAnnotation { factor, .. })
            | Self::FactorExponent(FactorExponent { factor, .. })
            | Self::FactorAtom(FactorAtom { factor, .. })
            | Self::FactorAtomAnnotation(FactorAtomAnnotation { factor, .. })
            | Self::FactorAtomExponent(FactorAtomExponent { factor, .. })
            | Self::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation { factor, .. })
            | Self::FactorPrefixAtom(FactorPrefixAtom { factor, .. })
            | Self::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation { factor, .. })
            | Self::FactorPrefixAtomExponent(FactorPrefixAtomExponent { factor, .. })
            | Self::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                factor,
                ..
            }) => Some(*factor),
        }
    }

    pub(crate) fn set_factor(&mut self, new_factor: Factor) -> &mut Self {
        match self {
            Self::Annotation(annotation) => {
                *self = Self::FactorAnnotation(annotation.assign_factor_mut(new_factor));
            }
            Self::Atom(atom) => {
                *self = Self::FactorAtom(atom.assign_factor_mut(new_factor));
            }
            Self::AtomAnnotation(inner) => {
                *self = Self::FactorAtomAnnotation(inner.assign_factor_mut(new_factor));
            }
            Self::AtomExponent(inner) => {
                *self = Self::FactorAtomExponent(inner.assign_factor_mut(new_factor));
            }
            Self::AtomExponentAnnotation(inner) => {
                *self = Self::FactorAtomExponentAnnotation(inner.assign_factor_mut(new_factor));
            }
            Self::PrefixAtom(inner) => {
                *self = Self::FactorPrefixAtom(inner.assign_factor_mut(new_factor));
            }
            Self::PrefixAtomAnnotation(inner) => {
                *self = Self::FactorPrefixAtomAnnotation(inner.assign_factor_mut(new_factor));
            }
            Self::PrefixAtomExponent(inner) => {
                *self = Self::FactorPrefixAtomExponent(inner.assign_factor_mut(new_factor));
            }
            Self::PrefixAtomExponentAnnotation(inner) => {
                *self =
                    Self::FactorPrefixAtomExponentAnnotation(inner.assign_factor_mut(new_factor));
            }
            Self::Factor(factor)
            | Self::FactorAnnotation(FactorAnnotation { factor, .. })
            | Self::FactorExponent(FactorExponent { factor, .. })
            | Self::FactorExponentAnnotation(FactorExponentAnnotation { factor, .. })
            | Self::FactorAtom(FactorAtom { factor, .. })
            | Self::FactorAtomAnnotation(FactorAtomAnnotation { factor, .. })
            | Self::FactorAtomExponent(FactorAtomExponent { factor, .. })
            | Self::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation { factor, .. })
            | Self::FactorPrefixAtom(FactorPrefixAtom { factor, .. })
            | Self::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation { factor, .. })
            | Self::FactorPrefixAtomExponent(FactorPrefixAtomExponent { factor, .. })
            | Self::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                factor,
                ..
            }) => *factor = new_factor,
        }
        self
    }

    #[must_use]
    pub const fn prefix(&self) -> Option<Prefix> {
        match self {
            Self::Annotation(_)
            | Self::Atom(_)
            | Self::AtomAnnotation { .. }
            | Self::AtomExponent { .. }
            | Self::AtomExponentAnnotation { .. }
            | Self::Factor(_)
            | Self::FactorAnnotation { .. }
            | Self::FactorExponent { .. }
            | Self::FactorExponentAnnotation { .. }
            | Self::FactorAtom { .. }
            | Self::FactorAtomAnnotation { .. }
            | Self::FactorAtomExponent { .. }
            | Self::FactorAtomExponentAnnotation { .. } => None,
            Self::PrefixAtom(PrefixAtom { prefix, .. })
            | Self::PrefixAtomAnnotation(PrefixAtomAnnotation { prefix, .. })
            | Self::PrefixAtomExponent(PrefixAtomExponent { prefix, .. })
            | Self::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation { prefix, .. })
            | Self::FactorPrefixAtom(FactorPrefixAtom { prefix, .. })
            | Self::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation { prefix, .. })
            | Self::FactorPrefixAtomExponent(FactorPrefixAtomExponent { prefix, .. })
            | Self::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                prefix,
                ..
            }) => Some(*prefix),
        }
    }

    #[must_use]
    pub const fn atom(&self) -> Option<&Atom> {
        match self {
            Self::Annotation(_)
            | Self::Factor(_)
            | Self::FactorAnnotation { .. }
            | Self::FactorExponent { .. }
            | Self::FactorExponentAnnotation { .. } => None,
            Self::Atom(atom)
            | Self::AtomAnnotation(AtomAnnotation { atom, .. })
            | Self::AtomExponent(AtomExponent { atom, .. })
            | Self::AtomExponentAnnotation(AtomExponentAnnotation { atom, .. })
            | Self::PrefixAtom(PrefixAtom { atom, .. })
            | Self::PrefixAtomAnnotation(PrefixAtomAnnotation { atom, .. })
            | Self::PrefixAtomExponent(PrefixAtomExponent { atom, .. })
            | Self::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation { atom, .. })
            | Self::FactorAtom(FactorAtom { atom, .. })
            | Self::FactorAtomAnnotation(FactorAtomAnnotation { atom, .. })
            | Self::FactorAtomExponent(FactorAtomExponent { atom, .. })
            | Self::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation { atom, .. })
            | Self::FactorPrefixAtom(FactorPrefixAtom { atom, .. })
            | Self::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation { atom, .. })
            | Self::FactorPrefixAtomExponent(FactorPrefixAtomExponent { atom, .. })
            | Self::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                atom,
                ..
            }) => Some(atom),
        }
    }

    #[must_use]
    pub const fn exponent(&self) -> Option<Exponent> {
        match self {
            Self::Annotation(_)
            | Self::Atom(_)
            | Self::AtomAnnotation { .. }
            | Self::PrefixAtom { .. }
            | Self::PrefixAtomAnnotation { .. }
            | Self::Factor(_)
            | Self::FactorAnnotation { .. }
            | Self::FactorAtom { .. }
            | Self::FactorAtomAnnotation { .. }
            | Self::FactorPrefixAtom { .. }
            | Self::FactorPrefixAtomAnnotation { .. } => None,
            Self::FactorExponent(FactorExponent { exponent, .. })
            | Self::FactorExponentAnnotation(FactorExponentAnnotation { exponent, .. })
            | Self::AtomExponent(AtomExponent { exponent, .. })
            | Self::AtomExponentAnnotation(AtomExponentAnnotation { exponent, .. })
            | Self::PrefixAtomExponent(PrefixAtomExponent { exponent, .. })
            | Self::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
                exponent, ..
            })
            | Self::FactorAtomExponent(FactorAtomExponent { exponent, .. })
            | Self::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                exponent, ..
            })
            | Self::FactorPrefixAtomExponent(FactorPrefixAtomExponent { exponent, .. })
            | Self::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                exponent,
                ..
            }) => Some(*exponent),
        }
    }

    pub(crate) fn effective_exponent(&self) -> Exponent {
        self.exponent().unwrap_or(1)
    }

    #[must_use]
    pub fn annotation(&self) -> Option<&str> {
        match self {
            Self::Atom(_)
            | Self::AtomExponent { .. }
            | Self::PrefixAtom { .. }
            | Self::PrefixAtomExponent { .. }
            | Self::Factor(_)
            | Self::FactorAtom { .. }
            | Self::FactorExponent { .. }
            | Self::FactorExponentAnnotation { .. }
            | Self::FactorAtomExponent { .. }
            | Self::FactorPrefixAtom { .. }
            | Self::FactorPrefixAtomExponent { .. } => None,
            Self::Annotation(annotation)
            | Self::AtomAnnotation(AtomAnnotation { annotation, .. })
            | Self::AtomExponentAnnotation(AtomExponentAnnotation { annotation, .. })
            | Self::PrefixAtomAnnotation(PrefixAtomAnnotation { annotation, .. })
            | Self::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
                annotation, ..
            })
            | Self::FactorAnnotation(FactorAnnotation { annotation, .. })
            | Self::FactorAtomAnnotation(FactorAtomAnnotation { annotation, .. })
            | Self::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                annotation, ..
            })
            | Self::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation { annotation, .. })
            | Self::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                annotation,
                ..
            }) => Some(annotation.as_str()),
        }
    }

    pub(crate) fn assign_exponent(self, exponent: Exponent) -> Self {
        match self {
            Self::Annotation(annotation) => annotation.assign_exponent(exponent),
            Self::Atom(atom) => atom.assign_exponent(exponent),
            Self::AtomAnnotation(inner) => inner.assign_exponent(exponent),
            Self::AtomExponent(inner) => inner.assign_exponent(exponent),
            Self::AtomExponentAnnotation(inner) => inner.assign_exponent(exponent),
            Self::PrefixAtom(inner) => inner.assign_exponent(exponent),
            Self::PrefixAtomAnnotation(inner) => inner.assign_exponent(exponent),
            Self::PrefixAtomExponent(inner) => inner.assign_exponent(exponent),
            Self::PrefixAtomExponentAnnotation(inner) => inner.assign_exponent(exponent),
            Self::Factor(factor) => factor.assign_exponent(exponent),
            Self::FactorAnnotation(inner) => inner.assign_exponent(exponent),
            Self::FactorExponent(inner) => inner.assign_exponent(exponent),
            Self::FactorExponentAnnotation(inner) => inner.assign_exponent(exponent),
            Self::FactorAtom(inner) => inner.assign_exponent(exponent),
            Self::FactorAtomAnnotation(inner) => inner.assign_exponent(exponent),
            Self::FactorAtomExponent(inner) => inner.assign_exponent(exponent),
            Self::FactorAtomExponentAnnotation(inner) => inner.assign_exponent(exponent),
            Self::FactorPrefixAtomExponent(inner) => inner.assign_exponent(exponent),
            Self::FactorPrefixAtomExponentAnnotation(inner) => inner.assign_exponent(exponent),
            Self::FactorPrefixAtom(inner) => inner.assign_exponent(exponent),
            Self::FactorPrefixAtomAnnotation(inner) => inner.assign_exponent(exponent),
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
    /// NOTE: This does not check the annotation since that does not effect the
    /// quantity of the Term.
    ///
    #[must_use]
    pub const fn is_unity(&self) -> bool {
        matches!(self, Self::Factor(1))
    }

    /// Depending on the `Term`, its string representation could be anywhere from a `&'static str`
    /// to a combination of all of its fields as a `String`. For those former cases, we want to
    /// allow borrowing the `&'static str` to save on allocations.
    ///
    #[must_use]
    pub fn as_cow_str(&self) -> Cow<'_, str> {
        use crate::UcumSymbol;

        if self.is_unity() && self.annotation().is_none() {
            return Cow::Borrowed("1");
        };

        match self {
            Self::Annotation(annotation) => Cow::Owned(annotation.to_string()),
            Self::Atom(atom) => Cow::Borrowed(atom.primary_code()),
            Self::AtomAnnotation(inner) => Cow::Owned(inner.to_string()),
            Self::AtomExponent(inner) => Cow::Owned(inner.to_string()),
            Self::AtomExponentAnnotation(inner) => Cow::Owned(inner.to_string()),
            Self::PrefixAtom(inner) => Cow::Owned(inner.to_string()),
            Self::PrefixAtomAnnotation(inner) => Cow::Owned(inner.to_string()),
            Self::PrefixAtomExponent(inner) => Cow::Owned(inner.to_string()),
            Self::PrefixAtomExponentAnnotation(inner) => Cow::Owned(inner.to_string()),
            Self::Factor(factor) => Cow::Owned(factor.to_string()),
            Self::FactorAnnotation(inner) => Cow::Owned(inner.to_string()),
            Self::FactorExponent(inner) => Cow::Owned(inner.to_string()),
            Self::FactorExponentAnnotation(inner) => Cow::Owned(inner.to_string()),
            Self::FactorAtom(inner) => Cow::Owned(inner.to_string()),
            Self::FactorAtomAnnotation(inner) => Cow::Owned(inner.to_string()),
            Self::FactorAtomExponent(inner) => Cow::Owned(inner.to_string()),
            Self::FactorAtomExponentAnnotation(inner) => Cow::Owned(inner.to_string()),
            Self::FactorPrefixAtom(inner) => Cow::Owned(inner.to_string()),
            Self::FactorPrefixAtomAnnotation(inner) => Cow::Owned(inner.to_string()),
            Self::FactorPrefixAtomExponent(inner) => Cow::Owned(inner.to_string()),
            Self::FactorPrefixAtomExponentAnnotation(inner) => Cow::Owned(inner.to_string()),
        }
    }
}

impl Default for Term {
    fn default() -> Self {
        UNITY
    }
}

#[derive(Default)]
pub struct Builder {
    factor: Option<Factor>,
    prefix: Option<Prefix>,
    atom: Option<Atom>,
    exponent: Option<Exponent>,
    annotation: Option<String>,
}

impl Builder {
    pub fn factor(self, factor: Factor) -> Self {
        Self {
            factor: Some(factor),
            ..self
        }
    }
    pub fn prefix(self, prefix: Prefix) -> Self {
        Self {
            prefix: Some(prefix),
            ..self
        }
    }
    pub fn atom(self, atom: Atom) -> Self {
        Self {
            atom: Some(atom),
            ..self
        }
    }
    pub fn exponent(self, exponent: Exponent) -> Self {
        Self {
            exponent: Some(exponent),
            ..self
        }
    }
    pub fn annotation<T>(self, annotation: T) -> Self
    where
        T: ToString,
    {
        Self {
            annotation: Some(annotation.to_string()),
            ..self
        }
    }

    pub fn build(self) -> Term {
        match (
            self.factor,
            self.prefix,
            self.atom,
            self.exponent,
            self.annotation,
        ) {
            (None, None, None, None, None)
            | (None, None, None, Some(_), None)
            | (None, None, None, Some(_), Some(_))
            | (None, Some(_), None, None, None)
            | (None, Some(_), None, None, Some(_))
            | (None, Some(_), None, Some(_), None)
            | (None, Some(_), None, Some(_), Some(_))
            | (Some(_), Some(_), None, None, None)
            | (Some(_), Some(_), None, None, Some(_))
            | (Some(_), Some(_), None, Some(_), None)
            | (Some(_), Some(_), None, Some(_), Some(_)) => panic!(),
            (None, None, None, None, Some(annotation)) => {
                Term::Annotation(Annotation::new(annotation))
            }
            (None, None, Some(atom), None, None) => Term::Atom(atom),
            (None, None, Some(atom), None, Some(annotation)) => {
                Term::AtomAnnotation(AtomAnnotation {
                    atom,
                    annotation: Annotation::new(annotation),
                })
            }
            (None, None, Some(atom), Some(exponent), None) => {
                Term::AtomExponent(AtomExponent { atom, exponent })
            }
            (None, None, Some(atom), Some(exponent), Some(annotation)) => {
                Term::AtomExponentAnnotation(AtomExponentAnnotation {
                    atom,
                    exponent,
                    annotation: Annotation::new(annotation),
                })
            }
            (None, Some(prefix), Some(atom), None, None) => {
                Term::PrefixAtom(PrefixAtom { prefix, atom })
            }
            (None, Some(prefix), Some(atom), None, Some(annotation)) => {
                Term::PrefixAtomAnnotation(PrefixAtomAnnotation {
                    prefix,
                    atom,
                    annotation: Annotation::new(annotation),
                })
            }
            (None, Some(prefix), Some(atom), Some(exponent), None) => {
                Term::PrefixAtomExponent(PrefixAtomExponent {
                    prefix,
                    atom,
                    exponent,
                })
            }
            (None, Some(prefix), Some(atom), Some(exponent), Some(annotation)) => {
                Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
                    prefix,
                    atom,
                    exponent,
                    annotation: Annotation::new(annotation),
                })
            }
            (Some(factor), None, None, None, None) => Term::Factor(factor),
            (Some(factor), None, None, None, Some(annotation)) => {
                Term::FactorAnnotation(FactorAnnotation {
                    factor,
                    annotation: Annotation::new(annotation),
                })
            }
            (Some(factor), None, None, Some(exponent), None) => {
                Term::FactorExponent(FactorExponent { factor, exponent })
            }
            (Some(factor), None, None, Some(exponent), Some(annotation)) => {
                Term::FactorExponentAnnotation(FactorExponentAnnotation {
                    factor,
                    exponent,
                    annotation: Annotation::new(annotation),
                })
            }
            (Some(factor), None, Some(atom), None, None) => {
                Term::FactorAtom(FactorAtom { factor, atom })
            }
            (Some(factor), None, Some(atom), None, Some(annotation)) => {
                Term::FactorAtomAnnotation(FactorAtomAnnotation {
                    factor,
                    atom,
                    annotation: Annotation::new(annotation),
                })
            }
            (Some(factor), None, Some(atom), Some(exponent), None) => {
                Term::FactorAtomExponent(FactorAtomExponent {
                    factor,
                    atom,
                    exponent,
                })
            }
            (Some(factor), None, Some(atom), Some(exponent), Some(annotation)) => {
                Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                    factor,
                    atom,
                    exponent,
                    annotation: Annotation::new(annotation),
                })
            }
            (Some(factor), Some(prefix), Some(atom), None, None) => {
                Term::FactorPrefixAtom(FactorPrefixAtom {
                    factor,
                    prefix,
                    atom,
                })
            }
            (Some(factor), Some(prefix), Some(atom), None, Some(annotation)) => {
                Term::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation {
                    factor,
                    prefix,
                    atom,
                    annotation: Annotation::new(annotation),
                })
            }
            (Some(factor), Some(prefix), Some(atom), Some(exponent), None) => {
                Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent {
                    factor,
                    prefix,
                    atom,
                    exponent,
                })
            }
            (Some(factor), Some(prefix), Some(atom), Some(exponent), Some(annotation)) => {
                Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                    factor,
                    prefix,
                    atom,
                    exponent,
                    annotation: Annotation::new(annotation),
                })
            }
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
