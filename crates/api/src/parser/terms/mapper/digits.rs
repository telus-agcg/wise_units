use super::{Error, Visit};
use crate::parser::{term::Exponent, terms::term_parser::Rule as TermRule};
use pest::iterators::Pair;

pub(super) type Digits = Exponent;

impl Visit<TermRule> for Digits {
    fn visit(pair: Pair<'_, TermRule>) -> Result<Self, Error> {
        pair.as_span().as_str().parse::<Self>().map_err(Error::from)
    }
}
