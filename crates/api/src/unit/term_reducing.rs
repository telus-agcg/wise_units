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
    term::term_reduce::{ReducedTerm, TermReduce},
    FieldEq, Term,
};

/// This is the algorithm for determining if & how to combine or cancel out Terms when multiplying
/// and dividing Units/Measurements.
///
/// * `lhs_terms`: The `Term`s from the left-hand side of the operation.
/// * `rhs_terms`: The `Term`s from the right-hand side of the operation.
///
pub(super) fn compare_and_cancel(lhs_terms: &[Term], rhs_terms: Vec<Term>) -> Cow<'static, [Term]> {
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
    let mut output: Vec<Term> = vec![];
    let mut remaining_rhs = rhs_terms;

    // Iterate through all LHS terms, comparing them to RHS terms, reducing away or combining Terms
    // as needed.
    for lhs in lhs_terms {
        let what_to_keep = simplify_one_to_many(lhs, &remaining_rhs);

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

    // `remaining_rhs` are the RHS Terms that couldn't be combined with any LHS ones.
    output.extend_from_slice(&remaining_rhs);

    cleanup(output)
}

/// Helps gather information about what to keep, what got reduced, etc., when comparing a LHS Term
/// to the remaining RHS Terms.
///
#[derive(Debug, PartialEq)]
pub(super) struct WhatToKeep {
    /// If `true`, the `LHS` term that was passed in should be kept.
    ///
    pub(super) keep_lhs: bool,

    /// These are `Term`s that were combined from other terms. Ex. `m3` and `m-2` combined becomes
    /// `m`.
    ///
    pub(super) new_terms: Vec<Term>,

    /// These are RHS `Term`s that did not get reduced away (because they're not comparable to the
    /// LHS in this context).
    ///
    pub(super) kept_terms: Vec<Term>,
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

pub(super) fn simplify_one_to_many(lhs: &Term, rhs_terms: &[Term]) -> WhatToKeep {
    let mut what_to_keep = WhatToKeep::default();

    for (i, rhs) in rhs_terms.iter().enumerate() {
        match lhs.term_reduce(rhs) {
            ReducedTerm::ReducedToTerm(new_term) => {
                what_to_keep.keep_lhs = false;

                match rhs_terms.get((i + 1)..) {
                    Some(things) if !things.is_empty() => {
                        let wtk = simplify_one_to_many(&new_term, things);

                        if wtk.keep_lhs {
                            what_to_keep.new_terms.push(new_term);
                        }

                        what_to_keep.new_terms.extend(wtk.new_terms);
                        what_to_keep.kept_terms.extend(wtk.kept_terms);
                    }
                    _ => {
                        what_to_keep.new_terms.push(new_term);
                    }
                }

                return what_to_keep;
            }
            ReducedTerm::ReducedAway => {
                what_to_keep.keep_lhs = false;

                if let Some(things) = rhs_terms.get((i + 1)..) {
                    what_to_keep.kept_terms.extend_from_slice(things);
                }
                return what_to_keep;
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
    use crate::term::UNITY;

    use super::*;

    const METER: Term = Term::Atom(crate::atom::Atom::Meter);

    mod compare_one_to_many {
        use super::*;

        macro_rules! test_compare_one_to_many {
            ($test_name:ident, $lhs:expr, $rhs:expr, $expected:expr) => {
                #[test]
                fn $test_name() {
                    pretty_assertions::assert_eq!(simplify_one_to_many($lhs, $rhs), $expected);
                }
            };
        }

        test_compare_one_to_many!(
            test_unity_vs_empty_slice,
            &UNITY,
            &[],
            WhatToKeep::default()
        );

        test_compare_one_to_many!(
            test_meter_vs_meter,
            &METER,
            &[METER],
            WhatToKeep {
                keep_lhs: false,
                new_terms: vec![term!(Meter, exponent: 2)],
                kept_terms: vec![]
            }
        );

        test_compare_one_to_many!(
            test_meter_vs_meter_minus_1,
            &METER,
            &[term!(Meter, exponent: -1)],
            WhatToKeep {
                keep_lhs: false,
                new_terms: vec![],
                kept_terms: vec![]
            }
        );

        test_compare_one_to_many!(
            test_meter_vs_meter_minus_2,
            &METER,
            &[term!(Meter, exponent: -2)],
            WhatToKeep {
                keep_lhs: false,
                new_terms: vec![term!(Meter, exponent: -1)],
                kept_terms: vec![]
            }
        );

        test_compare_one_to_many!(
            test_unity_vs_meter,
            &UNITY,
            &[METER],
            WhatToKeep {
                keep_lhs: true,
                new_terms: vec![],
                kept_terms: vec![METER]
            }
        );

        test_compare_one_to_many!(
            test_meter_vs_gram,
            &METER,
            &[term!(Gram)],
            WhatToKeep {
                keep_lhs: true,
                new_terms: vec![],
                kept_terms: vec![term!(Gram)]
            }
        );

        test_compare_one_to_many!(
            test_meter_vs_meter_meter_meter,
            &METER,
            &[METER, METER, METER],
            WhatToKeep {
                keep_lhs: false,
                new_terms: vec![term!(Meter, exponent: 4)],
                kept_terms: vec![]
            }
        );
    }
}
