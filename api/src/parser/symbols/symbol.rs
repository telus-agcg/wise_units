use crate::parser::{symbols::symbol_parser::Rule, Atom, Error, Prefix, Visit};
use pest::iterators::Pair;

// Technically there should never be a symbol without an atom.
#[derive(Debug)]
pub(crate) enum Symbol {
    PrimaryPrefixed { prefix: Prefix, atom: Atom },
    PrimaryBasic { atom: Atom },
    SecondaryPrefixed { prefix: Prefix, atom: Atom },
    SecondaryBasic { atom: Atom },
    Unity,
}

impl Visit<Rule> for Symbol {
    fn visit(pair: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut pairs = pair.into_inner();

        let first_token = match pairs.next() {
            Some(first) => match first.as_rule() {
                Rule::pri_prefix => {
                    FirstToken::PrimaryPrefix(Prefix::visit(first.into_inner().next().unwrap())?)
                }
                Rule::sec_prefix => {
                    FirstToken::SecondaryPrefix(Prefix::visit(first.into_inner().next().unwrap())?)
                }
                Rule::pri_atom => {
                    FirstToken::PrimaryAtom(Atom::visit(first.into_inner().next().unwrap())?)
                }
                Rule::sec_atom => {
                    FirstToken::SecondaryAtom(Atom::visit(first.into_inner().next().unwrap())?)
                }
                Rule::EOI => {
                    return Ok(Self::Unity);
                }
                _ => unreachable!(),
            },
            None => unreachable!(),
        };

        let symbol = match first_token {
            FirstToken::PrimaryPrefix(prefix) => match pairs.next() {
                Some(second) => match second.as_rule() {
                    Rule::pri_atom => Self::PrimaryPrefixed {
                        prefix,
                        atom: { Atom::visit(second.into_inner().next().unwrap())? },
                    },
                    _ => unreachable!(),
                },
                None => unreachable!(),
            },
            FirstToken::SecondaryPrefix(prefix) => match pairs.next() {
                Some(second) => match second.as_rule() {
                    Rule::sec_atom => Self::SecondaryPrefixed {
                        prefix,
                        atom: Atom::visit(second.into_inner().next().unwrap())?,
                    },
                    _ => unreachable!(),
                },
                None => unreachable!(),
            },
            FirstToken::PrimaryAtom(atom) => Self::PrimaryBasic { atom },
            FirstToken::SecondaryAtom(atom) => Self::SecondaryBasic { atom },
        };

        match pairs.next() {
            Some(third) => match third.as_rule() {
                Rule::EOI => Ok(symbol),
                _ => unreachable!(),
            },
            None => Ok(symbol),
        }
    }
}

enum FirstToken {
    PrimaryPrefix(Prefix),
    PrimaryAtom(Atom),
    SecondaryPrefix(Prefix),
    SecondaryAtom(Atom),
}
