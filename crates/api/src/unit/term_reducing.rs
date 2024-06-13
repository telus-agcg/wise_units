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
use std::borrow::Cow;

use crate::{
    composable::ComposablyEq,
    term::term_reduce::{ReducedTerm, TermReduce},
    FieldEq, Term,
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
pub(super) fn reduce_terms(terms: &[Term]) -> Cow<'static, [Term]> {
    let mut ids_to_remove = vec![];
    let mut output = vec![];

    'outer: for (i, lhs) in terms.iter().enumerate() {
        if ids_to_remove.contains(&i) {
            continue;
        }

        let mut offset = i + 1;
        let mut new_term = lhs.clone();

        'inner: for (j, rhs) in terms[offset..].iter().enumerate() {
            if ids_to_remove.contains(&(offset + j)) {
                continue 'inner;
            }

            match lhs.composably_eq(rhs) {
                Some(0) => {
                    ids_to_remove.push(i);
                    ids_to_remove.push(offset);
                    continue 'outer;
                }
                Some(exponent) => {
                    let _ = new_term.set_exponent(exponent);

                    ids_to_remove.push(offset);
                }
                None => (),
            }

            offset += 1;
        }

        output.push(new_term);
    }

    cleanup(output)
}

fn cleanup(mut output: Vec<Term>) -> Cow<'static, [Term]> {
    // If everything is reduced away, the effective Unit should be "1".
    if output.is_empty() {
        Cow::Borrowed(crate::term::UNITY_ARRAY_REF)
    } else {
        if output.len() > 1 {
            output.retain(|t| !t.field_eq(&crate::term::UNITY));
        }
        output.into()
    }
}

/// This is the algorithm for determining if & how to combine or cancel out Terms when multiplying
/// and dividing Units/Measurements.
///
/// * `lhs_terms`: The `Term`s from the left-hand side of the operation.
/// * `rhs_terms`: The `Term`s from the right-hand side of the operation.
///
pub(super) fn meow(lhs_terms: &[Term], rhs_terms: Vec<Term>) -> Cow<'static, [Term]> {
    let mut output: Vec<Term> = vec![];
    let mut remaining_rhs = rhs_terms;

    for lhs in lhs_terms {
        let what_to_keep = lhs_single_rhs(lhs, &remaining_rhs);

        if what_to_keep.keep_lhs {
            output.push(lhs.clone());
        }

        output.extend(what_to_keep.new_terms);
        remaining_rhs = what_to_keep.kept_terms;

        // Break if there is nothing left on the RHS to compare the LHS to.
        if remaining_rhs.is_empty() {
            break;
        }
    }

    output.extend_from_slice(&remaining_rhs);

    cleanup(output)
}

#[derive(Debug)]
struct WhatToKeep {
    keep_lhs: bool,
    new_terms: Vec<Term>,
    kept_terms: Vec<Term>,
}

impl Default for WhatToKeep {
    fn default() -> Self {
        Self {
            keep_lhs: true,
            new_terms: Vec::default(),
            kept_terms: Vec::default(),
        }
    }
}

// TODO: Change back to Cow
fn lhs_single_rhs(lhs: &Term, rhs_terms: &[Term]) -> WhatToKeep {
    let mut what_to_keep = WhatToKeep::default();

    for (i, rhs) in rhs_terms.iter().enumerate() {
        match lhs.term_reduce(rhs) {
            ReducedTerm::ReducedToTerm(new_term) => {
                what_to_keep.new_terms.push(new_term);
                what_to_keep.keep_lhs = false;

                if let Some(things) = rhs_terms.get((i + 1)..) {
                    what_to_keep.kept_terms.extend_from_slice(things);
                }

                break;
            }
            ReducedTerm::ReducedAway => {
                what_to_keep.keep_lhs = false;

                if let Some(things) = rhs_terms.get((i + 1)..) {
                    what_to_keep.kept_terms.extend_from_slice(things);
                }
                break;
            }
            ReducedTerm::NotReducible => {
                what_to_keep.kept_terms.push(rhs.clone());
            }
        }
    }

    what_to_keep
}

#[cfg(test)]
mod tests {
    use crate::{
        term::{
            variants::{
                FactorPrefixAtomAnnotation, FactorPrefixAtomExponentAnnotation, PrefixAtom,
                PrefixAtomExponent,
            },
            UNITY,
        },
        Annotation, Atom, Prefix,
    };

    use super::*;

    #[test]
    fn empty_terms_test() {
        assert_eq!(reduce_terms(&[]), vec![UNITY]);
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
        assert_eq!(reduce_terms(&[term.clone()]), vec![term]);
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
            reduce_terms(&[term1.clone(), term2.clone()]),
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
            reduce_terms(&[term1, term2]),
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
            reduce_terms(&[term1, term2, term3]),
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
            reduce_terms(&[term1, term2, term3]),
            vec![
                Term::PrefixAtom(PrefixAtom::new(Prefix::Kilo, Atom::Meter,)),
                Term::PrefixAtomExponent(PrefixAtomExponent::new(Prefix::Kilo, Atom::Gram, 2))
            ]
        );
    }
}
