use parser::{Atom, Prefix, Term};
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

/// Internal struct used for reducing `Term`s.
///
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct ComposableTerm {
    atom: Option<Atom>,
    prefix: Option<Prefix>,
    factor: Option<u32>,
}

impl<'a> From<&'a Term> for ComposableTerm {
    fn from(term: &'a Term) -> Self {
        ComposableTerm {
            atom: term.atom,
            prefix: term.prefix,
            factor: term.factor,
        }
    }
}

type Parts = (ComposableTerm, i32);

impl From<Parts> for Term {
    fn from(parts: Parts) -> Self {
        let e = if parts.1 == 1 {
            None
        } else {
            Some(parts.1)
        };

        Term {
            atom: parts.0.atom,
            prefix: parts.0.prefix,
            factor: parts.0.factor,
            exponent: e,
            annotation: None,
        }
    }
}

/// Function used in `Unit` for reducing its `Term`s.
///
pub(super) fn reduce_terms(terms: &[Term]) -> Vec<Term> {
    let map = reduce_to_map(terms);

    // Reconstructs the map into the Vec<Term>.
    map.into_iter()
        .map(|parts| Term::from(parts))
        .collect()
}

/// Iterates through `terms`, finds `Term`s that have the same attributes that determine
/// uniqueness (`atom`, `prefix`, `factor`), and sums those exponents. This is the destructuring
/// part of `reduce_terms()`.
///
fn reduce_to_map(terms: &[Term]) -> BTreeMap<ComposableTerm, i32> {
    terms.iter()
        .fold(BTreeMap::<ComposableTerm, i32>::new(), |mut map, term| {
            let exponent = term.exponent.unwrap_or(1);
            let key = ComposableTerm::from(term);

            update_map(&mut map, key, exponent);
            map
        })
}

/// Logic for how to combine a new `key`/`exponent` pair in the `map`: if the `key` exists in
/// `map`, add the value + `exponent`; if it does not yet exist, insert it.
///
fn update_map(map: &mut BTreeMap<ComposableTerm, i32>, key: ComposableTerm, exponent: i32) {
    match map.entry(key) {
        Entry::Vacant(entry) => {
            entry.insert(exponent);
        }
        Entry::Occupied(mut entry) => {
            *entry.get_mut() += exponent;
        }
    }
}
