pub mod atom_collection;
pub mod base_units;
pub mod derived_units;
pub mod error;
pub mod parse;
pub mod parsers;
pub mod prefix_collection;
pub mod prefixes;
pub mod symbol_collection;
pub mod symbol_key;
pub mod tokens;
pub mod ucum_unit;
pub mod units;

#[cfg(test)]
mod test_helper;

use self::atom_collection::AtomCollection;
pub use self::{
    error::Error,
    parse::{Parse, ParseResult},
    prefix_collection::PrefixCollection,
    symbol_key::SymbolKey,
    ucum_unit::UcumUnit,
};

pub fn parse(input: &str) -> Result<crate::tokens::MainTerm<'_>, Error> {
    parse_general(input, &prefixes::PREFIXES, &units::UNITS)
}

pub fn parse_general<'input>(
    input: &'input str,
    prefixes: &'static PrefixCollection,
    atoms: &'static AtomCollection,
) -> Result<crate::tokens::MainTerm<'input>, Error> {
    match crate::parsers::main_term::Parser::parse(input, prefixes, atoms) {
        Ok((mt, tail)) => {
            if !tail.is_empty() {
                let matching_index = input.find(tail).unwrap();
                let matching = &input[..matching_index];

                let e = Error::PartialMatch {
                    matching: matching.to_string(),
                    remaining: tail.to_string(),
                };

                return Err(e);
            }

            Ok(mt)
        }
        Err(_) => Err(Error::Unparsable(input.to_string())),
    }
}
