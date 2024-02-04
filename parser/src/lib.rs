pub mod atom_collection;
pub mod base_units;
pub mod derived_units;
pub mod parse;
pub mod parsers;
pub mod prefix_collection;
pub mod symbol_collection;
pub mod symbol_key;
pub mod tokens;
pub mod ucum_unit;

pub use self::{
    // error::Error,
    parse::{Parse, ParseResult},
    prefix_collection::PrefixCollection,
    symbol_key::SymbolKey,
    ucum_unit::UcumUnit,
};

// pub fn parse_general<'input, 'atoms, A, T: symbol_collection::SymbolCollection<'atoms, A>>(
//     input: &'input str,
//     prefixes: &'atoms PrefixCollection<'atoms>,
//     atoms: &'atoms T,
// ) -> Result<crate::tokens::MainTerm<'input, 'atoms, A>, Error> {
//     match crate::parsers::main_term::Parser::parse(input, prefixes, atoms) {
//         Ok((mt, tail)) => {
//             if !tail.is_empty() {
//                 let matching_index = input.find(tail).unwrap();
//                 let matching = &input[..matching_index];
//
//                 let e = Error::PartialMatch {
//                     matching: matching.to_string(),
//                     remaining: tail.to_string(),
//                 };
//
//                 return Err(e);
//             }
//
//             Ok(mt)
//         }
//         Err(_) => Err(Error::Unparsable(input.to_string())),
//     }
// }
