use num_traits::Inv;
use pest::iterators::{Pair, Pairs};

use crate::parser::{terms::term_parser::Rule, Error, Term};

use super::{AstTerm, Finishable, Visit};

pub(super) struct MainTerm {
    pub(super) terms: Vec<Term>,
}

impl Visit<Rule> for MainTerm {
    // NOTE: Look at `term.pest` to see that parsing a `MainTerm` can only result in 2 options:
    // 1. a slash-term (ex. "/100m{stuff}.[ft_i]")
    // 2. a term (ex. "100m{stuff}.[ft_i]/cm")
    //
    // ...which means our first `Pairs` match can either be 1. a slash or 2. a term; if it's 1,
    // then we're in case 1 above; if it's 2 then we're in case 2 above.
    fn visit(pair: Pair<'_, Rule>) -> Result<Self, Error> {
        enum OneOrTwoTerms {
            Term(MainTerm),
            Slash,
        }

        fn parse_first(pairs: &mut Pairs<'_, Rule>) -> Result<OneOrTwoTerms, Error> {
            match pairs.next() {
                Some(first) => match first.as_rule() {
                    Rule::term => Ok(OneOrTwoTerms::Term(MainTerm {
                        terms: AstTerm::visit(first)?.finish(),
                    })),
                    // Don't do anything, but because we proceed through this method, we can later
                    // assume that we parsed a slash.
                    Rule::slash => Ok(OneOrTwoTerms::Slash),
                    _ => unreachable!(),
                },
                None => unreachable!(),
            }
        }

        fn parse_second(pairs: &mut Pairs<'_, Rule>) -> Result<MainTerm, Error> {
            match pairs.next() {
                Some(second) => match second.as_rule() {
                    Rule::term => {
                        let terms: Vec<Term> = AstTerm::visit(second)?.finish();

                        // If we're here it's because there was a leading slash, so invert.
                        let u = crate::Unit::new(terms).inv();

                        Ok(MainTerm {
                            terms: u.into_terms().to_vec(),
                        })
                    }
                    _ => unreachable!(),
                },
                None => unreachable!(),
            }
        }

        let mut pairs = pair.into_inner();

        match parse_first(&mut pairs)? {
            OneOrTwoTerms::Term(main_term) => Ok(main_term),
            OneOrTwoTerms::Slash => parse_second(&mut pairs),
        }
    }
}

impl From<MainTerm> for Vec<Term> {
    fn from(main_term: MainTerm) -> Self {
        main_term.terms
    }
}
