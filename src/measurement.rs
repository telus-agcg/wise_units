use parser::parse_MainTerm;
use parser_terms::Term;
use std::fmt;

#[derive(Debug)]
pub struct Measurement<'a> {
    pub value: f64,
    term: Term<'a>,
}

#[derive(Debug)]
pub enum ConversionError {
    Incompatible
}

impl<'a> Measurement<'a> {
    // TODO: wrong lifetime for expression here
    pub fn new(value: f64, expression: &'a str) -> Self {
        // TODO: unwrap
        let main_term = parse_MainTerm(expression).unwrap();

        Measurement {
            value: value,
            term: main_term
        }
    }

    pub fn convert_to(&self, expression: &'a str) -> Result<Measurement<'a>, ConversionError> {
        let other_term = parse_MainTerm(expression).unwrap();
        let my_term = &self.term;

        if !my_term.is_compatible_with(&other_term) {
            return Err(ConversionError::Incompatible)
        }

        let new_measurement = Measurement {
            value: self.converted_value(&other_term),
            term: other_term
        };

        Ok(new_measurement)
    }

    pub fn is_special(&self) -> bool {
        let ref t = self.term;

        t.is_special()
    }

    pub fn scalar(&self, magnitude: f64) -> f64 {
        if self.is_special() {
            self.term.scalar(magnitude)
        } else {
            self.value * self.term.scalar_default()
        }
    }

    pub fn scalar_default(&self) -> f64 {
        self.scalar(self.value)
    }

    /// The Measurement's Term as a String.
    ///
    /// # Example
    ///
    /// ```
    /// use wu::Measurement;
    /// let km = Measurement::new(1.0, "km");
    /// assert_eq!(km.term_string(), "km".to_string());
    /// ```
    ///
    pub fn term_string(&self) -> String {
        self.term.to_string()
    }

    fn converted_value(&self, other_term: &Term) -> f64 {
        if other_term.is_special() {
            other_term.magnitude(self.scalar_default())
        } else {
            self.scalar_default() / other_term.scalar_default()
        }
    }
}

impl<'a> fmt::Display for Measurement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.value, self.term)
    }
}

impl<'a> PartialEq for Measurement<'a> {
    fn eq(&self, other: &Self) -> bool {
        let my_term_string = self.term_string();

        if let Ok(converted_other) = other.convert_to(&my_term_string) {
            self.to_string() == converted_other.to_string()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser_terms::{Annotatable, Component, SimpleUnit, Term};
    use unit::base::Meter;

    #[test]
    fn validate_new() {
        let m = Measurement::new(1.0, "m");

        assert_eq!(m.value, 1.0);
        assert_eq!(
            m.term,
            Term::Basic(
                Component::Annotatable(
                    Annotatable::Unit(
                        SimpleUnit::Atom(Box::new(Meter))
                        )
                    )
                )
            );
    }

    #[test]
    fn validate_convert_to() {
        let m = Measurement::new(1.0, "m");
        let other = m.convert_to("m").unwrap();
        assert_eq!(other, m);

        let mut other = m.convert_to("m").unwrap();
        other.value = 2.0;
        assert_ne!(other, m);
    }

    #[test]
    fn validate_display() {
        assert_eq!(Measurement::new(1.0, "m").to_string(), "1m".to_string());
        assert_eq!(Measurement::new(1.1, "m").to_string(), "1.1m".to_string());
        assert_eq!(Measurement::new(1.1, "m2").to_string(), "1.1m2".to_string());
        assert_eq!(Measurement::new(1.1, "km2").to_string(), "1.1km2".to_string());
        assert_eq!(Measurement::new(1.1, "km2/s").to_string(), "1.1km2/s".to_string());
        assert_eq!(Measurement::new(1.1, "km2/s.rad").to_string(), "1.1km2/s.rad".to_string());
    }

    #[test]
    fn validate_eq_same_unit() {
        let m1 = Measurement::new(1.0, "m");
        let m2 = Measurement::new(1.0, "m");
        assert_eq!(&m1, &m2);

        let m2 = Measurement::new(1.1, "m");
        assert_ne!(m1, m2);
    }

    #[test]
    fn validate_eq_unit_with_prefix() {
        let m = Measurement::new(1000.0, "m");
        let km = Measurement::new(1.0, "km");
        assert_eq!(&m, &km);

        let km = Measurement::new(1.1, "km");
        assert_ne!(&m, &km);
    }

    #[test]
    fn validate_eq_different_unit() {
        let m = Measurement::new(1.0, "m");
        let s = Measurement::new(1.0, "s");
        assert_ne!(&m, &s);
    }
}
