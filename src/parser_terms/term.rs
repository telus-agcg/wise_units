use atom::Dimension;
use parser_terms::Component;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

pub enum Term<'a> {
    DotCombined(Component<'a>, Box<Term<'a>>),
    SlashCombined(Component<'a>, Box<Term<'a>>),
    Basic(Component<'a>),
}

impl<'a> Term<'a> {
    // pub fn factor(&self) -> u32 {
    //     match *self {
    //         Term::DotCombined(ref component, ref box_term) => {
    //             let component_factor = component.factor();
    //             let ref term = *box_term;
    //             let term_factor = term.factor();

    //             // Not sure if this is right...
    //             component_factor * term_factor
    //         }
    //         _ => 1
    //     }
    // }

    pub fn composition(&self) -> BTreeMap<Dimension, i32> {
        match *self {
            Term::Basic(ref component) => component.composition(),
            Term::SlashCombined(ref component, ref box_term) => {
                let mut component_composition = component.composition();
                let ref term = *box_term;
                let term_composition = term.composition();

                for (term_dim_name, term_value) in term_composition {
                    match component_composition.entry(term_dim_name) {
                        Entry::Vacant(e) => { e.insert(-term_value); },
                        Entry::Occupied(mut e) => { *e.get_mut() -= term_value; }
                    }
                }

                component_composition
            },
            Term::DotCombined(ref component, ref box_term) => {
                let mut component_composition = component.composition();
                let ref term = *box_term;
                let term_composition = term.composition();

                for (term_dim_name, term_value) in term_composition {
                    match component_composition.entry(term_dim_name) {
                        Entry::Vacant(e) => { e.insert(term_value); },
                        Entry::Occupied(mut e) => { *e.get_mut() += term_value; }
                    }
                }

                component_composition
            }
        }
    }

    fn composition_string(&self) -> String {
        let composition = self.composition();

        composition.into_iter()
            .map(|(k, v)| match v {
                1 => k.to_string(),
                _ => format!("{}{}", k, v),
            })
            .collect::<Vec<String>>()
            .as_slice()
            .join(".")
    }

    pub fn is_compatible_with<'b>(&self, other: &Term<'b>) -> bool {
        let me = self.composition();
        let other = other.composition();

        me == other
    }

    pub fn is_special(&self) -> bool {
        match *self {
            Term::Basic(ref component) => {
                component.is_special()
            },
            _ => false
        }
    }

    pub fn magnitude(&self, scalar: f64) -> f64 {
        match *self {
            Term::DotCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.magnitude(scalar) * term.magnitude(scalar)
            },
            Term::SlashCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.magnitude(scalar) * term.magnitude(scalar)
            },
            Term::Basic(ref component) => {
                component.magnitude(scalar)
            }
        }
    }

    pub fn magnitude_default(&self) -> f64 {
        self.magnitude(1.0)
    }

    pub fn scalar(&self, magnitude: f64) -> f64 {
        match *self {
            Term::DotCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.scalar(magnitude) * term.scalar(magnitude)
            },
            Term::SlashCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.scalar(magnitude) * term.scalar(magnitude)
            },
            Term::Basic(ref component) => {
                component.scalar(magnitude)
            }
        }
    }

    pub fn scalar_default(&self) -> f64 {
        self.scalar(1.0)
    }

    pub fn prefix_scalar(&self) -> f64 {
        match *self {
            Term::DotCombined(ref component, ref box_term) => {
                let comp_prefix = component.prefix_scalar();
                let ref term = *box_term;
                let term_prefix = term.prefix_scalar();

                comp_prefix * term_prefix
            },
            Term::SlashCombined(ref component, ref box_term) => {
                let comp_prefix = component.prefix_scalar();
                let ref term = *box_term;
                let term_prefix = term.prefix_scalar();

                comp_prefix / term_prefix
            },
            Term::Basic(ref component) => {
                component.prefix_scalar()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use atom::{ATOMS, Dimension};
    use parser::*;
    use parser_terms::*;
    use prefix::PREFIXES;
    use std::collections::BTreeMap;

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
    fn validate_composition_string() {
        let term = parse_Term("m").unwrap();
        assert_eq!(term.composition_string(), "L");

        let term = parse_Term("m2").unwrap();
        assert_eq!(term.composition_string(), "L2");

        let term = parse_Term("m2/s").unwrap();
        assert_eq!(term.composition_string(), "L2.T-1");

        let term = parse_Term("s/m2").unwrap();
        assert_eq!(term.composition_string(), "L-2.T");
    }

    #[test]
    fn validate_composition_string_with_prefix() {
        let term = parse_Term("km").unwrap();
        assert_eq!(term.composition_string(), "L");

        let term = parse_Term("km2").unwrap();
        assert_eq!(term.composition_string(), "L2");

        let term = parse_Term("km2/s").unwrap();
        assert_eq!(term.composition_string(), "L2.T-1");

        let term = parse_Term("s/km2").unwrap();
        assert_eq!(term.composition_string(), "L-2.T");
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
        println!("me: {:?}", me.composition_string());
        println!("other: {:?}", other.composition_string());
        assert!(me.is_compatible_with(&other))
    }

    #[test]
    fn validate_prefix_scalar_dot_combined() {
        let term = parse_Term("m2.s").unwrap();
        assert_eq!(term.prefix_scalar(), 1.0);

        let term = parse_Term("km.kg").unwrap();
        assert_eq!(term.prefix_scalar(), 1_000_000.0);
    }

    #[test]
    fn validate_prefix_scalar_slash_combined() {
        let term = parse_Term("m2/s").unwrap();
        assert_eq!(term.prefix_scalar(), 1.0);

        let term = parse_Term("km/kg").unwrap();
        assert_eq!(term.prefix_scalar(), 1.0);
    }
}
