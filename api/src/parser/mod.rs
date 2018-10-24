include!(concat!(env!("OUT_DIR"), "/classification.rs"));
include!(concat!(env!("OUT_DIR"), "/property.rs"));
include!(concat!(env!("OUT_DIR"), "/atom.rs"));

pub(self) mod symbols;

#[cfg(test)]
mod atom_test;
mod composable;
mod composition;
mod definition;
mod dimension;
mod error;
mod function_set;
mod is_compatible_with;
mod prefix;
mod term;
mod terms;
mod ucum_symbol;

pub use self::atom::Atom;
pub use self::classification::Classification;
pub use self::composable::Composable;
pub use self::composition::Composition;
pub use self::dimension::Dimension;
pub use self::error::Error;
pub use self::is_compatible_with::IsCompatibleWith;
pub use self::prefix::Prefix;
pub use self::property::Property;
pub use self::term::Term;
pub use self::ucum_symbol::UcumSymbol;

use self::terms::term_parser::Rule;
use self::terms::term_parser::TermParser;
use pest::Parser;

#[inline]
pub(crate) fn parse(expression: &str) -> Result<Vec<Term>, Error> {
    match TermParser::parse(Rule::main_term, expression) {
        Ok(pairs) => Ok(terms::mapper::map(pairs)?),
        Err(_) => Err(Error::UnknownUnitString(expression.to_string())),
    }
}
