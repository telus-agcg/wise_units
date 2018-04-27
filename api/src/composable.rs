use composition::Composition;
use dimension::Dimension;
use wise_units_parsing::{Atom, Term, UcumSymbol};

pub trait Composable {
    fn composition(&self) -> Composition;

    fn is_compatible_with<T: Composable>(&self, other_unit: &T) -> bool {
        let me = self.composition();
        let other_comp = other_unit.composition();

        me == other_comp
    }
}

impl Composable for Atom {
    fn composition(&self) -> Composition {
        match *self {
            Atom::Candela => Composition::new(Dimension::LuminousIntensity, 1),
            Atom::Coulomb => Composition::new(Dimension::ElectricCharge, 1),
            Atom::Gram => Composition::new(Dimension::Mass, 1),
            Atom::Kelvin => Composition::new(Dimension::Temperature, 1),
            Atom::Meter => Composition::new(Dimension::Length, 1),
            Atom::Radian => Composition::new(Dimension::PlaneAngle, 1),
            Atom::Second => Composition::new(Dimension::Time, 1),
            _ => self.definition().terms.composition(),
        }
    }
}

impl Composable for Term {
    fn composition(&self) -> Composition {
        match self.atom {
            Some(atom) => {
                let composition = atom.composition();

                if self.exponent == 1 {
                    return composition;
                }

                let mut new_composition = Composition::default();

                for (dim, exp) in composition {
                    let atom_exp = if exp == 1 { 0 } else { exp };
                    new_composition.insert(dim, atom_exp + self.exponent);
                }

                new_composition
            },
            None => Composition::default()
        }
    }
}

impl Composable for Vec<Term> {
    fn composition(&self) -> Composition {
        let mut composition = Composition::default();

        for term in self {
            let term_composition = term.composition();

            for (term_dimension, term_exponent) in term_composition {
                composition.insert(term_dimension, term_exponent);
            }
        }

        composition
    }
}

#[cfg(test)]
mod tests {
    use super::Composable;
    use composition::Composition;
    use dimension::Dimension;
    use wise_units_parsing::Atom;

    #[test]
    fn validate_atom_composition() {
        let atom = Atom::Candela;
        let composition = Composition::new(Dimension::LuminousIntensity, 1);
        assert_eq!(atom.composition(), composition);

        let atom = Atom::Coulomb;
        let composition = Composition::new(Dimension::ElectricCharge, 1);
        assert_eq!(atom.composition(), composition);

        let atom = Atom::Gram;
        let composition = Composition::new(Dimension::Mass, 1);
        assert_eq!(atom.composition(), composition);

        let atom = Atom::Kelvin;
        let composition = Composition::new(Dimension::Temperature, 1);
        assert_eq!(atom.composition(), composition);

        let atom = Atom::Meter;
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(atom.composition(), composition);

        let atom = Atom::Radian;
        let composition = Composition::new(Dimension::PlaneAngle, 1);
        assert_eq!(atom.composition(), composition);

        let atom = Atom::Second;
        let composition = Composition::new(Dimension::Time, 1);
        assert_eq!(atom.composition(), composition);
    }
}
