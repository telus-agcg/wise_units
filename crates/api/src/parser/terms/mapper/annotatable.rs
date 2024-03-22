use pest::iterators::Pair;

use crate::parser::{term::Exponent as IExponent, terms::term_parser::Rule as TermRule};

use super::{Atom, Error, Exponent, Prefix, SimpleUnit, Visit};

pub(super) enum Annotatable {
    PrefixedWithExponent {
        prefix: Prefix,
        atom: Atom,
        exponent: IExponent,
    },
    Prefixed {
        prefix: Prefix,
        atom: Atom,
    },
    BasicWithExponent {
        atom: Atom,
        exponent: IExponent,
    },
    Basic {
        atom: Atom,
    },
    Unity,
}

impl Visit<TermRule> for Annotatable {
    fn visit(pair: Pair<'_, TermRule>) -> Result<Self, Error> {
        let mut pairs = pair.into_inner();

        let simple_unit = match pairs.next() {
            Some(first) => match first.as_rule() {
                TermRule::simple_unit => SimpleUnit::visit(first)?,
                _ => unreachable!(),
            },
            None => unreachable!(),
        };

        match simple_unit {
            SimpleUnit::Prefixed { prefix, atom } => match pairs.next() {
                Some(second) => match second.as_rule() {
                    TermRule::exponent => Ok(Self::PrefixedWithExponent {
                        prefix,
                        atom,
                        exponent: Exponent::visit(second)?.0,
                    }),
                    _ => unreachable!(),
                },
                None => Ok(Self::Prefixed { prefix, atom }),
            },
            SimpleUnit::Basic { atom } => match pairs.next() {
                Some(second) => match second.as_rule() {
                    TermRule::exponent => Ok(Self::BasicWithExponent {
                        atom,
                        exponent: Exponent::visit(second)?.0,
                    }),
                    _ => unreachable!(),
                },
                None => Ok(Self::Basic { atom }),
            },
            SimpleUnit::Unity => Ok(Self::Unity),
        }
    }
}
