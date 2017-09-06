use dimension::Dimension;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fmt;
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

impl fmt::Display for Composition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut expressions = Vec::<String>::new();

        for (key, value) in self.0.iter() {
            if *key == Dimension::None { continue; };

            expressions.push(format!("{}{}", key, value));
        }

        write!(f, "{}", expressions.join("."))
    }
}

impl IntoIterator for Composition {
    type Item = (Dimension, i32);
    type IntoIter = ::std::collections::btree_map::IntoIter<Dimension, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Composition;
    use dimension::Dimension;
    use std::collections::BTreeMap;

    #[test]
    fn validate_new() {
        let composition = Composition::new();
        let dimension: BTreeMap<Dimension, i32> = BTreeMap::new();

        assert_eq!(composition.0, dimension)
    }

    #[test]
    fn validate_insert() {
        let mut composition = Composition::new();
        composition.insert(Dimension::Mass, 3);
        assert_eq!(composition.to_string().as_str(), "M3");

        composition.insert(Dimension::Mass, 3);
        assert_eq!(composition.to_string().as_str(), "M6");

        composition.insert(Dimension::Mass, -6);
        // TODO: Is this right?
        assert_eq!(composition.to_string().as_str(), "M0");

        let mut composition = Composition::new();
        composition.insert(Dimension::Mass, -1);
        composition.insert(Dimension::Temperature, -2);
        composition.insert(Dimension::ElectricCharge, -3);
        composition.insert(Dimension::Time, -4);
        composition.insert(Dimension::Length, -5);
        composition.insert(Dimension::PlaneAngle, -6);
        composition.insert(Dimension::None, 0);
        composition.insert(Dimension::LuminousIntensity, -7);
        assert_eq!(composition.to_string().as_str(), "Q-3.L-5.F-7.M-1.A-6.C-2.T-4");
    }

    #[test]
    fn validate_new_from_values() {
        let composition = Composition::new_from_values(Dimension::Time, -6);
        assert_eq!(composition.to_string().as_str(), "T-6");

        let mut composition = Composition::new_from_values(Dimension::Time, -6);
        composition.insert(Dimension::Mass, 3);
        assert_eq!(composition.to_string().as_str(), "M3.T-6");
    }
}
