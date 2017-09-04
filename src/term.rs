use atom::Atom;
use composition::Composition;
use prefix::Prefix;

#[derive(Debug, PartialEq)]
pub struct Term {
    // TODO: atom probably shouldn't be an Option since a Term is kinda pointless
    // without an atom.
    pub atom: Option<Atom>,
    pub prefix: Option<Prefix>,
    pub factor: u32,
    pub exponent: i32,
    pub annotation: Option<String>
}

impl Term {
    pub fn new(atom: Option<Atom>, prefix: Option<Prefix>) -> Self {
        Term {
            atom: atom,
            prefix: prefix,
            factor: 1,
            exponent: 1,
            annotation: None
        }
    }

    pub fn scalar(&self) -> f64 {
        self.calculate_scalar(1.0)
    }

    pub fn magnitude(&self) -> f64 {
        self.calculate_magnitude(self.scalar())
    }

    // TODO: does this need to take a value? Can this just be scalar()?
    pub fn calculate_scalar(&self, value: f64) -> f64 {
        let e = self.exponent;

        let atom_scalar = match self.atom {
            Some(ref atom) => atom.calculate_scalar(value),
            None => 1.0
        };

        let prefix_scalar = match self.prefix {
            Some(ref prefix) => prefix.magnitude(),
            None => 1.0
        };

        (atom_scalar * prefix_scalar * self.factor as f64).powi(e)
    }

    // TODO: does this need to take a value? Can this just be magnitude()?
    pub fn calculate_magnitude(&self, value: f64) -> f64 {
        let e = self.exponent;

        let atom_magnitude = match self.atom {
            Some(ref atom) => atom.calculate_magnitude(value),
            None => 1.0
        };

        let prefix_magnitude = match self.prefix {
            Some(ref prefix) => atom_magnitude * prefix.scalar(),
            None => 1.0
        };

        (atom_magnitude * prefix_magnitude * self.factor as f64).powi(e)
    }

    pub fn composition(&self) -> Option<Composition> {
        match self.atom {
            Some(ref atom) => {
                let atom_composition = atom.dimension();

                if self.exponent == 1 {
                    return Some(atom_composition);
                }

                let mut new_composition = Composition::new();

                for (dim, exp) in atom_composition.into_iter() {
                    let atom_exp = if exp == 1 { 0 } else { exp };
                    println!("atom_exp: {}", &atom_exp);
                    new_composition.insert(dim, atom_exp + self.exponent);
                }

                println!("new composition: {:?}", &new_composition);
                Some(new_composition)
            },
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use atom::Atom;
    use composition::Composition;
    use dimension::Dimension;
    use prefix::Prefix;
    use term::Term;

    #[test]
    fn validate_scalar() {
        let term = Term::new(Some(Atom::Meter), None);
        assert_eq!(term.scalar(), 1.0);

        let term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        assert_eq!(term.scalar(), 1000.0);

        let mut term = Term::new(Some(Atom::Meter), None);
        term.exponent = -1;
        assert_eq!(term.scalar(), 1.0);

        let mut term = Term::new(Some(Atom::Meter), None);
        term.factor = 10;
        assert_eq!(term.scalar(), 10.0);

        let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        term.factor = 10;
        assert_eq!(term.scalar(), 10_000.0);

        let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        term.factor = 10;
        term.exponent = -1;
        assert_eq!(term.scalar(), 0.0001);
    }

    #[test]
    fn validate_magnitude() {
        let term = Term::new(Some(Atom::Meter), None);
        assert_eq!(term.magnitude(), 1.0);

        let term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        assert_eq!(term.magnitude(), 1000.0);

        let mut term = Term::new(Some(Atom::Meter), None);
        term.exponent = -1;
        assert_eq!(term.magnitude(), 1.0);

        let mut term = Term::new(Some(Atom::Meter), None);
        term.factor = 10;
        assert_eq!(term.magnitude(), 10.0);

        let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        term.factor = 10;
        assert_eq!(term.magnitude(), 10_000.0);

        let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        term.factor = 10;
        term.exponent = -1;
        assert_eq!(term.magnitude(), 0.0001);
    }

    #[test]
    fn validate_composition() {
        let term = Term::new(Some(Atom::Meter), None);
        let composition = Composition::new_from_values(Dimension::Length, 1);
        assert_eq!(term.composition().unwrap(), composition);

        let term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        let composition = Composition::new_from_values(Dimension::Length, 1);
        assert_eq!(term.composition().unwrap(), composition);

        let mut term = Term::new(Some(Atom::Meter), None);
        term.exponent = -1;
        let composition = Composition::new_from_values(Dimension::Length, -1);
        assert_eq!(term.composition().unwrap(), composition);

        let mut term = Term::new(Some(Atom::Meter), None);
        term.exponent = -2;
        let composition = Composition::new_from_values(Dimension::Length, -2);
        assert_eq!(term.composition().unwrap(), composition);

        let mut term = Term::new(Some(Atom::Meter), None);
        term.factor = 10;
        let composition = Composition::new_from_values(Dimension::Length, 1);
        assert_eq!(term.composition().unwrap(), composition);

        let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        term.factor = 10;
        let composition = Composition::new_from_values(Dimension::Length, 1);
        assert_eq!(term.composition().unwrap(), composition);

        let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        term.factor = 10;
        term.exponent = -1;
        let composition = Composition::new_from_values(Dimension::Length, -1);
        assert_eq!(term.composition().unwrap(), composition);
    }
}
