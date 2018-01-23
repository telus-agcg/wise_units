use atom::Atom;
use composition::Composition;
use prefix::Prefix;
use std::fmt;

/// A Term makes up an Atom (at its core) along with any Atom modifiers
/// (anything that can change its scalar). It is, however, possible to have an
/// Atom-less Term, which would simple be a Factor (with or without an
/// annotation) (ex. the 10 in "10" or "10/m" would be an Atom-less Term).
/// 
#[derive(Clone, Debug, PartialEq)]
pub struct Term {
    pub atom: Option<Atom>,
    pub prefix: Option<Prefix>,
    pub factor: u32,
    pub exponent: i32,
    pub annotation: Option<String>,
}

impl Term {
    pub fn new(atom: Option<Atom>, prefix: Option<Prefix>) -> Self {
        Term {
            atom: atom,
            prefix: prefix,
            factor: 1,
            exponent: 1,
            annotation: None,
        }
    }

    pub fn scalar(&self) -> f64 { self.calculate_scalar(1.0) }

    pub fn magnitude(&self) -> f64 { self.calculate_magnitude(self.scalar()) }

    pub fn calculate_scalar(&self, value: f64) -> f64 {
        let e = self.exponent;

        let atom_scalar = self.atom.map_or(1.0, |a| a.calculate_scalar(value));
        let prefix_scalar = self.prefix.map_or(1.0, |p| p.magnitude());

        (atom_scalar * prefix_scalar * f64::from(self.factor)).powi(e)
    }

    pub fn calculate_magnitude(&self, value: f64) -> f64 {
        let e = self.exponent;

        let atom_magnitude = self.atom.map_or(1.0, |a| a.calculate_magnitude(value));
        let prefix_magnitude = self.prefix.map_or(1.0, |p| atom_magnitude * p.scalar());

        (atom_magnitude * prefix_magnitude * f64::from(self.factor)).powi(e)
    }

    pub fn composition(&self) -> Option<Composition> {
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

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", extract_term_string(self))
    }
}

fn extract_term_string(term: &Term) -> String {
    let mut term_string = String::new();

    if term.factor != 1 {
        term_string.push_str(&term.factor.to_string())
    };

    if let Some(prefix) = term.prefix {
        term_string.push_str(&prefix.to_string());
    }

    if let Some(atom) = term.atom {
        if term.exponent == 1 {
            term_string.push_str(&atom.to_string());
        } else {
            term_string.push_str(&format!("{}{}", atom, term.exponent));
        }
    }

    term_string
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
        let term = Term::new(None, None);
        assert_eq!(term.composition(), None);

        let term = Term::new(Some(Atom::Meter), None);
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(term.composition().unwrap(), composition);

        let term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(term.composition().unwrap(), composition);

        let mut term = Term::new(Some(Atom::Meter), None);
        term.exponent = -1;
        let composition = Composition::new(Dimension::Length, -1);
        assert_eq!(term.composition().unwrap(), composition);

        let mut term = Term::new(Some(Atom::Meter), None);
        term.exponent = -2;
        let composition = Composition::new(Dimension::Length, -2);
        assert_eq!(term.composition().unwrap(), composition);

        let mut term = Term::new(Some(Atom::Meter), None);
        term.factor = 10;
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(term.composition().unwrap(), composition);

        let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        term.factor = 10;
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(term.composition().unwrap(), composition);

        let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        term.factor = 10;
        term.exponent = -1;
        let composition = Composition::new(Dimension::Length, -1);
        assert_eq!(term.composition().unwrap(), composition);
    }

    #[test]
    fn validate_display() {
        let term = Term::new(None, None);
        assert_eq!(term.to_string().as_str(), "");

        let term = Term::new(Some(Atom::Meter), None);
        assert_eq!(term.to_string().as_str(), "m");

        let term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        assert_eq!(term.to_string().as_str(), "km");

        let mut term = Term::new(Some(Atom::Meter), None);
        term.exponent = -1;
        assert_eq!(term.to_string().as_str(), "m-1");

        let mut term = Term::new(Some(Atom::Meter), None);
        term.exponent = -2;
        assert_eq!(term.to_string().as_str(), "m-2");

        let mut term = Term::new(Some(Atom::Meter), None);
        term.factor = 10;
        assert_eq!(term.to_string().as_str(), "10m");

        let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        term.factor = 10;
        assert_eq!(term.to_string().as_str(), "10km");

        let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        term.factor = 10;
        term.exponent = -1;
        assert_eq!(term.to_string().as_str(), "10km-1");
    }
}
