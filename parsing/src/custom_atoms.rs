use std::collections::HashMap;
use ucum_symbol::UcumSymbol;

#[derive(Default)]
pub struct CustomAtoms {
    // TODO make key a (str, str) to handle secondary_code lookups.
    inner: HashMap<&'static str, &'static UcumSymbol>
}

impl CustomAtoms {
    pub fn insert(&mut self, key: &'static str, value: &'static UcumSymbol) -> Option<Box<UcumSymbol>> {
        self.inner.insert(key, value)
    }

    pub fn get(&self, key: &'static str) -> Option<&'static UcumSymbol> {
        self.inner.get(key)
            .and_then(|k| {
                let inner_key = *k;
                inner_key.clone()
            })
    }
}
