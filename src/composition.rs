use dimension::Dimension;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::iter::IntoIterator;

#[derive(Debug, PartialEq)]
pub struct Composition(BTreeMap<Dimension, i32>);

impl Composition {
    pub fn new() -> Self {
        let map: BTreeMap<Dimension, i32> = BTreeMap::new();

        Composition(map)
    }

    pub fn new_from_values(dimension: Dimension, exponent: i32) -> Self {
        let mut c = Composition::new();
        c.insert(dimension, exponent);

        c
    }

    pub fn new_unity() -> Self {
        Composition::new_from_values(Dimension::None, 0)
    }

    pub fn insert(&mut self, dimension: Dimension, exponent: i32) {
        match self.0.entry(dimension) {
            Entry::Vacant(entry) => {
                entry.insert(exponent);
            },
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += exponent;
            }
        }
    }
}

impl IntoIterator for Composition {
    type Item = (Dimension, i32);
    type IntoIter = ::std::collections::btree_map::IntoIter<Dimension, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
