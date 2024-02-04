use crate::{symbol_collection::SymbolCollection, tokens::AtomSymbol, SymbolKey};

#[allow(missing_copy_implementations)]
pub struct AtomCollection {
    pub(crate) inner: [(SymbolKey, AtomSymbol); 290],
}

// TODO: Not sure if this should return a reference to an AtomSymbol or value.
impl<'a> SymbolCollection<'a, AtomSymbol> for AtomCollection {
    fn find_match(&'a self, input: &str) -> Option<(&'a AtomSymbol, usize)> {
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
