use super::{Error, Visit};
use crate::parser::terms::term_parser::Rule as TermRule;
use pest::iterators::Pair;

pub(super) type Factor = u32;

impl Visit<TermRule> for Factor {
    fn visit(pair: Pair<'_, TermRule>) -> Result<Self, Error> {
        pair.as_span().as_str().parse::<u32>().map_err(Error::from)
    }
}
