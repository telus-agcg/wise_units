use parser::parse_MainTerm;
use parser_terms::Term;
use std::fmt;

/// A Measurement is the prime interface for consumers of the library. It
/// consists of some scalar value and a `Term`, where the Term represents the
/// type of unit.
///
/// # Examples
///
/// ```
/// use wise_units::Measurement;
///
/// let one_km = Measurement::new(1.0, "km");
/// let in_meters = one_km.convert_to("m").unwrap();
///
/// // Since we can't assert float values, check that the difference is
/// // negligible.
/// let value_difference = (in_meters.value - 1_000.0).abs();
///
/// assert!(value_difference < 0.000_001);
/// ```
///
#[derive(Debug)]
pub struct Measurement<'a> {
    pub value: f64,
    term: Term<'a>,
}

/// Errors when trying to convert between types that aren't commensurable.
///
#[derive(Debug)]
pub enum ConversionError {
    IncompatibleUnitTypes,
}

impl<'a> Measurement<'a> {
    // TODO: wrong lifetime for expression here
    pub fn new(value: f64, expression: &'a str) -> Self {
        // TODO: unwrap
        let main_term = parse_MainTerm(expression).unwrap();

        Measurement {
            value: value,
            term: main_term,
        }
    }

    /// Converts the Measurement to another unit type. That type is specified
    /// using a str of characters that represents the other unit type: ex.
    /// `"m2/s"`.
    ///
    pub fn convert_to(&self, expression: &'a str) -> Result<Measurement<'a>, ConversionError> {
        let other_term = parse_MainTerm(expression).unwrap();
        let my_term = &self.term;

        if !my_term.is_compatible_with(&other_term) {
            return Err(ConversionError::IncompatibleUnitTypes);
        }

        let new_measurement = Measurement {
            value: self.converted_scalar(&other_term),
            term: other_term,
        };

        Ok(new_measurement)
    }

    /// Checks if the associated Term is "special". "Special" units are ones
    /// that must be converted using a function in combination with some other
    /// non-special units. For example, Celsius is special since it must be
    /// first converted to Kelvin before converting to the requested unit.
    ///
    pub fn is_special(&self) -> bool {
        let ref t = self.term;

        t.is_special()
    }

    /// This scalar is the Measurement's value combined with any scalars that
    /// are part of the Term's designation.
    ///
    /// # Examples
    ///
    /// ```
    /// use wise_units::Measurement;
    ///
    /// let five_meters = Measurement::new(5.0, "m");
    /// assert_eq!(five_meters.scalar(), 5.0);
    ///
    /// let five_meters_squared = Measurement::new(5.0, "m2");
    /// assert_eq!(five_meters_squared.scalar(), 5.0);
    ///
    /// let five_three_meters = Measurement::new(5.0, "[pi].m");
    /// assert_eq!(five_three_meters.scalar(), 15.707_963_267_948_966);
    ///
    /// let sixty_five_f = Measurement::new(65.0, "[degF]");
    /// assert!((sixty_five_f.scalar() - 291.483_333).abs() < 0.000_001);
    /// ```
    ///
    pub fn scalar(&self) -> f64 {
        if self.is_special() {
            let magnitude = self.value;
            self.term.calculate_scalar(magnitude)
        } else {
            self.value * self.term.scalar()
        }
    }

    /// This magnitude is the Measurement's value combined with any magnitude
    /// that is part of the Term's designation.
    ///
    /// # Examples
    ///
    /// ```
    /// use wise_units::Measurement;
    ///
    /// let five_meters = Measurement::new(5.0, "m");
    /// assert_eq!(five_meters.magnitude(), 5.0);
    ///
    /// let five_meters_squared = Measurement::new(5.0, "m2");
    /// assert_eq!(five_meters_squared.magnitude(), 5.0);
    ///
    /// let five_three_meters = Measurement::new(5.0, "[pi].m");
    /// assert_eq!(five_three_meters.magnitude(), 15.707_963_267_948_966);
    ///
    /// let sixty_five_f = Measurement::new(65.0, "[degF]");
    /// assert!((sixty_five_f.magnitude() - 65.0).abs() < 0.000_001);
    /// ```
    ///
    pub fn magnitude(&self) -> f64 {
        if self.is_special() {
            let scalar = self.scalar();
            self.term.calculate_magnitude(scalar)
        } else {
            self.value * self.term.magnitude()
        }
    }

    /// The Measurement's Term as a String.
    ///
    /// # Example
    ///
    /// ```
    /// use wise_units::Measurement;
    /// let km = Measurement::new(1.0, "km");
    /// assert_eq!(km.term_string(), "km".to_string());
    /// ```
    ///
    pub fn term_string(&self) -> String { self.term.to_string() }

    fn converted_scalar(&self, other_term: &Term) -> f64 {
        if self.is_special() && other_term.is_special() {
            let ts = self.term.calculate_scalar(self.value);
            other_term.calculate_magnitude(ts)
        } else if self.is_special() {
            self.term.calculate_scalar(self.value)
        } else if other_term.is_special() {
            other_term.calculate_magnitude(self.value)
        } else {
            self.scalar() / other_term.scalar()
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
            Term::Basic(Component::Annotatable(
                    Annotatable::Unit(SimpleUnit::Atom(Box::new(Meter))),
                    ))
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
        assert_eq!(
            Measurement::new(1.1, "km2").to_string(),
            "1.1km2".to_string()
            );
        assert_eq!(
            Measurement::new(1.1, "km2/s").to_string(),
            "1.1km2/s".to_string()
            );
        assert_eq!(
            Measurement::new(1.1, "km2/s.rad").to_string(),
            "1.1km2/s.rad".to_string()
            );
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
