use atom::Dimension;
use component::Component;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

#[derive(Clone, Debug, PartialEq)]
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
                        Entry::Vacant(e) => {
                            println!("term value: {}", term_value);
                            e.insert(-term_value);
                        },
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
        // println!("composition: {:?}", composition);

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
}

#[cfg(test)]
mod tests {
    use atom::Dimension;
    use parser::*;
    use std::collections::BTreeMap;

    #[test]
    fn validate_composition() {
        let term = parse_MainTerm("m").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 1);
        assert_eq!(term.composition(), map);

        let term = parse_MainTerm("m2").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 2);
        assert_eq!(term.composition(), map);

        let term = parse_MainTerm("m2/s").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 2);
        map.insert(Dimension::Time, -1);
        assert_eq!(term.composition(), map);

        let term = parse_MainTerm("s/m2").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, -2);
        map.insert(Dimension::Time, 1);
        assert_eq!(term.composition(), map);
    }

    #[test]
    fn validate_composition_string() {
        let term = parse_MainTerm("m").unwrap();
        assert_eq!(term.composition_string(), "L");

        let term = parse_MainTerm("m2").unwrap();
        assert_eq!(term.composition_string(), "L2");

        let term = parse_MainTerm("m2/s").unwrap();
        assert_eq!(term.composition_string(), "L2.T-1");

        let term = parse_MainTerm("s/m2").unwrap();
        assert_eq!(term.composition_string(), "L-2.T");
    }

    #[test]
    fn validate_is_compatible_with() {
        let me = parse_MainTerm("m2").unwrap();
        let other = parse_MainTerm("m3/m").unwrap();
        assert!(me.is_compatible_with(&other))
    }
}
