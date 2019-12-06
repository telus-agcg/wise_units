#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownAtomSymbol(String),
    UnknownPrefixSymbol(String),
}
