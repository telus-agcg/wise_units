mod annotation_composable;
mod builder;
mod composable;
mod display;
mod field_eq;
mod invert;
mod is_compatible_with;
pub(crate) mod num_traits;
mod partial_eq;
mod partial_ord;
mod reducible;
pub(crate) mod term_reduce;
mod ucum_unit;
#[cfg(feature = "v2")]
mod v2;
pub mod variants;

use std::borrow::Cow;

pub use builder::Builder;

use crate::{Annotation, Atom, Prefix};

use self::variants::{
    AssignFactor, AtomAnnotation, AtomExponent, AtomExponentAnnotation, FactorAnnotation,
    FactorAtom, FactorAtomAnnotation, FactorAtomExponent, FactorAtomExponentAnnotation,
    FactorExponent, FactorExponentAnnotation, FactorPrefixAtom, FactorPrefixAtomAnnotation,
    FactorPrefixAtomExponent, FactorPrefixAtomExponentAnnotation, PowOutput, PrefixAtom,
    PrefixAtomAnnotation, PrefixAtomExponent, PrefixAtomExponentAnnotation, SetAnnotation,
    SetExponent,
};

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
    /// While this constructor allows for some basic cases, it's a remnant of old design; you
    /// should consider using `term::Builder` instead.
    ///
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

    /// Returns the value of the `Term`'s factor, if it has one.
    ///
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
                *self = Self::FactorAnnotation(annotation.assign_factor(new_factor));
            }
            Self::Atom(atom) => {
                *self = Self::FactorAtom(atom.assign_factor(new_factor));
            }
            Self::AtomAnnotation(inner) => {
                *self = Self::FactorAtomAnnotation(inner.assign_factor(new_factor));
            }
            Self::AtomExponent(inner) => {
                *self = Self::FactorAtomExponent(inner.assign_factor(new_factor));
            }
            Self::AtomExponentAnnotation(inner) => {
                *self = Self::FactorAtomExponentAnnotation(inner.assign_factor(new_factor));
            }
            Self::PrefixAtom(inner) => {
                *self = Self::FactorPrefixAtom(inner.assign_factor(new_factor));
            }
            Self::PrefixAtomAnnotation(inner) => {
                *self = Self::FactorPrefixAtomAnnotation(inner.assign_factor(new_factor));
            }
            Self::PrefixAtomExponent(inner) => {
                *self = Self::FactorPrefixAtomExponent(inner.assign_factor(new_factor));
            }
            Self::PrefixAtomExponentAnnotation(inner) => {
                *self = Self::FactorPrefixAtomExponentAnnotation(inner.assign_factor(new_factor));
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

    /// Returns the value of the `Term`'s `Prefix`, if it has one.
    ///
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

    /// Returns the value of the `Term`'s `Atom`, if it has one.
    ///
    #[must_use]
    pub const fn atom(&self) -> Option<Atom> {
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
            }) => Some(*atom),
        }
    }

    /// Returns the value of the `Term`'s `Exponent`, if it has one.
    ///
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

    #[allow(clippy::too_many_lines)]
    pub(crate) fn set_exponent(&mut self, exponent: Exponent) -> &mut Self {
        match self {
            Self::Annotation(annotation) => match annotation.set_exponent(exponent) {
                PowOutput::Zero(()) | PowOutput::One(()) => (),
                PowOutput::Rest(fea) => *self = Self::FactorExponentAnnotation(fea),
            },
            Self::Atom(atom) => match atom.set_exponent(exponent) {
                PowOutput::Zero(factor) => *self = Self::Factor(factor),
                PowOutput::One(()) => (),
                PowOutput::Rest(ae) => *self = Self::AtomExponent(ae),
            },
            Self::AtomAnnotation(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(annotation) => *self = Self::Annotation(annotation),
                PowOutput::One(()) => (),
                PowOutput::Rest(aea) => *self = Self::AtomExponentAnnotation(aea),
            },
            Self::AtomExponent(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(factor) => *self = Self::Factor(factor),
                PowOutput::One(atom) => *self = Self::Atom(atom),
                PowOutput::Rest(()) => (),
            },
            Self::AtomExponentAnnotation(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(annotation) => *self = Self::Annotation(annotation),
                PowOutput::One(atom_annotation) => *self = Self::AtomAnnotation(atom_annotation),
                PowOutput::Rest(()) => (),
            },
            Self::PrefixAtom(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(factor) => *self = Self::Factor(factor),
                PowOutput::One(()) => (),
                PowOutput::Rest(pae) => *self = Self::PrefixAtomExponent(pae),
            },
            Self::PrefixAtomAnnotation(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(annotation) => *self = Self::Annotation(annotation),
                PowOutput::One(()) => (),
                PowOutput::Rest(paea) => *self = Self::PrefixAtomExponentAnnotation(paea),
            },
            Self::PrefixAtomExponent(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(factor) => *self = Self::Factor(factor),
                PowOutput::One(prefix_atom) => *self = Self::PrefixAtom(prefix_atom),
                PowOutput::Rest(()) => (),
            },
            Self::PrefixAtomExponentAnnotation(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(annotation) => *self = Self::Annotation(annotation),
                PowOutput::One(paa) => *self = Self::PrefixAtomAnnotation(paa),
                PowOutput::Rest(()) => (),
            },
            Self::Factor(factor) => match factor.set_exponent(exponent) {
                PowOutput::Zero(()) | PowOutput::One(()) => (),
                PowOutput::Rest(fe) => *self = Self::FactorExponent(fe),
            },
            Self::FactorAnnotation(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(annotation) => *self = Self::Annotation(annotation),
                PowOutput::One(()) => (),
                PowOutput::Rest(fea) => *self = Self::FactorExponentAnnotation(fea),
            },
            Self::FactorExponent(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(factor) | PowOutput::One(factor) => *self = Self::Factor(factor),
                PowOutput::Rest(()) => (),
            },
            Self::FactorExponentAnnotation(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(annotation) => *self = Self::Annotation(annotation),
                PowOutput::One(factor_annotation) => *self = factor_annotation.into(),
                PowOutput::Rest(()) => (),
            },
            Self::FactorAtom(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(factor) => *self = Self::Factor(factor),
                PowOutput::One(()) => (),
                PowOutput::Rest(fae) => *self = Self::FactorAtomExponent(fae),
            },
            Self::FactorAtomAnnotation(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(annotation) => *self = Self::Annotation(annotation),
                PowOutput::One(()) => (),
                PowOutput::Rest(faea) => *self = Self::FactorAtomExponentAnnotation(faea),
            },
            Self::FactorAtomExponent(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(factor) => *self = Self::Factor(factor),
                PowOutput::One(factor_atom) => *self = Self::FactorAtom(factor_atom),
                PowOutput::Rest(()) => (),
            },
            Self::FactorAtomExponentAnnotation(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(annotation) => *self = Self::Annotation(annotation),
                PowOutput::One(faa) => *self = Self::FactorAtomAnnotation(faa),
                PowOutput::Rest(()) => (),
            },
            Self::FactorPrefixAtomExponent(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(factor) => *self = Self::Factor(factor),
                PowOutput::One(fpa) => *self = Self::FactorPrefixAtom(fpa),
                PowOutput::Rest(()) => (),
            },
            Self::FactorPrefixAtomExponentAnnotation(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(annotation) => *self = Self::Annotation(annotation),
                PowOutput::One(fpaa) => *self = Self::FactorPrefixAtomAnnotation(fpaa),
                PowOutput::Rest(()) => (),
            },
            Self::FactorPrefixAtom(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(factor) => *self = Self::Factor(factor),
                PowOutput::One(()) => (),
                PowOutput::Rest(fpae) => *self = fpae.into(),
            },
            Self::FactorPrefixAtomAnnotation(inner) => match inner.set_exponent(exponent) {
                PowOutput::Zero(annotation) => *self = Self::Annotation(annotation),
                PowOutput::One(()) => (),
                PowOutput::Rest(fpaea) => *self = fpaea.into(),
            },
        }

        self
    }

    /// Returns the value of the `Term`'s `Annotation`, if it has one.
    ///
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
            | Self::FactorExponentAnnotation(FactorExponentAnnotation { annotation, .. })
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

    pub(crate) fn set_annotation<T>(&mut self, new_annotation: T) -> &mut Self
    where
        Annotation: From<T>,
    {
        match self {
            Self::Annotation(annotation) => {
                *annotation = Annotation::from(new_annotation);
            }
            Self::Atom(atom) => {
                *self = AtomAnnotation::new(*atom, Annotation::from(new_annotation)).into();
            }
            Self::AtomAnnotation(inner) => {
                inner.set_annotation(new_annotation);
            }
            Self::AtomExponent(inner) => {
                *self = inner.set_annotation(new_annotation).into();
            }
            Self::AtomExponentAnnotation(inner) => inner.set_annotation(new_annotation),
            Self::PrefixAtom(inner) => {
                *self = inner.set_annotation(new_annotation).into();
            }
            Self::PrefixAtomAnnotation(inner) => inner.set_annotation(new_annotation),
            Self::PrefixAtomExponent(inner) => {
                *self = inner.set_annotation(new_annotation).into();
            }
            Self::PrefixAtomExponentAnnotation(inner) => inner.set_annotation(new_annotation),
            Self::FactorAnnotation(inner) => inner.set_annotation(new_annotation),
            Self::FactorAtomExponentAnnotation(inner) => inner.set_annotation(new_annotation),
            Self::FactorAtomAnnotation(inner) => inner.set_annotation(new_annotation),
            Self::FactorExponentAnnotation(inner) => inner.set_annotation(new_annotation),
            Self::FactorPrefixAtomAnnotation(inner) => inner.set_annotation(new_annotation),
            Self::Factor(factor) => {
                *self = FactorAnnotation::new(*factor, Annotation::from(new_annotation)).into();
            }
            Self::FactorExponent(inner) => {
                *self = inner.set_annotation(new_annotation).into();
            }
            Self::FactorAtom(inner) => {
                *self = inner.set_annotation(new_annotation).into();
            }
            Self::FactorAtomExponent(inner) => {
                *self = inner.set_annotation(new_annotation).into();
            }
            Self::FactorPrefixAtom(inner) => {
                *self = inner.set_annotation(new_annotation).into();
            }
            Self::FactorPrefixAtomExponent(inner) => {
                *self = inner.set_annotation(new_annotation).into();
            }
            Self::FactorPrefixAtomExponentAnnotation(inner) => inner.set_annotation(new_annotation),
        }

        self
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

#[cfg(test)]
mod tests {
    use crate::FieldEq;

    use super::*;

    fn subject_annotation() -> Annotation {
        Annotation::from("meow")
    }

    #[test]
    fn validate_new_unity() {
        assert_eq!(UNITY.to_string(), "1");
    }

    mod factor {
        use super::*;

        macro_rules! assert_set_factor_no_factor {
            ($subject:ident => $expected:expr) => {
                assert!($subject.factor().is_none());

                let _ = $subject.set_factor(42);
                assert_eq!($subject.factor().unwrap(), 42);
                assert!($subject.field_eq(&$expected));
            };
        }

        macro_rules! assert_set_factor_has_factor {
            ($subject:ident) => {
                assert_eq!($subject.factor().unwrap(), 42);
                let _ = $subject.set_factor(1);
                assert_eq!($subject.factor().unwrap(), 1);
            };
        }

        #[test]
        fn annotation_test() {
            let mut subject = Term::Annotation(subject_annotation());
            assert_set_factor_no_factor!(subject => Term::FactorAnnotation(FactorAnnotation::new(
                42,
                subject_annotation()
            )));
        }

        #[test]
        fn atom_test() {
            let mut subject = Term::Atom(Atom::Meter);
            assert_set_factor_no_factor!(subject => Term::FactorAtom(FactorAtom::new(42, Atom::Meter)));
        }

        #[test]
        fn atom_annotation_test() {
            let mut subject =
                Term::AtomAnnotation(AtomAnnotation::new(Atom::Meter, subject_annotation()));

            assert_set_factor_no_factor!(subject => Term::FactorAtomAnnotation(FactorAtomAnnotation::new(
                            42,
                            Atom::Meter,
                            subject_annotation()
                        ))
            );
        }

        #[test]
        fn atom_exponent_test() {
            let mut subject = Term::AtomExponent(AtomExponent::new(Atom::Meter, 43));

            assert_set_factor_no_factor!(
                subject => Term::FactorAtomExponent(FactorAtomExponent::new(42, Atom::Meter, 43))
            );
        }

        #[test]
        fn atom_exponent_annotation_test() {
            let mut subject = Term::AtomExponentAnnotation(AtomExponentAnnotation::new(
                Atom::Meter,
                43,
                subject_annotation(),
            ));

            assert_set_factor_no_factor!(
                subject => Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation::new(
                    42,
                    Atom::Meter,
                    43,
                    subject_annotation()))
            );
        }

        #[test]
        fn prefix_atom_test() {
            let mut subject = Term::PrefixAtom(PrefixAtom::new(Prefix::Kilo, Atom::Meter));

            assert_set_factor_no_factor!(
                subject => Term::FactorPrefixAtom(FactorPrefixAtom::new(42, Prefix::Kilo, Atom::Meter,))
            );
        }

        #[test]
        fn prefix_atom_annotation_test() {
            let mut subject = Term::PrefixAtomAnnotation(PrefixAtomAnnotation::new(
                Prefix::Kilo,
                Atom::Meter,
                subject_annotation(),
            ));
            assert_set_factor_no_factor!(
                subject => Term::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation::new(
                    42,
                    Prefix::Kilo,
                    Atom::Meter,
                    subject_annotation()
                ))
            );
        }

        #[test]
        fn prefix_atom_exponent_test() {
            let mut subject =
                Term::PrefixAtomExponent(PrefixAtomExponent::new(Prefix::Kilo, Atom::Meter, 43));

            assert_set_factor_no_factor!(
                subject => Term::FactorPrefixAtomExponent(
                FactorPrefixAtomExponent::new(42, Prefix::Kilo, Atom::Meter, 43,)
            ));
        }

        #[test]
        fn prefix_atom_exponent_annotation_test() {
            let mut subject =
                Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation::new(
                    Prefix::Kilo,
                    Atom::Meter,
                    43,
                    subject_annotation(),
                ));
            assert_set_factor_no_factor!(
                subject => Term::FactorPrefixAtomExponentAnnotation(
                FactorPrefixAtomExponentAnnotation::new(
                    42,
                    Prefix::Kilo,
                    Atom::Meter,
                    43,
                    subject_annotation()
                )
            ));
        }

        #[test]
        fn factor_test() {
            let mut subject = Term::Factor(42);
            assert_set_factor_has_factor!(subject);
        }

        #[test]
        fn factor_annotation_test() {
            let mut subject =
                Term::FactorAnnotation(FactorAnnotation::new(42, subject_annotation()));
            assert_set_factor_has_factor!(subject);
        }

        #[test]
        fn factor_exponent_test() {
            let mut subject = Term::FactorExponent(FactorExponent::new(42, 43));
            assert_set_factor_has_factor!(subject);
        }

        #[test]
        fn factor_exponent_annotation_test() {
            let mut subject = Term::FactorExponentAnnotation(FactorExponentAnnotation::new(
                42,
                43,
                subject_annotation(),
            ));
            assert_set_factor_has_factor!(subject);
        }

        #[test]
        fn factor_atom_test() {
            let mut subject = Term::FactorAtom(FactorAtom::new(42, Atom::Meter));
            assert_set_factor_has_factor!(subject);
        }

        #[test]
        fn factor_atom_annotation_test() {
            let mut subject = Term::FactorAtomAnnotation(FactorAtomAnnotation::new(
                42,
                Atom::Meter,
                subject_annotation(),
            ));
            assert_set_factor_has_factor!(subject);
        }

        #[test]
        fn factor_atom_exponent_test() {
            let mut subject =
                Term::FactorAtomExponent(FactorAtomExponent::new(42, Atom::Meter, 43));
            assert_set_factor_has_factor!(subject);
        }

        #[test]
        fn factor_atom_exponent_annotation_test() {
            let mut subject = Term::FactorAtomExponentAnnotation(
                FactorAtomExponentAnnotation::new(42, Atom::Meter, 43, subject_annotation()),
            );
            assert_set_factor_has_factor!(subject);
        }

        #[test]
        fn factor_prefix_atom_test() {
            let mut subject =
                Term::FactorPrefixAtom(FactorPrefixAtom::new(42, Prefix::Kilo, Atom::Meter));
            assert_set_factor_has_factor!(subject);
        }

        #[test]
        fn factor_prefix_atom_annotation_test() {
            let mut subject = Term::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation::new(
                42,
                Prefix::Kilo,
                Atom::Meter,
                subject_annotation(),
            ));
            assert_set_factor_has_factor!(subject);
        }

        #[test]
        fn factor_prefix_atom_exponent_test() {
            let mut subject = Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent::new(
                42,
                Prefix::Kilo,
                Atom::Meter,
                43,
            ));
            assert_set_factor_has_factor!(subject);
        }

        #[test]
        fn factor_prefix_atom_exponent_annotation_test() {
            let mut subject =
                Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation::new(
                    42,
                    Prefix::Kilo,
                    Atom::Meter,
                    43,
                    subject_annotation(),
                ));
            assert_set_factor_has_factor!(subject);
        }
    }

    mod set_factor {
        use super::*;

        macro_rules! test_non_factor_variant {
            ($subject_variant:ident, $expected_variant:ident, $($params:tt)+) => {
                let mut subject = Term::$subject_variant($subject_variant::new($($params)+));
                let _ = subject.set_factor(1);
                assert_eq!(
                    subject,
                    Term::$expected_variant($expected_variant::new(1, $($params)+))
                );
            };
        }

        macro_rules! test_factor_variant {
            ($subject_variant:ident, $($params:tt)+) => {
                let mut subject = Term::$subject_variant($subject_variant::new(2, $($params)+));
                let _ = subject.set_factor(42);
                assert_eq!(
                    subject,
                    Term::$subject_variant($subject_variant::new(42, $($params)+))
                );
            };
        }

        #[test]
        fn annotation_test() {
            let mut subject = Term::Annotation(Annotation::from("foo"));
            let _ = subject.set_factor(1);
            assert_eq!(
                subject,
                Term::FactorAnnotation(FactorAnnotation::new(1, "foo".into()))
            );
        }

        #[test]
        fn atom_test() {
            let mut subject = Term::Atom(Atom::Meter);
            let _ = subject.set_factor(1);
            assert_eq!(subject, Term::FactorAtom(FactorAtom::new(1, Atom::Meter)));
        }

        #[test]
        fn atom_annotation_test() {
            test_non_factor_variant!(
                AtomAnnotation,
                FactorAtomAnnotation,
                Atom::Meter,
                "foo".into()
            );
        }

        #[test]
        fn atom_exponent_test() {
            test_non_factor_variant!(AtomExponent, FactorAtomExponent, Atom::Meter, 2);
        }

        #[test]
        fn atom_exponent_annotation_test() {
            test_non_factor_variant!(
                AtomExponentAnnotation,
                FactorAtomExponentAnnotation,
                Atom::Meter,
                2,
                "foo".into()
            );
        }

        #[test]
        fn prefix_atom_test() {
            test_non_factor_variant!(PrefixAtom, FactorPrefixAtom, Prefix::Yocto, Atom::Meter);
        }

        #[test]
        fn prefix_atom_annotation_test() {
            test_non_factor_variant!(
                PrefixAtomAnnotation,
                FactorPrefixAtomAnnotation,
                Prefix::Yocto,
                Atom::Meter,
                "foo".into()
            );
        }

        #[test]
        fn prefix_atom_exponent_test() {
            test_non_factor_variant!(
                PrefixAtomExponent,
                FactorPrefixAtomExponent,
                Prefix::Yocto,
                Atom::Meter,
                2
            );
        }

        #[test]
        fn prefix_atom_exponent_annotation_test() {
            test_non_factor_variant!(
                PrefixAtomExponentAnnotation,
                FactorPrefixAtomExponentAnnotation,
                Prefix::Yocto,
                Atom::Meter,
                2,
                "foo".into()
            );
        }

        #[test]
        fn factor_test() {
            let mut subject = Term::Factor(42);
            let _ = subject.set_factor(1);
            assert_eq!(subject, Term::Factor(1));
        }

        #[test]
        fn factor_annotation_test() {
            test_factor_variant!(FactorAnnotation, "foo".into());
        }

        #[test]
        fn factor_exponent_test() {
            test_factor_variant!(FactorExponent, 2);
        }

        #[test]
        fn factor_exponent_annotation_test() {
            test_factor_variant!(FactorExponentAnnotation, 2, "foo".into());
        }

        #[test]
        fn factor_atom_test() {
            test_factor_variant!(FactorAtom, Atom::Meter);
        }

        #[test]
        fn factor_atom_annotation_test() {
            test_factor_variant!(FactorAtomAnnotation, Atom::Meter, "foo".into());
        }

        #[test]
        fn factor_atom_exponent_test() {
            test_factor_variant!(FactorAtomExponent, Atom::Meter, 2);
        }

        #[test]
        fn factor_atom_exponent_annotation_test() {
            test_factor_variant!(FactorAtomExponentAnnotation, Atom::Meter, 2, "foo".into());
        }

        #[test]
        fn factor_prefix_atom_test() {
            test_factor_variant!(FactorPrefixAtom, Prefix::Yocto, Atom::Meter);
        }

        #[test]
        fn factor_prefix_atom_annotation_test() {
            test_factor_variant!(
                FactorPrefixAtomAnnotation,
                Prefix::Yocto,
                Atom::Meter,
                "foo".into()
            );
        }

        #[test]
        fn factor_prefix_atom_exponent_test() {
            test_factor_variant!(FactorPrefixAtomExponent, Prefix::Yocto, Atom::Meter, 2);
        }

        #[test]
        fn factor_prefix_atom_exponent_annotation_test() {
            test_factor_variant!(
                FactorPrefixAtomExponentAnnotation,
                Prefix::Yocto,
                Atom::Meter,
                2,
                "foo".into()
            );
        }
    }

    mod exponent {
        use super::*;

        macro_rules! assert_set_exponent_no_exponent {
            ($subject:ident => $expected:expr) => {
                assert!($subject.exponent().is_none());

                let _ = $subject.set_exponent(3);
                assert_eq!($subject.exponent().unwrap(), 3);
                assert!(
                    $subject.field_eq(&$expected),
                    "{:?}\n{:?}",
                    &$subject,
                    $expected
                );
            };
        }

        macro_rules! assert_set_exponent_has_exponent {
            ($subject:ident) => {
                assert_eq!($subject.exponent().unwrap(), 2);
                let _ = $subject.set_exponent(3);
                assert_eq!($subject.exponent().unwrap(), 3);
            };
        }

        #[test]
        fn annotation_test() {
            let mut subject = Term::Annotation(subject_annotation());
            assert_set_exponent_no_exponent!(subject => Term::FactorExponentAnnotation(
                    FactorExponentAnnotation::new(1, 3, subject_annotation())
            ));
        }

        #[test]
        fn atom_test() {
            let mut subject = Term::Atom(Atom::Meter);
            assert_set_exponent_no_exponent!(subject => Term::AtomExponent(AtomExponent::new(Atom::Meter, 3)));
        }

        #[test]
        fn atom_annotation_test() {
            let mut subject =
                Term::AtomAnnotation(AtomAnnotation::new(Atom::Meter, subject_annotation()));

            assert_set_exponent_no_exponent!(
                subject => Term::AtomExponentAnnotation(AtomExponentAnnotation::new(
                        Atom::Meter,
                        3,
                        subject_annotation()))
            );
        }

        #[test]
        fn atom_exponent_test() {
            let mut subject = Term::AtomExponent(AtomExponent::new(Atom::Meter, 2));
            assert_set_exponent_has_exponent!(subject);
        }

        #[test]
        fn atom_exponent_annotation_test() {
            let mut subject = Term::AtomExponentAnnotation(AtomExponentAnnotation::new(
                Atom::Meter,
                2,
                subject_annotation(),
            ));

            assert_set_exponent_has_exponent!(subject);
        }

        #[test]
        fn prefix_atom_test() {
            let mut subject = Term::PrefixAtom(PrefixAtom::new(Prefix::Kilo, Atom::Meter));

            assert_set_exponent_no_exponent!(
                subject => Term::PrefixAtomExponent(PrefixAtomExponent::new(Prefix::Kilo, Atom::Meter, 3))
            );
        }

        #[test]
        fn prefix_atom_annotation_test() {
            let mut subject = Term::PrefixAtomAnnotation(PrefixAtomAnnotation::new(
                Prefix::Kilo,
                Atom::Meter,
                subject_annotation(),
            ));
            assert_set_exponent_no_exponent!(
                subject => Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation::new(
                    Prefix::Kilo,
                    Atom::Meter,
                    3,
                    subject_annotation()
                ))
            );
        }

        #[test]
        fn prefix_atom_exponent_test() {
            let mut subject =
                Term::PrefixAtomExponent(PrefixAtomExponent::new(Prefix::Kilo, Atom::Meter, 2));

            assert_set_exponent_has_exponent!(subject);
        }

        #[test]
        fn prefix_atom_exponent_annotation_test() {
            let mut subject =
                Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation::new(
                    Prefix::Kilo,
                    Atom::Meter,
                    2,
                    subject_annotation(),
                ));
            assert_set_exponent_has_exponent!(subject);
        }

        #[test]
        fn factor_test() {
            let mut subject = Term::Factor(42);
            assert_set_exponent_no_exponent!(subject => Term::FactorExponent(FactorExponent::new(42, 3)));
        }

        #[test]
        fn factor_annotation_test() {
            let mut subject =
                Term::FactorAnnotation(FactorAnnotation::new(42, subject_annotation()));
            assert_set_exponent_no_exponent!(
                subject => Term::FactorExponentAnnotation(FactorExponentAnnotation::new(42, 3, subject_annotation()))
            );
        }

        #[test]
        fn factor_exponent_test() {
            let mut subject = Term::FactorExponent(FactorExponent::new(42, 2));
            assert_set_exponent_has_exponent!(subject);
        }

        #[test]
        fn factor_exponent_annotation_test() {
            let mut subject = Term::FactorExponentAnnotation(FactorExponentAnnotation::new(
                42,
                2,
                subject_annotation(),
            ));
            assert_set_exponent_has_exponent!(subject);
        }

        #[test]
        fn factor_atom_test() {
            let mut subject = Term::FactorAtom(FactorAtom::new(42, Atom::Meter));

            assert_set_exponent_no_exponent!(
                subject => Term::FactorAtomExponent(FactorAtomExponent::new(42, Atom::Meter, 3))
            );
        }

        #[test]
        fn factor_atom_annotation_test() {
            let mut subject = Term::FactorAtomAnnotation(FactorAtomAnnotation::new(
                42,
                Atom::Meter,
                subject_annotation(),
            ));
            assert_set_exponent_no_exponent!(
                subject => Term::FactorAtomExponentAnnotation(
                    FactorAtomExponentAnnotation::new(42, Atom::Meter, 3, subject_annotation())
                )
            );
        }

        #[test]
        fn factor_atom_exponent_test() {
            let mut subject = Term::FactorAtomExponent(FactorAtomExponent::new(42, Atom::Meter, 2));
            assert_set_exponent_has_exponent!(subject);
        }

        #[test]
        fn factor_atom_exponent_annotation_test() {
            let mut subject = Term::FactorAtomExponentAnnotation(
                FactorAtomExponentAnnotation::new(42, Atom::Meter, 2, subject_annotation()),
            );
            assert_set_exponent_has_exponent!(subject);
        }

        #[test]
        fn factor_prefix_atom_test() {
            let mut subject =
                Term::FactorPrefixAtom(FactorPrefixAtom::new(42, Prefix::Kilo, Atom::Meter));
            assert_set_exponent_no_exponent!(
                subject => Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent::new(42, Prefix::Kilo, Atom::Meter, 3))
            );
        }

        #[test]
        fn factor_prefix_atom_annotation_test() {
            let mut subject = Term::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation::new(
                42,
                Prefix::Kilo,
                Atom::Meter,
                subject_annotation(),
            ));
            assert_set_exponent_no_exponent!(
                subject =>  Term::FactorPrefixAtomExponentAnnotation(
                    FactorPrefixAtomExponentAnnotation::new(42, Prefix::Kilo, Atom::Meter, 3, subject_annotation())
                )
            );
        }

        #[test]
        fn factor_prefix_atom_exponent_test() {
            let mut subject = Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent::new(
                42,
                Prefix::Kilo,
                Atom::Meter,
                2,
            ));
            assert_set_exponent_has_exponent!(subject);
        }

        #[test]
        fn factor_prefix_atom_exponent_annotation_test() {
            let mut subject =
                Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation::new(
                    42,
                    Prefix::Kilo,
                    Atom::Meter,
                    2,
                    subject_annotation(),
                ));
            assert_set_exponent_has_exponent!(subject);
        }
    }

    mod set_exponent {
        use super::*;

        #[test]
        fn annotation_test() {
            let mut subject = Term::Annotation(Annotation::from("foo"));

            // 0
            {
                let mut subject = subject.clone();
                let _ = subject.set_exponent(0);
                assert_eq!(subject, Term::Annotation("foo".into()));
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(it, subject);
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::FactorExponentAnnotation(FactorExponentAnnotation {
                        factor: 1,
                        annotation: "foo".into(),
                        exponent: 4
                    })
                );
            }
        }

        #[test]
        fn atom_test() {
            let mut subject = Term::Atom(Atom::Meter);

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, UNITY);
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(it, subject);
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::AtomExponent(AtomExponent {
                        atom: Atom::Meter,
                        exponent: 4
                    })
                );
            }
        }

        #[test]
        fn atom_annotation_test() {
            let mut subject = Term::AtomAnnotation(AtomAnnotation::new(Atom::Meter, "foo".into()));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, Term::Annotation("foo".into()));
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(it, subject);
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::AtomExponentAnnotation(AtomExponentAnnotation::new(
                        Atom::Meter,
                        4,
                        "foo".into(),
                    ))
                );
            }
        }

        #[test]
        fn atom_exponent_test() {
            let mut subject = Term::AtomExponent(AtomExponent::new(Atom::Meter, 2));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, UNITY);
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(it, Term::Atom(Atom::Meter));
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::AtomExponent(AtomExponent::new(Atom::Meter, 4,))
                );
            }
        }

        #[test]
        fn atom_exponent_annotation_test() {
            let mut subject = Term::AtomExponentAnnotation(AtomExponentAnnotation::new(
                Atom::Meter,
                2,
                "foo".into(),
            ));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, Term::Annotation("foo".into()));
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(
                    it,
                    Term::AtomAnnotation(AtomAnnotation::new(Atom::Meter, "foo".into()))
                );
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::AtomExponentAnnotation(AtomExponentAnnotation::new(
                        Atom::Meter,
                        4,
                        "foo".into()
                    ))
                );
            }
        }

        #[test]
        fn prefix_atom_test() {
            let mut subject = Term::PrefixAtom(PrefixAtom::new(Prefix::Yocto, Atom::Meter));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, UNITY);
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(it, subject);
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::PrefixAtomExponent(PrefixAtomExponent::new(
                        Prefix::Yocto,
                        Atom::Meter,
                        4,
                    ))
                );
            }
        }

        #[test]
        fn prefix_atom_annotation_test() {
            let mut subject = Term::PrefixAtomAnnotation(PrefixAtomAnnotation::new(
                Prefix::Yocto,
                Atom::Meter,
                "foo".into(),
            ));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, Term::Annotation("foo".into()));
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(it, subject);
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation::new(
                        Prefix::Yocto,
                        Atom::Meter,
                        4,
                        "foo".into()
                    ))
                );
            }
        }

        #[test]
        fn prefix_atom_exponent_test() {
            let mut subject =
                Term::PrefixAtomExponent(PrefixAtomExponent::new(Prefix::Yocto, Atom::Meter, 2));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, UNITY);
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(
                    it,
                    Term::PrefixAtom(PrefixAtom::new(Prefix::Yocto, Atom::Meter))
                );
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::PrefixAtomExponent(PrefixAtomExponent::new(
                        Prefix::Yocto,
                        Atom::Meter,
                        4,
                    ))
                );
            }
        }

        #[test]
        fn prefix_atom_exponent_annotation_test() {
            let mut subject = Term::PrefixAtomExponentAnnotation(
                PrefixAtomExponentAnnotation::new(Prefix::Yocto, Atom::Meter, 2, "foo".into()),
            );

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, Term::Annotation("foo".into()));
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(
                    it,
                    Term::PrefixAtomAnnotation(PrefixAtomAnnotation::new(
                        Prefix::Yocto,
                        Atom::Meter,
                        "foo".into()
                    ))
                );
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation::new(
                        Prefix::Yocto,
                        Atom::Meter,
                        4,
                        "foo".into()
                    ))
                );
            }
        }

        #[test]
        fn factor_test() {
            let mut subject = Term::Factor(42);

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, UNITY);
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(it, subject);
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::FactorExponent(FactorExponent {
                        factor: 42,
                        exponent: 4
                    })
                );
            }
        }

        #[test]
        fn factor_annotation_test() {
            let mut subject = Term::FactorAnnotation(FactorAnnotation::new(42, "foo".into()));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, Term::Annotation("foo".into()));
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(it, subject);
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::FactorExponentAnnotation(FactorExponentAnnotation {
                        factor: 42,
                        exponent: 4,
                        annotation: "foo".into()
                    })
                );
            }
        }

        #[test]
        fn factor_exponent_test() {
            let mut subject = Term::FactorExponent(FactorExponent::new(42, 2));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, UNITY);
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(it, Term::Factor(42));
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::FactorExponent(FactorExponent {
                        factor: 42,
                        exponent: 4,
                    })
                );
            }
        }

        #[test]
        fn factor_exponent_annotation_test() {
            let mut subject =
                Term::FactorExponentAnnotation(FactorExponentAnnotation::new(42, 2, "foo".into()));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, Term::Annotation("foo".into()));
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(
                    it,
                    Term::FactorAnnotation(FactorAnnotation::new(42, "foo".into()))
                );
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::FactorExponentAnnotation(FactorExponentAnnotation {
                        factor: 42,
                        exponent: 4,
                        annotation: "foo".into()
                    })
                );
            }
        }

        #[test]
        fn factor_atom_test() {
            let mut subject = Term::FactorAtom(FactorAtom::new(42, Atom::Meter));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, UNITY);
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(it, subject);
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::FactorAtomExponent(FactorAtomExponent {
                        factor: 42,
                        atom: Atom::Meter,
                        exponent: 4,
                    })
                );
            }
        }

        #[test]
        fn factor_atom_annotation_test() {
            let mut subject = Term::FactorAtomAnnotation(FactorAtomAnnotation::new(
                42,
                Atom::Meter,
                "foo".into(),
            ));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, Term::Annotation("foo".into()));
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(it, subject);
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                        factor: 42,
                        atom: Atom::Meter,
                        exponent: 4,
                        annotation: "foo".into()
                    })
                );
            }
        }

        #[test]
        fn factor_atom_exponent_test() {
            let mut subject = Term::FactorAtomExponent(FactorAtomExponent::new(42, Atom::Meter, 2));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, UNITY);
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(
                    it,
                    Term::FactorAtomExponent(FactorAtomExponent::new(42, Atom::Meter, 1))
                );
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::FactorAtomExponent(FactorAtomExponent {
                        factor: 42,
                        atom: Atom::Meter,
                        exponent: 4,
                    })
                );
            }
        }

        #[test]
        fn factor_atom_exponent_annotation_test() {
            let mut subject = Term::FactorAtomExponentAnnotation(
                FactorAtomExponentAnnotation::new(42, Atom::Meter, 2, "foo".into()),
            );

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, Term::Annotation("foo".into()));
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(
                    it,
                    Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation::new(
                        42,
                        Atom::Meter,
                        1,
                        "foo".into()
                    ))
                );
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                        factor: 42,
                        atom: Atom::Meter,
                        exponent: 4,
                        annotation: "foo".into()
                    })
                );
            }
        }

        #[test]
        fn factor_prefix_atom_test() {
            let mut subject =
                Term::FactorPrefixAtom(FactorPrefixAtom::new(42, Prefix::Yocto, Atom::Meter));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, UNITY);
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(it, subject);
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent {
                        factor: 42,
                        prefix: Prefix::Yocto,
                        atom: Atom::Meter,
                        exponent: 4,
                    })
                );
            }
        }

        #[test]
        fn factor_prefix_atom_annotation_test() {
            let mut subject = Term::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation::new(
                42,
                Prefix::Yocto,
                Atom::Meter,
                "foo".into(),
            ));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, Term::Annotation("foo".into()));
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(it, subject);
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                        factor: 42,
                        prefix: Prefix::Yocto,
                        atom: Atom::Meter,
                        exponent: 4,
                        annotation: "foo".into()
                    })
                );
            }
        }

        #[test]
        fn factor_prefix_atom_exponent_test() {
            let mut subject = Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent::new(
                42,
                Prefix::Yocto,
                Atom::Meter,
                2,
            ));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, UNITY);
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(
                    it,
                    Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent::new(
                        42,
                        Prefix::Yocto,
                        Atom::Meter,
                        1
                    ))
                );
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent {
                        factor: 42,
                        prefix: Prefix::Yocto,
                        atom: Atom::Meter,
                        exponent: 4,
                    })
                );
            }
        }

        #[test]
        fn factor_prefix_atom_exponent_annotation_test() {
            let mut subject =
                Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation::new(
                    42,
                    Prefix::Yocto,
                    Atom::Meter,
                    2,
                    "foo".into(),
                ));

            // 0
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(0);
                assert_eq!(it, Term::Annotation("foo".into()));
            }

            // 1
            {
                let mut it = subject.clone();
                let _ = it.set_exponent(1);
                assert_eq!(
                    it,
                    Term::FactorPrefixAtomExponentAnnotation(
                        FactorPrefixAtomExponentAnnotation::new(
                            42,
                            Prefix::Yocto,
                            Atom::Meter,
                            1,
                            "foo".into(),
                        )
                    )
                );
            }

            // n
            {
                let _ = subject.set_exponent(4);
                assert_eq!(
                    subject,
                    Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                        factor: 42,
                        prefix: Prefix::Yocto,
                        atom: Atom::Meter,
                        exponent: 4,
                        annotation: "foo".into()
                    })
                );
            }
        }
    }

    mod set_annotation {
        use super::*;

        macro_rules! test_non_annotation {
            ($subject_variant:ident, $expected_variant:ident, $($params:tt)+) => {
                let mut subject = Term::$subject_variant($subject_variant {
                    $($params)+
                });
                let _ = subject.set_annotation("bar");

                if let Term::$expected_variant($expected_variant { annotation, .. }) = subject {
                    assert_eq!(annotation.as_str(), "bar");
                } else {
                    panic!("Unexpected type! {:#?}", subject);
                }
            }
        }

        macro_rules! test_annotation {
            ($subject_variant:ident, $($params:tt)+) => {
                let mut subject = Term::$subject_variant($subject_variant {
                    annotation: "foo".into(),
                    $($params)+
                });
                let _ = subject.set_annotation("bar");

                if let Term::$subject_variant($subject_variant { annotation, .. }) = subject {
                    assert_eq!(annotation.as_str(), "bar");
                } else {
                    panic!("Unexpected type! {:#?}", subject);
                }
            }
        }

        #[test]
        fn annotation_test() {
            let mut subject = Term::Annotation("foo".into());
            let _ = subject.set_annotation("bar");

            assert_eq!(subject, Term::Annotation("bar".into()));
        }

        #[test]
        fn atom_test() {
            let mut subject = Term::Atom(Atom::Meter);
            let _ = subject.set_annotation("bar");

            assert_eq!(
                subject,
                Term::AtomAnnotation(AtomAnnotation::new(Atom::Meter, "bar".into()))
            );
        }

        #[test]
        fn atom_annotation_test() {
            test_annotation!(AtomAnnotation, atom: Atom::Meter);
        }

        #[test]
        fn atom_exponent_test() {
            test_non_annotation!(AtomExponent, AtomExponentAnnotation, atom: Atom::Meter, exponent: 2);
        }

        #[test]
        fn atom_exponent_annotation_test() {
            test_annotation!(AtomExponentAnnotation, atom: Atom::Meter, exponent: 2);
        }

        #[test]
        fn prefix_atom_test() {
            test_non_annotation!(PrefixAtom, PrefixAtomAnnotation, prefix: Prefix::Yocto, atom: Atom::Meter);
        }

        #[test]
        fn prefix_atom_annotation_test() {
            test_annotation!(PrefixAtomAnnotation, prefix: Prefix::Yocto, atom: Atom::Meter);
        }

        #[test]
        fn prefix_atom_exponent_test() {
            test_non_annotation!(
                PrefixAtomExponent,
                PrefixAtomExponentAnnotation,
                prefix: Prefix::Yocto,
                atom: Atom::Meter,
                exponent: 2
            );
        }

        #[test]
        fn prefix_atom_exponent_annotation_test() {
            test_annotation!(PrefixAtomExponentAnnotation,
                prefix: Prefix::Yocto,
                atom: Atom::Meter,
                exponent: 2
            );
        }

        #[test]
        fn factor_test() {
            let mut subject = Term::Factor(2);
            let _ = subject.set_annotation("bar");

            if let Term::FactorAnnotation(FactorAnnotation { annotation, .. }) = subject {
                assert_eq!(annotation.as_str(), "bar");
            } else {
                panic!("Unexpected type! {subject:#?}");
            }
        }

        #[test]
        fn factor_annotation_test() {
            test_annotation!(FactorAnnotation, factor: 2);
        }

        #[test]
        fn factor_exponent_test() {
            test_non_annotation!(
                FactorExponent,
                FactorExponentAnnotation,
                factor: 4,
                exponent: 2
            );
        }

        #[test]
        fn factor_exponent_annotation_test() {
            test_annotation!(FactorExponentAnnotation,
                factor: 4,
                exponent: 2
            );
        }

        #[test]
        fn factor_atom_test() {
            test_non_annotation!(
                FactorAtom,
                FactorAtomAnnotation,
                factor: 4,
                atom: Atom::Meter,
            );
        }

        #[test]
        fn factor_atom_annotation_test() {
            test_annotation!(FactorAtomAnnotation,
                factor: 4,
                atom: Atom::Meter,
            );
        }

        #[test]
        fn factor_atom_exponent_test() {
            test_non_annotation!(
                FactorAtomExponent,
                FactorAtomExponentAnnotation,
                factor: 4,
                atom: Atom::Meter,
                exponent: 2
            );
        }

        #[test]
        fn factor_atom_exponent_annotation_test() {
            test_annotation!(FactorAtomExponentAnnotation,
                factor: 4,
                atom: Atom::Meter,
                exponent: 2
            );
        }

        #[test]
        fn factor_prefix_atom_test() {
            test_non_annotation!(
                FactorPrefixAtom,
                FactorPrefixAtomAnnotation,
                factor: 4,
                prefix: Prefix::Yocto,
                atom: Atom::Meter,
            );
        }

        #[test]
        fn factor_prefix_atom_annotation_test() {
            test_annotation!(FactorPrefixAtomAnnotation,
                factor: 4,
                prefix: Prefix::Yocto,
                atom: Atom::Meter,
            );
        }

        #[test]
        fn factor_prefix_atom_exponent_test() {
            test_non_annotation!(
                FactorPrefixAtomExponent,
                FactorPrefixAtomExponentAnnotation,
                factor: 4,
                prefix: Prefix::Yocto,
                atom: Atom::Meter,
                exponent: 2
            );
        }

        #[test]
        fn factor_prefix_atom_exponent_annotation_test() {
            test_annotation!(FactorPrefixAtomExponentAnnotation,
                factor: 4,
                prefix: Prefix::Yocto,
                atom: Atom::Meter,
                exponent: 2
            );
        }
    }

    #[test]
    fn as_str_test() {
        assert_eq!(UNITY.as_cow_str(), "1");

        // None | Some(1), None, None, None, Some(ann)
        assert_eq!(term!(annotation: "hi").as_cow_str(), "{hi}");
        assert_eq!(term!(factor: 1, annotation: "hi").as_cow_str(), "{hi}");

        // None, None, Some(atom), None | Some(1), None
        assert_eq!(term!(Meter).as_cow_str(), "m");
        assert_eq!(term!(Meter, exponent: 1).as_cow_str(), "m");

        // None, None, Some(atom), None | Some(1), Some(ann)
        assert_eq!(term!(Meter, annotation: "hi").as_cow_str(), "m{hi}");
        assert_eq!(
            term!(Meter, exponent: 1, annotation: "hi").as_cow_str(),
            "m{hi}"
        );

        assert_eq!(term!(Meter, exponent: 2).as_cow_str(), "m2");
        assert_eq!(term!(Meter, exponent: -1).as_cow_str(), "m-1");

        assert_eq!(
            term!(Meter, exponent: 2, annotation: "hi").as_cow_str(),
            "m2{hi}"
        );

        assert_eq!(term!(Kilo, Meter).as_cow_str(), "km");

        assert_eq!(term!(Kilo, Meter, annotation: "hi").as_cow_str(), "km{hi}");

        assert_eq!(
            term!(Kilo, Meter, exponent: 1, annotation: "hi").as_cow_str(),
            "km{hi}"
        );
        assert_eq!(
            term!(Kilo, Meter, exponent: 2, annotation: "hi").as_cow_str(),
            "km2{hi}"
        );
    }
}
