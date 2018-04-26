#[cfg(test)]
#[macro_use]
extern crate approx;

extern crate failure;
#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate lazy_static;

// Only include macros for testing
#[cfg(test)]
#[macro_use(parses_to, consumes_to, fails_with)]
extern crate pest;

#[cfg(not(test))]
extern crate pest;

#[macro_use]
extern crate pest_derive;

pub mod interpreter;
pub mod parser;

mod atom;
mod classification;
mod definition;
mod error;
mod prefix;
mod property;
mod symbol_interpreter;
mod symbol_parser;
mod term;
mod ucum_symbol;

pub use atom::Atom;
pub use classification::Classification;
pub use error::Error;
pub use parser::UnitParser;
pub use prefix::Prefix;
pub use term::Term;
pub use ucum_symbol::UcumSymbol;
pub use symbol_interpreter::SymbolInterpreter;
pub use symbol_parser::SymbolParser;

use interpreter::Interpreter;
use pest::Parser;
use parser::Rule;

pub fn parse(expression: &str) -> Result<Vec<Term>, Error> {
    match UnitParser::parse(Rule::main_term, expression) {
        Ok(pairs) => {
            let mut interpreter = Interpreter;
            Ok(interpreter.interpret(pairs)?)
        }
        Err(_) => Err(Error::UnknownUnitString(expression.to_string())),
    }
}
