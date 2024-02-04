use crate::{symbol_collection::SymbolCollection, tokens::PrefixSymbol, SymbolKey};

#[derive(Debug)]
pub struct PrefixCollection {
    pub(crate) inner: [(SymbolKey, PrefixSymbol); 24],
}

impl SymbolCollection<PrefixSymbol> for PrefixCollection {
    fn find_match(&'static self, input: &str) -> Option<(&'static PrefixSymbol, usize)> {
        self.inner.iter().find_map(|(key, valid_symbol)| {
            if input.starts_with(&**key) {
                Some((valid_symbol, key.len()))
            } else {
                None
            }
        })
    }
}
