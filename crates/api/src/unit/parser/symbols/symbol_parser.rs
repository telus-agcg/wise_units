#![allow(clippy::upper_case_acronyms)]

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "unit/parser/symbols/symbol.pest"]
pub(crate) struct SymbolParser;
