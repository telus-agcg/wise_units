use pest::{iterators::Pair, Parser};

use crate::{
    term::{variants::PrefixAtom, UNITY},
    unit::parser::{
        symbols::{
            mapper as symbol_mapper,
            symbol_parser::{Rule as SymbolRule, SymbolParser},
            Symbol,
        },
        terms::term_parser::Rule as TermRule,
        Error, Visit,
    },
    Term,
};

pub(super) struct SimpleUnit(pub(super) Term);

impl Visit<'_, TermRule> for SimpleUnit {
    fn visit(pair: Pair<'_, TermRule>) -> Result<Self, Error> {
        let string = pair.as_span().as_str();

        if string == "1" {
            return Ok(Self(UNITY));
        }

        if let Ok(mut symbol_pairs) = SymbolParser::parse(SymbolRule::symbol, string) {
            match symbol_mapper::map(symbol_pairs.next().unwrap())? {
                Symbol::PrimaryPrefixed { prefix, atom }
                | Symbol::SecondaryPrefixed { prefix, atom } => {
                    Ok(Self(Term::PrefixAtom(PrefixAtom { prefix, atom })))
                }
                Symbol::PrimaryBasic { atom } | Symbol::SecondaryBasic { atom } => {
                    Ok(Self(Term::Atom(atom)))
                }
                Symbol::Unity => Ok(Self(UNITY)),
            }
        } else {
            Err(Error::BadFragment {
                fragment: string.to_string(),
                position: pair.as_span().start(),
            })
        }
    }
}
