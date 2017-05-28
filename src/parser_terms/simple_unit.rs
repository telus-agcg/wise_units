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
            SimpleUnit::Atom(ref box_unit) => {
                let unit_dim = box_unit.dim();

                if unit_dim != Dimension::None {
                    map.insert(box_unit.dim(), 1);
                }
            },
            SimpleUnit::PrefixedAtom(ref _box_prefix, ref box_unit) => {
                let unit_dim = box_unit.dim();

                if unit_dim != Dimension::None {
                    map.insert(box_unit.dim(), 1);
                }
            },
        }

        map
    }

    pub fn is_special(&self) -> bool {
        match *self {
            SimpleUnit::Atom(ref box_unit) => box_unit.is_special(),
            SimpleUnit::PrefixedAtom(ref _box_prefix, ref box_unit) => box_unit.is_special()
        }
    }

    pub fn scalar(&self) -> f64 {
        match *self {
            SimpleUnit::Atom(ref box_unit) => box_unit.definition().scalar(),
            SimpleUnit::PrefixedAtom(ref box_prefix, ref box_unit) => {
                box_prefix.definition().scalar() * box_unit.definition().scalar()
            }
        }
    }

    pub fn calculate_scalar(&self, input: f64) -> f64 {
        match *self {
            SimpleUnit::Atom(ref box_unit) => box_unit.definition().calculate_scalar(input),
            SimpleUnit::PrefixedAtom(ref box_prefix, ref box_unit) => {
                box_prefix.definition().scalar() * box_unit.definition().calculate_scalar(input)
            }
        }
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
    fn validate_parsing_simple_unit() {
        let su_atom = SimpleUnit::Atom(Box::new(Meter));
        let su_pre_atom = SimpleUnit::PrefixedAtom(Box::new(Kilo), Box::new(Meter));

        assert_eq!(&parse_SimpleUnit("m").unwrap(), &su_atom);
        assert_eq!(&parse_SimpleUnit("km").unwrap(), &su_pre_atom);
        assert_eq!(&parse_SimpleUnit("M").unwrap(), &su_atom);
        assert_eq!(&parse_SimpleUnit("kM").unwrap(), &su_pre_atom);
        assert_eq!(&parse_SimpleUnit("Km").unwrap(), &su_pre_atom);
        assert_eq!(&parse_SimpleUnit("KM").unwrap(), &su_pre_atom);
    }
}
