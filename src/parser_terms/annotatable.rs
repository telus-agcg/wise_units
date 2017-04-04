use parser_terms::{Exponent, SimpleUnit};
use std::collections::BTreeMap;
use std::fmt;
use unit::Dimension;

#[derive(Debug, PartialEq)]
pub enum Annotatable {
    Unit(SimpleUnit),
    UnitWithPower(SimpleUnit, Exponent),
}

impl Annotatable {
    pub fn composition(&self) -> BTreeMap<Dimension, i32> {
        match *self {
            Annotatable::Unit(ref simple_unit) => simple_unit.composition(),
            Annotatable::UnitWithPower(ref simple_unit, ref exponent) => {
                let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();

                match *simple_unit {
                    SimpleUnit::Atom(ref box_unit) => {
                        let exp: i32 = exponent.as_i32();
                        let ref unit = **box_unit;
                        let unit_dim = unit.dim();

                        if unit_dim != Dimension::None {
                            map.insert(unit_dim, exp);
                        }
                    },
                    SimpleUnit::PrefixedAtom(ref _prefix, ref box_unit) => {
                        let exp: i32 = exponent.as_i32();
                        let ref unit = *box_unit;
                        let unit_dim = unit.dim();

                        if unit_dim != Dimension::None {
                            map.insert(unit_dim, exp);
                        }
                    },
                }

                map
            }
        }
    }

    pub fn is_special(&self) -> bool {
        match *self {
            Annotatable::Unit(ref simple_unit) => {
                simple_unit.is_special()
            },
            Annotatable::UnitWithPower(ref simple_unit, ref _exponent) => {
                simple_unit.is_special()
            }
        }
    }

    pub fn prefix_scalar(&self) -> f64 {
        match *self {
            Annotatable::Unit(ref simple_unit) => {
                simple_unit.prefix_scalar()
            },
            Annotatable::UnitWithPower(ref simple_unit, ref _exponent) => {
                simple_unit.prefix_scalar()
            }
        }
    }

    pub fn scalar(&self, magnitude: f64) -> f64 {
        match *self {
            Annotatable::Unit(ref simple_unit) => {
                simple_unit.scalar(magnitude)
            },
            Annotatable::UnitWithPower(ref simple_unit, ref _exponent) => {
                simple_unit.scalar(magnitude)
            }
        }
    }

    pub fn scalar_default(&self) -> f64 {
        self.scalar(1.0)
    }

    pub fn magnitude(&self, scalar: f64) -> f64 {
        match *self {
            Annotatable::Unit(ref simple_unit) => {
                simple_unit.magnitude(scalar)
            },
            Annotatable::UnitWithPower(ref simple_unit, ref _exponent) => {
                simple_unit.magnitude(scalar)
            }
        }
    }

    pub fn magnitude_default(&self) -> f64 {
        self.magnitude(1.0)
    }
}

impl fmt::Display for Annotatable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Annotatable::Unit(ref simple_unit) => { write!(f, "{}", simple_unit) },
            Annotatable::UnitWithPower(ref simple_unit, ref exponent) => {
                write!(f, "{}{}", simple_unit, exponent)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Annotatable;
    use unit::base::Meter;
    use dimension::Dimension;
    use parser::parse_Annotatable;
    use parser_terms::{Exponent, SimpleUnit, UnitSign};
    use prefix::PREFIXES;
    use std::collections::BTreeMap;

    #[test]
    fn validate_annotatable() {
        let ann = Annotatable::Unit(make_su_pre_unit());

        let ann_with_pos_power = Annotatable::UnitWithPower(
            make_su_pre_unit(),
            Exponent(UnitSign::Positive, 10)
            );

        let ann_with_neg_power = Annotatable::UnitWithPower(
            make_su_pre_unit(),
            Exponent(UnitSign::Negative, 10)
            );
        assert_eq!(&parse_Annotatable("km").unwrap(), &ann);
        assert_eq!(&parse_Annotatable("km10").unwrap(), &ann_with_pos_power);
        assert_eq!(&parse_Annotatable("km-10").unwrap(), &ann_with_neg_power);
    }

    #[test]
    fn validate_prefix_scalar() {
        let ann = Annotatable::Unit(make_su_pre_unit());

        let ann_with_pos_power = Annotatable::UnitWithPower(
            make_su_pre_unit(),
            Exponent(UnitSign::Positive, 10)
            );

        let ann_with_neg_power = Annotatable::UnitWithPower(
            make_su_pre_unit(),
            Exponent(UnitSign::Negative, 10)
            );
        assert_eq!(ann.prefix_scalar(), 1000.0);
        assert_eq!(ann_with_pos_power.prefix_scalar(), 1000.0);
        assert_eq!(ann_with_neg_power.prefix_scalar(), 1000.0);
    }

    #[test]
    fn validate_composition() {
        let annotatable = parse_Annotatable("m").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 1);
        assert_eq!(annotatable.composition(), map);

        let annotatable = parse_Annotatable("m2").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 2);
        assert_eq!(annotatable.composition(), map);
    }

    #[test]
    fn validate_composition_with_prefix() {
        let annotatable = parse_Annotatable("km").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 1);
        assert_eq!(annotatable.composition(), map);

        let annotatable = parse_Annotatable("km2").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 2);
        assert_eq!(annotatable.composition(), map);
    }

    #[test]
    fn validate_composition_dimless() {
        let annotatable = parse_Annotatable("[pi]").unwrap();
        let map: BTreeMap<Dimension, i32> = BTreeMap::new();
        assert_eq!(annotatable.composition(), map);
    }

    fn make_su_pre_unit() -> SimpleUnit {
        SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), Box::new(Meter))
    }
}
