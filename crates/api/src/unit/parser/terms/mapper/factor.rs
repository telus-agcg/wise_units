use pest::iterators::Pair;

use crate::{
    term,
    unit::parser::{terms::term_parser::Rule as TermRule, Error, Visit},
};

impl Visit<'_, TermRule> for term::Factor {
    fn visit(pair: Pair<'_, TermRule>) -> Result<Self, Error> {
        pair.as_span().as_str().parse::<Self>().map_err(Error::from)
    }
}
