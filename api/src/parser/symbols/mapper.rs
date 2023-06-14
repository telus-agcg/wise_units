use crate::parser::{
    symbols::{symbol_parser::Rule, Symbol},
    Error, Visit,
};
use pest::iterators::Pair;

pub(in super::super) fn map(pair: Pair<'_, Rule>) -> Result<Symbol, Error> {
    if pair.as_rule() == Rule::symbol {
        Ok(Symbol::visit(pair)?)
    } else {
        Err(Error::BadFragment {
            fragment: pair.as_str().to_string(),
            position: pair.as_span().start(),
        })
    }
}
