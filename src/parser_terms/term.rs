use parser_terms::Component;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fmt;
use unit::Dimension;

/// The almost-root level node in the AST, it represents how a Term can be
/// combined with other Terms: sometimes Terms are combined multiplicatively
/// (DotCombined, ex "[pi].m2"), sometimes on a Term-per-Term basis
/// (SlashCombined, "m2/s"), and sometimes they stand by themselves (Basic,
/// "m2").
///
#[derive(Debug, PartialEq, Eq)]
pub enum Term {
    DotCombined(Component, Box<Term>),
    SlashCombined(Component, Box<Term>),
    Basic(Component),
}

impl Term {
    pub fn composition(&self) -> BTreeMap<Dimension, i32> {
        match *self {
            Term::Basic(ref component) => component.composition(),
            Term::SlashCombined(ref component, ref box_term) => {
                let mut component_composition = component.composition();
                let ref term = *box_term;
                let term_composition = term.composition();

                for (term_dim_name, term_value) in term_composition {
                    match component_composition.entry(term_dim_name) {
                        Entry::Vacant(e) => {
                            e.insert(-term_value);
                        }
                        Entry::Occupied(mut e) => {
                            *e.get_mut() -= term_value;
                        }
                    }
                }

                component_composition
            }
            Term::DotCombined(ref component, ref box_term) => {
                let mut component_composition = component.composition();
                let ref term = *box_term;
                let term_composition = term.composition();

                for (term_dim_name, term_value) in term_composition {
                    match component_composition.entry(term_dim_name) {
                        Entry::Vacant(e) => {
                            e.insert(term_value);
                        }
                        Entry::Occupied(mut e) => {
                            *e.get_mut() += term_value;
                        }
                    }
                }

                component_composition
            }
        }
    }

    pub fn is_compatible_with(&self, other: &Term) -> bool {
        let me = self.composition();
        let other_comp = other.composition();

        me == other_comp
    }

    pub fn is_special(&self) -> bool {
        match *self {
            Term::Basic(ref component) => component.is_special(),
            Term::SlashCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.is_special() || term.is_special()
            }
            Term::DotCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.is_special() || term.is_special()
            }
        }
    }

    pub fn scalar(&self) -> f64 {
        match *self {
            Term::Basic(ref component) => component.scalar(),
            Term::DotCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.scalar() * term.scalar()
            }
            Term::SlashCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.scalar() / term.scalar()
            }
        }
    }

    pub fn magnitude(&self) -> f64 {
        match *self {
            Term::Basic(ref component) => component.magnitude(),
            Term::DotCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.magnitude() * term.magnitude()
            }
            Term::SlashCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.magnitude() / term.magnitude()
            }
        }
    }

    pub fn calculate_scalar(&self, magnitude: f64) -> f64 {
        match *self {
            Term::Basic(ref component) => component.calculate_scalar(magnitude),
            Term::DotCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.calculate_scalar(magnitude) * term.calculate_scalar(magnitude)
            }
            Term::SlashCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.calculate_scalar(magnitude) * term.calculate_scalar(magnitude)
            }
        }
    }

    pub fn calculate_magnitude(&self, scalar: f64) -> f64 {
        match *self {
            Term::Basic(ref component) => component.calculate_magnitude(scalar),
            Term::DotCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.calculate_magnitude(scalar) * term.calculate_magnitude(scalar)
            }
            Term::SlashCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.calculate_magnitude(scalar) * term.calculate_magnitude(scalar)
            }
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Term::DotCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                write!(f, "{}.{}", &component, &term)
            }
            Term::SlashCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                write!(f, "{}/{}", component, term)
            }
            Term::Basic(ref component) => write!(f, "{}", component),
        }
    }
}

#[cfg(test)]
mod tests {
    use parser::*;
    use parser_terms::*;
    use parser_terms::UnitSign::*;
    use std::collections::BTreeMap;
    use unit::Dimension;
    use unit::base::{Gram, Meter, Second};
    use unit::prefix::Kilo;

    #[test]
    fn validate_parsing_term_with_dot() {
        assert_eq!(
            parse_Term("g.m").unwrap(),
            Term::DotCombined(
                Component::Annotatable(Annotatable::Unit(SimpleUnit::Atom(Box::new(Gram)))),
                Box::new(Term::Basic(Component::Annotatable(
                    Annotatable::Unit(SimpleUnit::Atom(Box::new(Meter))),
                ))),
            )
        );
    }

    #[test]
    fn validate_parsing_term_with_slash() {
        assert_eq!(
            parse_Term("kg/s").unwrap(),
            Term::SlashCombined(
                Component::Annotatable(Annotatable::Unit(
                    SimpleUnit::PrefixedAtom(Box::new(Kilo), Box::new(Gram)),
                )),
                Box::new(Term::Basic(Component::Annotatable(
                    Annotatable::Unit(SimpleUnit::Atom(Box::new(Second))),
                ))),
            )
        );
    }

    #[test]
    fn validate_parsing_term_basic_with_prefix_and_exponent() {
        assert_eq!(
            parse_Term("kg2").unwrap(),
            Term::Basic(Component::Annotatable(Annotatable::UnitWithPower(
                SimpleUnit::PrefixedAtom(Box::new(Kilo), Box::new(Gram)),
                Exponent(Positive, 2),
            )))
        );
    }

    #[test]
    fn validate_parsing_term_basic_with_exponent() {
        assert_eq!(
            parse_Term("g2").unwrap(),
            Term::Basic(Component::Annotatable(Annotatable::UnitWithPower(
                SimpleUnit::Atom(Box::new(Gram)),
                Exponent(Positive, 2),
            )))
        );
    }

    #[test]
    fn validate_parsing_term_basic() {
        assert_eq!(
            parse_Term("g").unwrap(),
            Term::Basic(Component::Annotatable(
                Annotatable::Unit(SimpleUnit::Atom(Box::new(Gram))),
            ))
        );
    }

    #[test]
    fn validate_composition() {
        let term = parse_Term("m").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 1);
        assert_eq!(term.composition(), map);

        let term = parse_Term("m2").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 2);
        assert_eq!(term.composition(), map);

        let term = parse_Term("m2/s").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 2);
        map.insert(Dimension::Time, -1);
        assert_eq!(term.composition(), map);

        let term = parse_Term("s/m2").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, -2);
        map.insert(Dimension::Time, 1);
        assert_eq!(term.composition(), map);
    }

    #[test]
    fn validate_composition_with_prefix() {
        let term = parse_Term("km").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 1);
        assert_eq!(term.composition(), map);

        let term = parse_Term("km2").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 2);
        assert_eq!(term.composition(), map);

        let term = parse_Term("km2/s").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 2);
        map.insert(Dimension::Time, -1);
        assert_eq!(term.composition(), map);

        let term = parse_Term("s/km2").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, -2);
        map.insert(Dimension::Time, 1);
        assert_eq!(term.composition(), map);
    }

    #[test]
    fn validate_composition_with_dimless() {
        let term = parse_Term("[pi].m2").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 2);
        assert_eq!(term.composition(), map);
    }

    #[test]
    fn validate_is_compatible_with() {
        let me = parse_Term("m2").unwrap();
        let other = parse_Term("m3/m").unwrap();
        assert!(me.is_compatible_with(&other))
    }

    #[test]
    fn validate_is_compatible_with_with_prefix() {
        let me = parse_Term("m").unwrap();
        let other = parse_Term("km").unwrap();
        assert!(me.is_compatible_with(&other))
    }
}
