//!
//! This module provides a single internal function for reducing the `Term`s that make up a `Unit`.
//! It aims, for example, to turn a `Unit` that represents `"m3/m2"` into one that represents
//! `"m"`.
//!
//! The main function here, `reduce_terms()` works by classifying `Term`s by all fields _except_
//! their `exponent`. It adds up the exponents of each `Term`, which effectively reduces those
//! `Term`s. Put differently, the algorithm will only reduce `Term`s that have all non-exponent
//! fields in common. Some examples:
//!
//! 1. It won't reduce `10m3/m2`, but will reduce `10m3/10m2` (to `10m`).
//! 2. It won't reduce `cm3/m2`, but will reduce `cm3/cm2` (to `cm`).
//! 3. It won't reduce `cm3{foo}/cm2`, but will reduce `cm3{foo}/cm2{foo}` (to `cm{foo}`).
//!
use std::{borrow::Cow, collections::BTreeMap};

use num_traits::Zero;

use crate::{
    term::{
        variants::{
            AtomAnnotation, AtomExponent, AtomExponentAnnotation, FactorAnnotation, FactorAtom,
            FactorAtomAnnotation, FactorAtomExponent, FactorAtomExponentAnnotation, FactorExponent,
            FactorExponentAnnotation, FactorPrefixAtom, FactorPrefixAtomAnnotation,
            FactorPrefixAtomExponent, FactorPrefixAtomExponentAnnotation, PrefixAtom,
            PrefixAtomAnnotation, PrefixAtomExponent, PrefixAtomExponentAnnotation,
        },
        Exponent, Factor, UNITY,
    },
    Annotation, Atom, Prefix, Term,
};

/// Function used in `Unit` for reducing its `Term`s.
///
/// This takes `terms` by value due to how this function sits in relation to `Unit`: any time
/// `reduce_terms()` is called, we're going from some set of terms to 1) the same set, 2) a set
/// with some elements removed, or 3) a set without any elements. By taking the `Term`s by value
/// here instead of references, there's a good chance we can reuse some of those `Term`s in the
/// result of this call (if we passed a reference/slice instead, we'd have to clone the originals,
/// and this is most likely to happen).
///
pub(super) fn reduce_terms(terms: Vec<Term>) -> Cow<'static, [Term]> {
    let map = reduce_to_map(terms);

    // If everything is reduced away, the effective Unit should be "1".
    if map.is_empty() {
        Cow::Borrowed(crate::term::UNITY_ARRAY_REF)
    } else {
        // Reconstructs the map into the Vec<Term>.
        map.into_iter().map(Term::from).collect()
    }
}

/// Iterates through `terms`, finds `Term`s that have the same attributes that determine
/// uniqueness (`atom`, `prefix`, `factor`), and sums those exponents. This is the destructuring
/// part of `reduce_terms()`.
///
fn reduce_to_map(terms: Vec<Term>) -> BTreeMap<ComposableTerm, Exponent> {
    let mut map = BTreeMap::new();

    for term in terms {
        let exponent = term.effective_exponent();
        let key = ComposableTerm::from(term);

        let _ = map
            .entry(key)
            .and_modify(|entry| *entry += exponent)
            .or_insert_with(|| exponent);
    }

    // If the summing of an exponent is 0, that means it has been reduced away (so remove it from
    // the output).
    map.retain(|_, exponent| !exponent.is_zero());

    map
}

/// Internal enum used for reducing `Term`s.
///
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct ComposableTerm {
    factor: Option<Factor>,
    prefix: Option<Prefix>,
    atom: Option<Atom>,
    annotation: Option<Annotation>,
}

impl ComposableTerm {
    fn set_factor(self, factor: Factor) -> Self {
        Self {
            factor: Some(factor),
            ..self
        }
    }

    fn set_prefix(self, prefix: Prefix) -> Self {
        Self {
            prefix: Some(prefix),
            ..self
        }
    }

    fn set_atom(self, atom: Atom) -> Self {
        Self {
            atom: Some(atom),
            ..self
        }
    }

    fn set_annotation(self, annotation: Annotation) -> Self {
        Self {
            annotation: Some(annotation),
            ..self
        }
    }
}

impl From<Term> for ComposableTerm {
    fn from(term: Term) -> Self {
        match term {
            Term::Annotation(annotation) => Self::default().set_annotation(annotation),
            Term::Atom(atom) | Term::AtomExponent(AtomExponent { atom, .. }) => {
                Self::default().set_atom(atom)
            }
            Term::AtomAnnotation(AtomAnnotation { atom, annotation })
            | Term::AtomExponentAnnotation(AtomExponentAnnotation {
                atom, annotation, ..
            }) => Self::default().set_atom(atom).set_annotation(annotation),
            Term::PrefixAtom(PrefixAtom { prefix, atom })
            | Term::PrefixAtomExponent(PrefixAtomExponent { prefix, atom, .. }) => {
                Self::default().set_prefix(prefix).set_atom(atom)
            }
            Term::PrefixAtomAnnotation(PrefixAtomAnnotation {
                prefix,
                atom,
                annotation,
            })
            | Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
                prefix,
                atom,
                annotation,
                ..
            }) => Self::default()
                .set_prefix(prefix)
                .set_atom(atom)
                .set_annotation(annotation),
            Term::Factor(factor) | Term::FactorExponent(FactorExponent { factor, .. }) => {
                Self::default().set_factor(factor)
            }
            Term::FactorAnnotation(FactorAnnotation { factor, annotation })
            | Term::FactorExponentAnnotation(FactorExponentAnnotation {
                factor, annotation, ..
            }) => Self::default()
                .set_factor(factor)
                .set_annotation(annotation),

            Term::FactorAtom(FactorAtom { factor, atom })
            | Term::FactorAtomExponent(FactorAtomExponent { factor, atom, .. }) => {
                Self::default().set_factor(factor).set_atom(atom)
            }
            Term::FactorAtomAnnotation(FactorAtomAnnotation {
                factor,
                atom,
                annotation,
            })
            | Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                factor,
                atom,
                annotation,
                ..
            }) => Self::default()
                .set_factor(factor)
                .set_atom(atom)
                .set_annotation(annotation),
            Term::FactorPrefixAtom(FactorPrefixAtom {
                factor,
                prefix,
                atom,
            })
            | Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent {
                factor,
                prefix,
                atom,
                ..
            }) => Self::default()
                .set_factor(factor)
                .set_prefix(prefix)
                .set_atom(atom),
            Term::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation {
                factor,
                prefix,
                atom,
                annotation,
            })
            | Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                factor,
                prefix,
                atom,
                annotation,
                ..
            }) => Self::default()
                .set_factor(factor)
                .set_prefix(prefix)
                .set_atom(atom)
                .set_annotation(annotation),
        }
    }
}

// NOTE: To be clear, this is only for when the `Term` has NO `exponent`.
//
impl From<ComposableTerm> for Term {
    fn from(value: ComposableTerm) -> Self {
        match (value.factor, value.prefix, value.atom, value.annotation) {
            (None, None, None, None) => unreachable!(),
            (None, None, None, Some(ann)) => Self::Annotation(ann),
            (None, None, Some(atom), None) => Self::Atom(atom),
            (None, None, Some(atom), Some(ann)) => {
                Self::AtomAnnotation(AtomAnnotation::new(atom, ann))
            }
            (None, Some(_prefix), None, None) => unreachable!(),
            (None, Some(_prefix), None, Some(_ann)) => unreachable!(),
            (None, Some(prefix), Some(atom), None) => {
                Self::PrefixAtom(PrefixAtom::new(prefix, atom))
            }
            (None, Some(prefix), Some(atom), Some(ann)) => {
                Self::PrefixAtomAnnotation(PrefixAtomAnnotation::new(prefix, atom, ann))
            }
            (Some(factor), None, None, None) => Self::Factor(factor),
            (Some(factor), None, None, Some(ann)) => {
                Self::FactorAnnotation(FactorAnnotation::new(factor, ann))
            }
            (Some(factor), None, Some(atom), None) => {
                Self::FactorAtom(FactorAtom::new(factor, atom))
            }
            (Some(factor), None, Some(atom), Some(ann)) => {
                Self::FactorAtomAnnotation(FactorAtomAnnotation::new(factor, atom, ann))
            }
            (Some(_factor), Some(_prefix), None, None) => unreachable!(),
            (Some(_factor), Some(_prefix), None, Some(_ann)) => unreachable!(),
            (Some(factor), Some(prefix), Some(atom), None) => {
                Self::FactorPrefixAtom(FactorPrefixAtom::new(factor, prefix, atom))
            }
            (Some(factor), Some(prefix), Some(atom), Some(ann)) => {
                Self::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation::new(
                    factor, prefix, atom, ann,
                ))
            }
        }
    }
}

type Parts = (ComposableTerm, Exponent);

impl From<Parts> for Term {
    fn from(parts: Parts) -> Self {
        match parts.1 {
            // For anything to the 0th power:
            // - any variant that has an annotation, keep only the annotation
            // - all other variants, return the Unity (1).
            0 => parts.0.annotation.map_or(UNITY, Self::Annotation),

            exponent => {
                let mut t = Self::from(parts.0);
                let _ = t.set_exponent(exponent);
                t
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_terms_test() {
        assert_eq!(reduce_terms(vec![]), vec![UNITY]);
    }

    #[test]
    fn single_term_test() {
        let term =
            Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation::new(
                42,
                Prefix::Kilo,
                Atom::Meter,
                3,
                Annotation::from("foo"),
            ));
        assert_eq!(reduce_terms(vec![term.clone()]), vec![term]);
    }

    // NOTE: The two terms only differ in their `annotation`s here. (and `exponent`, but that field
    // isn't used in grouping terms);
    //
    #[test]
    fn two_unrelated_term_test() {
        let term1 =
            Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation::new(
                42,
                Prefix::Kilo,
                Atom::Meter,
                3,
                Annotation::from("foo"),
            ));
        let term2 =
            Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation::new(
                42,
                Prefix::Kilo,
                Atom::Gram,
                2,
                Annotation::from("bar"),
            ));
        assert_eq!(
            reduce_terms(vec![term1.clone(), term2.clone()]),
            vec![term1, term2]
        );
    }

    #[test]
    fn two_related_term_test() {
        let term1 =
            Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation::new(
                42,
                Prefix::Kilo,
                Atom::Meter,
                3,
                Annotation::from("foo"),
            ));
        let term2 =
            Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation::new(
                42,
                Prefix::Kilo,
                Atom::Meter,
                -2,
                Annotation::from("foo"),
            ));

        assert_eq!(
            reduce_terms(vec![term1, term2]),
            vec![Term::FactorPrefixAtomAnnotation(
                FactorPrefixAtomAnnotation::new(
                    42,
                    Prefix::Kilo,
                    Atom::Meter,
                    Annotation::from("foo")
                )
            )]
        );
    }

    #[test]
    fn three_related_term_test() {
        let term1 = Term::PrefixAtomExponent(PrefixAtomExponent::new(Prefix::Kilo, Atom::Meter, 3));
        let term2 =
            Term::PrefixAtomExponent(PrefixAtomExponent::new(Prefix::Kilo, Atom::Meter, -2));
        let term3 = Term::PrefixAtom(PrefixAtom::new(Prefix::Kilo, Atom::Meter));

        assert_eq!(
            reduce_terms(vec![term1, term2, term3]),
            vec![Term::PrefixAtomExponent(PrefixAtomExponent::new(
                Prefix::Kilo,
                Atom::Meter,
                2
            ))]
        );
    }

    #[test]
    fn three_term_two_related_test() {
        let term1 = Term::PrefixAtomExponent(PrefixAtomExponent::new(Prefix::Kilo, Atom::Meter, 3));
        let term2 =
            Term::PrefixAtomExponent(PrefixAtomExponent::new(Prefix::Kilo, Atom::Meter, -2));
        let term3 = Term::PrefixAtomExponent(PrefixAtomExponent::new(Prefix::Kilo, Atom::Gram, 2));

        assert_eq!(
            reduce_terms(vec![term1, term2, term3]),
            vec![
                Term::PrefixAtom(PrefixAtom::new(Prefix::Kilo, Atom::Meter,)),
                Term::PrefixAtomExponent(PrefixAtomExponent::new(Prefix::Kilo, Atom::Gram, 2))
            ]
        );
    }
}
