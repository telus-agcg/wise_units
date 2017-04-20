use std::collections::BTreeMap;
use std::fmt;
use unit::{Prefix, Unit, Dimension};

#[derive(Debug)]
pub enum SimpleUnit {
    Atom(Box<Unit>),
    PrefixedAtom(Box<Prefix>, Box<Unit>),
}

impl SimpleUnit {
    pub fn composition(&self) -> BTreeMap<Dimension, i32> {
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();

        match *self {
            SimpleUnit::Atom(ref unit) => {
                let unit_dim = unit.dim();

                if unit_dim != Dimension::None {
                    map.insert(unit.dim(), 1);
                }
            },
            SimpleUnit::PrefixedAtom(ref _prefix, ref unit) => {
                let unit_dim = unit.dim();

                if unit_dim != Dimension::None {
                    map.insert(unit.dim(), 1);
                }
            },
        }

        map
    }

    pub fn is_special(&self) -> bool {
        match *self {
            SimpleUnit::Atom(ref unit) => unit.is_special(),
            SimpleUnit::PrefixedAtom(ref _prefix, ref unit) => unit.is_special()
        }
    }

    pub fn prefix_scalar(&self) -> f64 {
        match *self {
            SimpleUnit::Atom(_) => 1.0,
            SimpleUnit::PrefixedAtom(ref prefix, ref _unit) => prefix.scalar()
        }
    }

    pub fn scalar(&self, magnitude: f64) -> f64 {
        match *self {
            SimpleUnit::Atom(ref unit) => unit.scale() * magnitude,
            SimpleUnit::PrefixedAtom(ref prefix, ref unit) => {
                prefix.scalar() * unit.scale() * magnitude
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
            SimpleUnit::Atom(ref box_unit) => {
                let ref unit = *box_unit;
                write!(f, "{}", unit)
            },
            SimpleUnit::PrefixedAtom(ref box_prefix, ref box_unit) => {
                let ref prefix = *box_prefix;
                let ref unit = *box_unit;
                write!(f, "{}{}", prefix, unit)
            },
        }
    }
}

impl PartialEq for SimpleUnit {
    fn eq(&self, rhs: &Self) -> bool {
        match *self {
            SimpleUnit::Atom(ref box_unit) => {
                match *rhs {
                    SimpleUnit::Atom(ref box_rhs) => box_unit == box_rhs,
                    SimpleUnit::PrefixedAtom(ref _box_prefix, ref _box_rhs) => false
                }
            },
            SimpleUnit::PrefixedAtom(ref box_prefix, ref box_unit) => {
                match *rhs {
                    SimpleUnit::Atom(ref _box_rhs) => false,
                    SimpleUnit::PrefixedAtom(ref box_rhs_prefix, ref box_rhs) => {
                        box_prefix == box_rhs_prefix && box_unit == box_rhs
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleUnit;
    use unit::base::Meter;
    use unit::prefix::Kilo;
    use parser::parse_SimpleUnit;

    #[test]
    fn validate_simple_unit() {
        let su_atom = SimpleUnit::Atom(Box::new(Meter));
        let su_pre_atom = SimpleUnit::PrefixedAtom(Box::new(Kilo), Box::new(Meter));

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
        let su_pre_atom = SimpleUnit::PrefixedAtom(Box::new(Kilo), Box::new(Meter));

        assert_eq!(su_atom.prefix_scalar(), 1.0);
        assert_eq!(su_pre_atom.prefix_scalar(), 1000.0);
    }
}
