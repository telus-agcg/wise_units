use atom::Atom;
use composition::Composition;
use prefix::Prefix;

#[derive(Debug, PartialEq)]
pub struct Term {
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
        let atom_scalar = match self.atom {
            Some(ref unit) => unit.scalar(),
            None => self.magnitude()
        };

        match self.prefix {
            Some(ref prefix) => atom_scalar * prefix.scalar(),
            None => atom_scalar
        }
    }

    pub fn magnitude(&self) -> f64 {
        let atom_magnitude = match self.atom {
            Some(ref unit) => unit.magnitude(),
            None => 1.0
        };

        match self.prefix {
            Some(ref prefix) => atom_magnitude * prefix.magnitude(),
            None => atom_magnitude
        }
    }

    // Does this need to account for prefix?
    pub fn calculate_scalar(&self, magnitude: f64) -> f64 {
        let e = self.exponent;

        let atom_scalar = match self.atom {
            Some(ref atom) => atom.calculate_scalar(magnitude).powi(e),
            None => magnitude
        };

        match self.prefix {
            Some(ref prefix) => atom_scalar * prefix.magnitude(),
            None => atom_scalar
        }
    }

    pub fn calculate_magnitude(&self, scalar: f64) -> f64 {
        let e = self.exponent;

        let atom_magnitude = match self.atom {
            Some(ref atom) => atom.calculate_magnitude(scalar).powi(e),
            None => 1.0
        };

        match self.prefix {
            Some(ref prefix) => atom_magnitude * prefix.scalar(),
            None => atom_magnitude
        }
    }

    pub fn composition(&self) -> Option<Composition> {
        match self.atom {
            Some(ref atom) => Some(atom.dimension()),
            None => None
        }
    }
}

