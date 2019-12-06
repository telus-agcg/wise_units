use crate::atom::{AtomValue, DerivedAtom, Number};
use crate::tokens::{Component, MainTerm, Term};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref DERIVED_ATOMS: HashMap<&'static str, DerivedAtom> = {
        let mut atoms = HashMap::new();

        atoms.insert(
            "10*",
            DerivedAtom {
                primary_code: "10*",
                atom_value: AtomValue {
                    value: Number::Integer(10),
                    main_term: MainTerm {
                        leading_slash: false,
                        term: Term::Basic(Box::new(Component::Factor(10))),
                    },
                },
            },
        );
        atoms
    };
}
