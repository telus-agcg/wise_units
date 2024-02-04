use crate::{symbol_collection::SymbolCollection, tokens::AtomSymbol, SymbolKey};

pub struct AtomCollection {
    pub(crate) inner: [(SymbolKey, AtomSymbol); 290],
}

// TODO: Not sure if this should return a reference to an AtomSymbol or value.
impl SymbolCollection<AtomSymbol> for AtomCollection {
    fn find_match(&'static self, input: &str) -> Option<(&'static AtomSymbol, usize)> {
        self.inner
            .iter()
            // .filter(|(k, _)| k.len() <= input.len())
            .find_map(|(key, valid_symbol)| {
                if input.starts_with(&**key) {
                    Some((valid_symbol, key.len()))
                } else {
                    None
                }
            })
    }
}
