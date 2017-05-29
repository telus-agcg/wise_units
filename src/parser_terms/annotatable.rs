use parser_terms::{Exponent, SimpleUnit, SpecialUnit};
use std::collections::BTreeMap;
use std::fmt;
use unit::Dimension;

#[derive(Debug, PartialEq)]
pub enum Annotatable<'a> {
    Unit(SimpleUnit),
    UnitWithPower(SimpleUnit, Exponent),
    SpecialUnit(SpecialUnit<'a>),
}

impl<'a> Annotatable<'a> {
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
            },
            Annotatable::SpecialUnit(ref special_unit) => special_unit.composition(),
        }
    }

    pub fn is_special(&self) -> bool {
        match *self {
            Annotatable::Unit(ref simple_unit) => {
                simple_unit.is_special()
            },
            Annotatable::UnitWithPower(ref simple_unit, ref _exponent) => {
                simple_unit.is_special()
            },
            Annotatable::SpecialUnit(ref special_unit) => {
                special_unit.is_special()
            }
        }
    }

    pub fn scalar(&self) -> f64 {
        match *self {
            Annotatable::Unit(ref simple_unit) => {
                simple_unit.scalar()
            },
            Annotatable::UnitWithPower(ref simple_unit, ref exponent) => {
                let e = exponent.as_i32();
                simple_unit.scalar().powi(e)
            },
            Annotatable::SpecialUnit(ref special_unit) => special_unit.scalar(),
        }
    }

    pub fn magnitude(&self) -> f64 {
        match *self {
            Annotatable::Unit(ref simple_unit) => {
                simple_unit.magnitude()
            },
            Annotatable::UnitWithPower(ref simple_unit, ref exponent) => {
                let e = exponent.as_i32();
                simple_unit.magnitude().powi(e)
            },
            Annotatable::SpecialUnit(ref special_unit) => special_unit.magnitude(),
        }
    }

    pub fn calculate_scalar(&self, magnitude: f64) -> f64 {
        match *self {
            Annotatable::Unit(ref simple_unit) => simple_unit.calculate_scalar(magnitude),
            Annotatable::UnitWithPower(ref simple_unit, ref exponent) => {
                let e = exponent.as_i32();
                simple_unit.calculate_scalar(magnitude).powi(e)
            },
            Annotatable::SpecialUnit(ref special_unit) => special_unit.calculate_scalar(magnitude),
        }
    }

    pub fn calculate_magnitude(&self, scalar: f64) -> f64 {
        match *self {
            Annotatable::Unit(ref simple_unit) => simple_unit.calculate_magnitude(scalar),
            Annotatable::UnitWithPower(ref simple_unit, ref exponent) => {
                let e = exponent.as_i32();
                simple_unit.calculate_magnitude(scalar).powi(e)
            },
            Annotatable::SpecialUnit(ref special_unit) => special_unit.calculate_magnitude(scalar),
        }
    }
}

impl<'a> fmt::Display for Annotatable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Annotatable::Unit(ref simple_unit) => { write!(f, "{}", simple_unit) },
            Annotatable::UnitWithPower(ref simple_unit, ref exponent) => {
                write!(f, "{}{}", simple_unit, exponent)
            }
            Annotatable::SpecialUnit(ref special_unit) => { write!(f, "{}", special_unit) },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Annotatable;
    use unit::base::Meter;
    use unit::prefix::Kilo;
    use dimension::Dimension;
    use parser::parse_Annotatable;
    use parser_terms::{Exponent, SimpleUnit, UnitSign};
    use std::collections::BTreeMap;

    #[test]
    fn validate_parsing_annotatable() {
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
        SimpleUnit::PrefixedAtom(Box::new(Kilo), Box::new(Meter))
    }
}
