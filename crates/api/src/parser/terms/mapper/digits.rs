use pest::iterators::Pair;

use crate::parser::{terms::term_parser::Rule as TermRule, Error, Visit};

pub(super) type Digits = i32;

impl Visit<TermRule> for Digits {
    fn visit(pair: Pair<'_, TermRule>) -> Result<Self, Error> {
        pair.as_span().as_str().parse::<Self>().map_err(Error::from)
    }
}
