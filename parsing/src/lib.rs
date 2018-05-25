// Turn on proc_macro if we're on nightly AND using the with_stdweb feature.
#![cfg_attr(all(any(target_arch = "wasm32", target_os = "emscripten"), feature = "with_stdweb"),
            feature(proc_macro))]

#[cfg(test)]
#[macro_use]
extern crate approx;

extern crate failure;
#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate log;

// Only include macros for testing
#[cfg(test)]
#[macro_use(consumes_to, fails_with, parses_to)]
extern crate pest;

#[cfg(not(test))]
extern crate pest;

#[macro_use]
extern crate pest_derive;

#[cfg(feature = "with_serde")]
extern crate serde;

#[cfg(feature = "with_serde")]
#[cfg_attr(feature = "with_serde", macro_use)]
extern crate serde_derive;

#[cfg(all(test, feature = "with_serde"))]
extern crate serde_json;

#[cfg(test)]
extern crate simple_logger;

#[macro_use]
mod macros;

mod atom;
mod classification;
mod composable;
mod composition;
mod definition;
mod dimension;
mod error;
mod function_set;
mod prefix;
mod property;
mod symbols;
mod term;
mod terms;
mod ucum_symbol;

pub use atom::Atom;
pub use classification::Classification;
pub use composable::Composable;
pub use composition::Composition;
pub use dimension::Dimension;
pub use error::Error;
pub use prefix::Prefix;
pub use symbols::symbol_parser::SymbolParser;
pub use term::Term;
pub use terms::term_parser::TermParser;
pub use ucum_symbol::UcumSymbol;

use pest::Parser;
use terms::term_parser::Rule;

pub fn parse(expression: &str) -> Result<Vec<Term>, Error> {
    match TermParser::parse(Rule::main_term, expression) {
        Ok(pairs) => Ok(terms::mapper::map(pairs)?),
        Err(_) => Err(Error::UnknownUnitString(expression.to_string())),
    }
}
