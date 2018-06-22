use super::Dimension;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fmt;
use std::iter::IntoIterator;

/// A `Composition` represents the makeup of a `Unit`'s dimensions; only dimensions and each
/// `Unit`s `Term`'s exponent. For example, "m" would effectively have the composition string of
/// "L"; "m2" would be "L2"; "1/m2" would be "L-2". This continues on when a Unit has multiple
/// `Term`s (ex. "mL/(kg.d)").
///
#[derive(Debug, PartialEq)]
pub struct Composition(BTreeMap<Dimension, i32>);

impl Composition {
    pub fn new(dimension: Dimension, exponent: i32) -> Self {
        let mut c = Self::default();
        c.insert(dimension, exponent);

        c
    }

    /// Convenience wrapper for updating the internal `BTreeMap` that contains the data.
    ///
    pub fn insert(&mut self, dimension: Dimension, exponent: i32) {
        match self.0.entry(dimension) {
            Entry::Vacant(entry) => {
                entry.insert(exponent);
            }
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += exponent;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for Composition {
    fn default() -> Self {
        let map: BTreeMap<Dimension, i32> = BTreeMap::new();

        Composition(map)
    }
}

impl fmt::Display for Composition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut expressions = Vec::<String>::new();

        for (key, value) in &self.0 {
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
    use super::super::Dimension;
    use super::Composition;
    use std::collections::BTreeMap;

    #[test]
    fn validate_default() {
        let composition = Composition::default();
        let dimension: BTreeMap<Dimension, i32> = BTreeMap::new();

        assert_eq!(composition.0, dimension)
    }

    #[test]
    fn validate_insert() {
        let mut composition = Composition::default();
        composition.insert(Dimension::Mass, 3);
        assert_eq!(composition.to_string().as_str(), "M3");

        composition.insert(Dimension::Mass, 3);
        assert_eq!(composition.to_string().as_str(), "M6");

        composition.insert(Dimension::Mass, -6);
        // TODO: Is this right?
        assert_eq!(composition.to_string().as_str(), "M0");

        let mut composition = Composition::default();
        composition.insert(Dimension::Mass, -1);
        composition.insert(Dimension::Temperature, -2);
        composition.insert(Dimension::ElectricCharge, -3);
        composition.insert(Dimension::Time, -4);
        composition.insert(Dimension::Length, -5);
        composition.insert(Dimension::PlaneAngle, -6);
        composition.insert(Dimension::LuminousIntensity, -7);
        assert_eq!(
            composition.to_string().as_str(),
            "Q-3.L-5.F-7.M-1.A-6.C-2.T-4"
        );
    }

    #[test]
    fn validate_new() {
        let composition = Composition::new(Dimension::Time, -6);
        assert_eq!(composition.to_string().as_str(), "T-6");

        let mut composition = Composition::new(Dimension::Time, -6);
        composition.insert(Dimension::Mass, 3);
        assert_eq!(composition.to_string().as_str(), "M3.T-6");
    }
}
