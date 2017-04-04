use atom::{Atom, Dimension};
use prefix::Prefix;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug)]
pub enum SimpleUnit {
    Atom(Box<Atom>),
    PrefixedAtom(Prefix, Box<Atom>),
}

impl SimpleUnit {
    pub fn composition(&self) -> BTreeMap<Dimension, i32> {
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();

        match *self {
            SimpleUnit::Atom(ref atom) => { map.insert(atom.dim(), 1); },
            SimpleUnit::PrefixedAtom(ref _prefix, ref atom) => {
                map.insert(atom.dim(), 1);
            },
        }

        map
    }

    pub fn is_special(&self) -> bool {
        match *self {
            SimpleUnit::Atom(ref atom) => atom.is_special(),
            SimpleUnit::PrefixedAtom(ref _prefix, ref atom) => atom.is_special()
        }
    }

    pub fn prefix_scalar(&self) -> f64 {
        match *self {
            SimpleUnit::Atom(_) => 1.0,
            SimpleUnit::PrefixedAtom(ref prefix, ref _atom) => prefix.scalar
        }
    }

    pub fn scalar(&self, magnitude: f64) -> f64 {
        match *self {
            SimpleUnit::Atom(ref atom) => atom.scale() * magnitude,
            SimpleUnit::PrefixedAtom(ref prefix, ref atom) => {
                prefix.scalar * atom.scale() * magnitude
            }
        }
    }

    pub fn scalar_default(&self) -> f64 {
        self.scalar(1.0)
    }

    pub fn magnitude(&self, scalar: f64) -> f64 {
        scalar
    }
}

impl fmt::Display for SimpleUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SimpleUnit::Atom(ref box_atom) => {
                let ref atom = *box_atom;
                write!(f, "{}", atom)
            },
            SimpleUnit::PrefixedAtom(ref prefix, ref box_atom) => {
                let ref atom = *box_atom;
                write!(f, "{}{}", prefix, atom)
            },
        }
    }
}

impl PartialEq for SimpleUnit {
    fn eq(&self, rhs: &Self) -> bool {
        match *self {
            SimpleUnit::Atom(ref box_atom) => {
                match *rhs {
                    SimpleUnit::Atom(ref box_rhs) => box_atom == box_rhs,
                    SimpleUnit::PrefixedAtom(ref _prefix, ref _box_rhs) => false
                }
            },
            SimpleUnit::PrefixedAtom(ref prefix, ref box_atom) => {
                match *rhs {
                    SimpleUnit::Atom(ref _box_rhs) => false,
                    SimpleUnit::PrefixedAtom(ref rhs_prefix, ref box_rhs) => {
                        prefix == rhs_prefix && box_atom == box_rhs
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleUnit;
    use atom::base::Meter;
    use parser::parse_SimpleUnit;
    use prefix::PREFIXES;

    #[test]
    fn validate_simple_unit() {
        let su_atom = SimpleUnit::Atom(Box::new(Meter));
        let su_pre_atom = SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), Box::new(Meter));

        assert_eq!(&parse_SimpleUnit("m").unwrap(), &su_atom);
        assert_eq!(&parse_SimpleUnit("km").unwrap(), &su_pre_atom);
        assert_eq!(&parse_SimpleUnit("M").unwrap(), &su_atom);
        assert_eq!(&parse_SimpleUnit("kM").unwrap(), &su_pre_atom);
        assert_eq!(&parse_SimpleUnit("Km").unwrap(), &su_pre_atom);
        assert_eq!(&parse_SimpleUnit("KM").unwrap(), &su_pre_atom);
    }

    #[test]
    fn validate_prefix_scalar() {
        let su_atom = SimpleUnit::Atom(Box::new(Meter));
        let su_pre_atom = SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), Box::new(Meter));

        assert_eq!(su_atom.prefix_scalar(), 1.0);
        assert_eq!(su_pre_atom.prefix_scalar(), 1000.0);
    }
}
