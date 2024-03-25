use pest::iterators::Pair;

use crate::parser::{term, terms::term_parser::Rule as TermRule, Error, Visit};

impl Visit<TermRule> for term::Factor {
    fn visit(pair: Pair<'_, TermRule>) -> Result<Self, Error> {
        pair.as_span().as_str().parse::<Self>().map_err(Error::from)
    }
}
