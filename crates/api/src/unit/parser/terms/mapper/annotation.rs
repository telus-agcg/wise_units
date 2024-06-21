use pest::iterators::Pair;

use crate::unit::parser::{terms::term_parser::Rule as TermRule, Error, Visit};

pub(super) type Annotation<'a> = &'a str;

impl<'a> Visit<'a, TermRule> for Annotation<'a> {
    fn visit(pair: Pair<'a, TermRule>) -> Result<Self, Error> {
        Ok(pair.as_span().as_str())
    }
}
