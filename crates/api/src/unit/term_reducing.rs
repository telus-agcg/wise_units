use crate::parser::{Atom, Prefix, Term};
use std::collections::BTreeMap;

/// Internal struct used for reducing `Term`s.
///
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ComposableTerm {
    factor: Option<u32>,
    prefix: Option<Prefix>,
    atom: Option<Atom>,
    annotation: Option<String>,
}

impl ComposableTerm {
    const fn has_value(&self) -> bool {
        self.atom.is_some() || self.factor.is_some() || self.annotation.is_some()
    }
}

impl<'a> From<&'a Term> for ComposableTerm {
    fn from(term: &'a Term) -> Self {
        Self {
            atom: term.atom,
            prefix: term.prefix,
            factor: term.factor,
            annotation: term.annotation.clone(),
        }
    }
}

type Parts = (ComposableTerm, i32);

impl From<Parts> for Term {
    fn from(parts: Parts) -> Self {
        let e = if parts.1 == 1 { None } else { Some(parts.1) };

        Self {
            atom: parts.0.atom,
            prefix: parts.0.prefix,
            factor: parts.0.factor,
            exponent: e,
            annotation: parts.0.annotation,
        }
    }
}

/// Function used in `Unit` for reducing its `Term`s.
///
pub(super) fn reduce_terms(terms: &[Term]) -> Vec<Term> {
    let map = reduce_to_map(terms);

    // If everything is reduced away, the effective Unit should be "1".
    if map.is_empty() {
        vec![Term::new_unity()]
    } else {
        // Reconstructs the map into the Vec<Term>.
        map.into_iter().map(Term::from).collect()
    }
}

/// Iterates through `terms`, finds `Term`s that have the same attributes that determine
/// uniqueness (`atom`, `prefix`, `factor`), and sums those exponents. This is the destructuring
/// part of `reduce_terms()`.
///
fn reduce_to_map(terms: &[Term]) -> BTreeMap<ComposableTerm, i32> {
    terms
        .iter()
        .map(|term| (ComposableTerm::from(term), term.exponent.unwrap_or(1)))
        .fold(
            BTreeMap::<ComposableTerm, i32>::new(),
            |mut map, (key, exponent)| {
                let _ = map
                    .entry(key)
                    .and_modify(|entry| *entry += exponent)
                    .or_insert(exponent);

                map
            },
        )
        .into_iter()
        // Filter out things that have no values
        .filter(|(ct, exponent)| ct.has_value() && *exponent != 0)
        .collect()
}
