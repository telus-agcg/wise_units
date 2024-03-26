use pest::{iterators::Pair, Parser};

use crate::{
    parser::{
        symbols::{
            mapper as symbol_mapper,
            symbol_parser::{Rule as SymbolRule, SymbolParser},
            Symbol,
        },
        terms::term_parser::Rule as TermRule,
        Error, Visit,
    },
    Atom, Prefix,
};

pub(super) enum SimpleUnit {
    Prefixed { prefix: Prefix, atom: Atom },
    Basic { atom: Atom },
    Unity,
}

impl Visit<TermRule> for SimpleUnit {
    fn visit(pair: Pair<'_, TermRule>) -> Result<Self, Error> {
        let string = pair.as_span().as_str();

        if string == "1" {
            return Ok(Self::Unity);
        }

        if let Ok(mut symbol_pairs) = SymbolParser::parse(SymbolRule::symbol, string) {
            match symbol_mapper::map(symbol_pairs.next().unwrap())? {
                Symbol::PrimaryPrefixed { prefix, atom }
                | Symbol::SecondaryPrefixed { prefix, atom } => Ok(Self::Prefixed { prefix, atom }),
                Symbol::PrimaryBasic { atom } | Symbol::SecondaryBasic { atom } => {
                    Ok(Self::Basic { atom })
                }
                Symbol::Unity => Ok(Self::Unity),
            }
        } else {
            Err(Error::BadFragment {
                fragment: string.to_string(),
                position: pair.as_span().start(),
            })
        }
    }
}
