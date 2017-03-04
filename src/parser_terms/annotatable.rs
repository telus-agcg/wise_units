use atom::Dimension;
use parser_terms::{Exponent, SimpleUnit};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
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
                    SimpleUnit::Atom(ref atom) => {
                        let exp: i32 = exponent.as_i32();
                        map.insert(atom.dim.clone(), exp);
                    },
                    // TODO: Fix!
                    _ => { map.insert(Dimension::Length, 0); }
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
}

#[cfg(test)]
mod tests {
    use super::Annotatable;
    use atom::ATOMS;
    use parser::parse_Annotatable;
    use parser_terms::{Exponent, SimpleUnit, UnitSign};
    use prefix::PREFIXES;

    #[test]
    fn validate_annotatable() {
        let su_pre_atom = SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), ATOMS[0].clone());
        let ann = Annotatable::Unit(su_pre_atom.clone());

        let ann_with_pos_power = Annotatable::UnitWithPower(
            su_pre_atom.clone(),
            Exponent(UnitSign::Positive, 10)
            );

        let ann_with_neg_power = Annotatable::UnitWithPower(
            su_pre_atom.clone(),
            Exponent(UnitSign::Negative, 10)
            );
        assert_eq!(&parse_Annotatable("km").unwrap(), &ann);
        assert_eq!(&parse_Annotatable("km10").unwrap(), &ann_with_pos_power);
        assert_eq!(&parse_Annotatable("km-10").unwrap(), &ann_with_neg_power);
    }
}
