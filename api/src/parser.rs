// Because long numbers are generated, there's no way (that I know of) to
// generate them using underscores (to make them pass the clippy lint).
#[cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::unreadable_literal,
        clippy::match_same_arms,
        clippy::too_many_lines,
        clippy::non_ascii_literal
    )
)]
pub mod atom;
pub mod classification;
#[allow(clippy::non_ascii_literal)]
pub mod property;

pub(self) mod symbols;

mod annotation_composition;
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

pub use self::{
    annotation_composition::AnnotationComposition, atom::Atom, classification::Classification,
    composable::Composable, composition::Composition, dimension::Dimension, error::Error,
    prefix::Prefix, property::Property, term::Term, ucum_symbol::UcumSymbol,
};

use self::terms::term_parser::{Rule, TermParser};
use pest::Parser;

#[inline]
pub(crate) fn parse(expression: &str) -> Result<Vec<Term>, Error> {
    match TermParser::parse(Rule::main_term, expression) {
        Ok(pairs) => Ok(terms::mapper::map(pairs)?),
        Err(_) => Err(Error::UnknownUnitString(expression.to_string())),
    }
}
