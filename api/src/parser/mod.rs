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
mod prefix;
mod term;
mod terms;
mod ucum_symbol;

pub use self::atom::Atom;
pub use self::classification::Classification;
pub use self::composable::Composable;
pub use self::composition::Composition;
pub use self::definition::Definition;
pub use self::dimension::Dimension;
pub use self::error::Error;
pub use self::function_set::FunctionSet;
pub use self::prefix::Prefix;
pub use self::property::Property;
pub use self::symbols::symbol_parser::SymbolParser;
pub use self::term::Term;
pub use self::ucum_symbol::UcumSymbol;

use self::terms::term_parser::Rule;
use self::terms::term_parser::TermParser;
use pest::Parser;

pub fn parse(expression: &str) -> Result<Vec<Term>, Error> {
    match TermParser::parse(Rule::main_term, expression) {
        Ok(pairs) => Ok(terms::mapper::map(pairs)?),
        Err(_) => Err(Error::UnknownUnitString(expression.to_string())),
    }
}
