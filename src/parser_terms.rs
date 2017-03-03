use atom::{Atom, Dimension};
use prefix::Prefix;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum UnitSign {
    Positive,
    Negative
}

impl fmt::Display for UnitSign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnitSign::Positive => write!(f, ""),
            UnitSign::Negative => write!(f, "-"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Factor(pub u32);

#[derive(Clone, Debug, PartialEq)]
pub struct Exponent(pub UnitSign, pub u32);

impl Exponent {
    pub fn as_i32(&self) -> i32 {
        match self.0 {
            UnitSign::Positive => { self.1 as i32 },
            UnitSign::Negative => { !self.1 as i32 },
        }
    }
}

impl fmt::Display for Exponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SimpleUnit {
    Atom(Atom),
    PrefixedAtom(Prefix, Atom),
}

impl SimpleUnit {
    pub fn composition(&self) -> BTreeMap<Dimension, i32> {
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();

        match *self {
            SimpleUnit::Atom(ref atom) => {
                map.insert(atom.dim.clone(), 1);
            },
            // TODO: Fix!
            _ => { map.insert(Dimension::Length, 0); }
        }

        map
    }

    pub fn is_special(&self) -> bool {
        match *self {
            SimpleUnit::Atom(ref atom) => {
                atom.special
            },
            SimpleUnit::PrefixedAtom(ref _prefix, ref atom) => {
                atom.special
            }
        }
    }
}

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

#[derive(Clone, Debug, PartialEq)]
pub struct Annotation<'a>(pub &'a str);

#[cfg(test)]
mod tests {
    use super::*;
    use atom::*;
    use component::Component;
    use parser::*;
    use prefix::*;
    use term::Term;

    #[test]
    fn validate_exponent() {
        assert_eq!(parse_Exponent("123").unwrap(), Exponent(UnitSign::Positive, 123));
        assert_eq!(parse_Exponent("-123").unwrap(), Exponent(UnitSign::Negative, 123));
    }

    #[test]
    fn validate_prefix_symbol() {
        assert_eq!(parse_PrefixSymbol("k").unwrap(), PREFIXES[7]);
        assert_eq!(parse_PrefixSymbol("K").unwrap(), PREFIXES[7]);
    }

    #[test]
    fn validate_atom_symbol() {
        assert_eq!(parse_AtomSymbol("m").unwrap(), ATOMS[0]);
        assert_eq!(parse_AtomSymbol("M").unwrap(), ATOMS[0]);
    }

    #[test]
    fn validate_simple_unit() {
        let su_atom = SimpleUnit::Atom(ATOMS[0].clone());
        let su_pre_atom = SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), ATOMS[0].clone());

        assert_eq!(&parse_SimpleUnit("m").unwrap(), &su_atom);
        assert_eq!(&parse_SimpleUnit("km").unwrap(), &su_pre_atom);
        assert_eq!(&parse_SimpleUnit("M").unwrap(), &su_atom);
        assert_eq!(&parse_SimpleUnit("kM").unwrap(), &su_pre_atom);
        assert_eq!(&parse_SimpleUnit("Km").unwrap(), &su_pre_atom);
        assert_eq!(&parse_SimpleUnit("KM").unwrap(), &su_pre_atom);
    }

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

    #[test]
    fn validate_annotation() {
        assert_eq!(parse_Annotation("{things123}").unwrap(), Annotation("things123"));
    }

    #[test]
    fn validate_component_with_annotations() {
        let simple_unit = SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), ATOMS[0].clone());
        let negative_exp = Exponent(UnitSign::Negative, 10);
        let annotatable = Annotatable::UnitWithPower(simple_unit, negative_exp);
        let annotation = Annotation("%vol");

        assert_eq!(
            parse_Component("km-10{%vol}").unwrap(),
            Component::AnnotatedAnnotatable(annotatable.clone(), annotation.clone())
            );

        assert_eq!(
            parse_Component("km-10").unwrap(),
            Component::Annotatable(annotatable.clone())
            );

        let annotation = Annotation("wet'tis.");

        assert_eq!(
            parse_Component("{wet'tis.}").unwrap(),
            Component::Annotation(annotation.clone())
            );
    }

    #[test]
    fn validate_component_with_factor() {
        assert_eq!(
            parse_Component("123").unwrap(),
            Component::Factor(Factor(123))
            );
    }

    #[test]
    fn validate_component_with_term() {
        assert_eq!(
            parse_Component("(123)").unwrap(),
            Component::Term(Box::new(Term::Basic(Component::Factor(Factor(123)))))
            );
    }

    #[test]
    fn validate_term_with_dot() {
        assert_eq!(
            parse_Term("g.m").unwrap(),
            Term::DotCombined(
                Component::Annotatable(
                    Annotatable::Unit(
                        SimpleUnit::Atom(ATOMS[2].clone())
                    )
                ),
                Box::new(
                    Term::Basic(
                        Component::Annotatable(
                            Annotatable::Unit(
                                SimpleUnit::Atom(ATOMS[0].clone())
                            )
                        )
                    )
                )
            )
        );
    }

    #[test]
    fn validate_term_with_slash() {
        assert_eq!(
            parse_Term("kg/s").unwrap(),
            Term::SlashCombined(
                Component::Annotatable(
                    Annotatable::Unit(
                        SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), ATOMS[2].clone())
                    )
                ),
                Box::new(
                    Term::Basic(
                        Component::Annotatable(
                            Annotatable::Unit(
                                SimpleUnit::Atom(ATOMS[1].clone())
                            )
                        )
                    )
                )
            )
        );
    }

    #[test]
    fn validate_term_basic() {
        assert_eq!(
            parse_Term("g").unwrap(),
            Term::Basic(
                Component::Annotatable(
                    Annotatable::Unit(
                        SimpleUnit::Atom(ATOMS[2].clone())
                    )
                )
            )
        );
    }

    #[test]
    fn validate_main_term_with_slash() {
        assert_eq!(
            parse_MainTerm("/g{tot'nit}").unwrap(),
            Term::Basic(
                Component::AnnotatedAnnotatable(
                    Annotatable::Unit(
                        SimpleUnit::Atom(ATOMS[2].clone())
                    ),
                    Annotation("tot'nit")
                )
            )
        );
    }
}
