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

use crate::Term;

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
    let mut ids_to_remove = vec![];
    let mut output = terms.clone();

    for (i, lhs) in output.iter_mut().enumerate() {
        if ids_to_remove.contains(&i) {
            continue;
        }
        let mut offset = i + 1;

        'inner: for (j, rhs) in terms[offset..].iter().enumerate() {
            if ids_to_remove.contains(&(offset + j)) {
                continue 'inner;
            }

            if composably_eq(lhs, rhs) {
                let new_exponent = lhs.effective_exponent() + rhs.effective_exponent();

                let _ = lhs.set_exponent(new_exponent);

                ids_to_remove.push(offset);
            }
            offset += 1;
        }
    }

    ids_to_remove.sort_unstable();
    ids_to_remove.reverse();

    for id in ids_to_remove {
        let _ = output.remove(id);
    }

    // If everything is reduced away, the effective Unit should be "1".
    if output.is_empty() {
        Cow::Borrowed(crate::term::UNITY_ARRAY_REF)
    } else {
        output.into()
    }
}

fn composably_eq(lhs: &Term, rhs: &Term) -> bool {
    lhs.atom() == rhs.atom()
        && lhs.prefix() == rhs.prefix()
        && lhs.factor() == rhs.factor()
        && lhs.annotation() == rhs.annotation()
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
