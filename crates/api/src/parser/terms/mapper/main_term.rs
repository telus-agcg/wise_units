use num_traits::Inv;
use pest::iterators::Pair;

use crate::parser::{terms::term_parser::Rule, Error, Term};

use super::{AstTerm, Finishable, Visit};

pub(super) struct MainTerm {
    pub(super) terms: Vec<Term>,
}

impl Visit<Rule> for MainTerm {
    fn visit(pair: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut pairs = pair.into_inner();

        // match first token
        match pairs.next() {
            Some(first) => match first.as_rule() {
                Rule::term => {
                    return Ok(Self {
                        terms: AstTerm::visit(first)?.finish(),
                    });
                }
                // Don't do anything, but because we proceed through this method, we can later
                // assume that we parsed a slash.
                Rule::slash => (),
                _ => unreachable!(),
            },
            None => unreachable!(),
        }

        match pairs.next() {
            Some(second) => match second.as_rule() {
                Rule::term => {
                    let terms: Vec<Term> = AstTerm::visit(second)?.finish();

                    // If we're here it's because there was a leading slash, so invert.
                    let u = crate::Unit::new(terms).inv();

                    Ok(Self {
                        terms: u.into_terms().to_vec(),
                    })
                }
                _ => unreachable!(),
            },
            None => unreachable!(),
        }
    }
}

impl From<MainTerm> for Vec<Term> {
    fn from(main_term: MainTerm) -> Self {
        main_term.terms
    }
}
