use pest::iterators::Pair;

use crate::parser::{terms::term_parser::Rule as TermRule, Error, Visit};

pub(super) type Annotation = String;

impl Visit<TermRule> for Annotation {
    fn visit(pair: Pair<'_, TermRule>) -> Result<Self, Error> {
        Ok(pair.as_span().as_str().to_string())
    }
}
