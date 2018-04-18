use composition::Composition;
use dimension::Dimension;
use unit_parser::{Atom, Prefix, Term};

pub trait Composable {
    fn composition(&self) -> Option<Composition>;

    fn is_compatible_with<T: Composable>(&self, other_unit: &T) -> bool {
        let me = self.composition();
        let other_comp = other_unit.composition();

        me == other_comp
    }
}

impl Composable for Prefix {
    fn composition(&self) -> Option<Composition> {
        None
    }
}

impl Composable for Atom {
    fn composition(&self) -> Option<Composition> {
        match *self {
            Atom::TheUnity => None,
            Atom::Candela => Some(Composition::new(Dimension::LuminousIntensity, 1)),
            Atom::Coulomb => Some(Composition::new(Dimension::ElectricCharge, 1)),
            Atom::Gram => Some(Composition::new(Dimension::Mass, 1)),
            Atom::Kelvin => Some(Composition::new(Dimension::Temperature, 1)),
            Atom::Meter => Some(Composition::new(Dimension::Length, 1)),
            Atom::Radian => Some(Composition::new(Dimension::PlaneAngle, 1)),
            Atom::Second => Some(Composition::new(Dimension::Time, 1)),
            _ => self.definition().unit.composition(),
        }
    }
}

impl Composable for Term {
    fn composition(&self) -> Option<Composition> {
        self.atom.and_then(|ref atom| {
            atom.composition().and_then(|composition| {
                if self.exponent == 1 {
                    return Some(composition);
                }

                let mut new_composition = Composition::default();

                for (dim, exp) in composition {
                    let atom_exp = if exp == 1 { 0 } else { exp };
                    new_composition.insert(dim, atom_exp + self.exponent);
                }

                Some(new_composition)
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use unit_parser::{Atom, Prefix};

    #[test]
    fn validate_prefix_composition() {
        let prefix = Prefix::Kilo;
        assert_eq!(prefix.composition(), None)
    }

    #[test]
    fn validate_atom_composition() {
        let atom = Atom::Candela;
        let composition = Composition::new(Dimension::LuminousIntensity, 1);
        assert_eq!(atom.composition().unwrap(), composition);

        let atom = Atom::Coulomb;
        let composition = Composition::new(Dimension::ElectricCharge, 1);
        assert_eq!(atom.composition().unwrap(), composition);

        let atom = Atom::Gram;
        let composition = Composition::new(Dimension::Mass, 1);
        assert_eq!(atom.composition().unwrap(), composition);

        let atom = Atom::Kelvin;
        let composition = Composition::new(Dimension::Temperature, 1);
        assert_eq!(atom.composition().unwrap(), composition);

        let atom = Atom::Meter;
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(atom.composition().unwrap(), composition);

        let atom = Atom::Radian;
        let composition = Composition::new(Dimension::PlaneAngle, 1);
        assert_eq!(atom.composition().unwrap(), composition);

        let atom = Atom::Second;
        let composition = Composition::new(Dimension::Time, 1);
        assert_eq!(atom.composition().unwrap(), composition);
    }
}
