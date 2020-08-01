use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/symbols/symbol.pest"]
pub(crate) struct SymbolParser;
