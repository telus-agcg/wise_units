use composition::Composition;
use classification::Classification;
use definition::Definition;
use measurable::Measurable;
use std::fmt;
use unit_type::UnitType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Prefix {
    Atto,
    Centi,
    Deci,
    Deka,
    Exa,
    Femto,
    Gibi,
    Giga,
    Hecto,
    Kibi,
    Kilo,
    Mebi,
    Mega,
    Micro,
    Milli,
    Nano,
    Peta,
    Pico,
    Tebi,
    Tera,
    Yocto,
    Yotta,
    Zepto,
    Zetta,
}

impl Prefix {
    pub fn classification(&self) -> Classification {
        Classification::SI
    }

    pub fn definition(&self) -> Definition {
        match *self {
            Prefix::Atto => Definition::new(1.0e-18, "1"),
            Prefix::Centi => Definition::new(1.0e-2, "1"),
            Prefix::Deci => Definition::new(1.0e-1, "1"),
            Prefix::Deka => Definition::new(1.0e1, "1"),
            Prefix::Exa => Definition::new(1.0e18, "1"),
            Prefix::Femto => Definition::new(1.0e-15, "1"),
            Prefix::Gibi => Definition::new(1_073_741_824.0, "1"),
            Prefix::Giga => Definition::new(1.0e9, "1"),
            Prefix::Hecto => Definition::new(1.0e2, "1"),
            Prefix::Kibi => Definition::new(1024.0, "1"),
            Prefix::Kilo => Definition::new(1.0e3, "1"),
            Prefix::Mebi => Definition::new(1_048_576.0, "1"),
            Prefix::Mega => Definition::new(1.0e6, "1"),
            Prefix::Micro => Definition::new(1.0e-6, "1"),
            Prefix::Milli => Definition::new(1.0e-3, "1"),
            Prefix::Nano => Definition::new(1.0e-9, "1"),
            Prefix::Peta => Definition::new(1.0e15, "1"),
            Prefix::Pico => Definition::new(1.0e-12, "1"),
            Prefix::Tebi => Definition::new(1_099_511_627_776.0, "1"),
            Prefix::Tera => Definition::new(1.0e12, "1"),
            Prefix::Yocto => Definition::new(1.0e-24, "1"),
            Prefix::Yotta => Definition::new(1.0e24, "1"),
            Prefix::Zepto => Definition::new(1.0e-21, "1"),
            Prefix::Zetta => Definition::new(1.0e21, "1"),
        }
    }

    pub fn composition(&self) -> Composition {
        self.definition().unit.composition()
    }

    pub fn names(&self) -> Vec<String> {
        match *self {
            Prefix::Atto => vec!["atto".to_string()],
            Prefix::Centi => vec!["centi".to_string()],
            Prefix::Deci => vec!["deci".to_string()],
            Prefix::Deka => vec!["deka".to_string()],
            Prefix::Exa => vec!["exa".to_string()],
            Prefix::Femto => vec!["femto".to_string()],
            Prefix::Gibi => vec!["gibi".to_string()],
            Prefix::Giga => vec!["giga".to_string()],
            Prefix::Hecto => vec!["hecto".to_string()],
            Prefix::Kibi => vec!["kibi".to_string()],
            Prefix::Kilo => vec!["kilo".to_string()],
            Prefix::Mebi => vec!["mebi".to_string()],
            Prefix::Mega => vec!["mega".to_string()],
            Prefix::Micro => vec!["micro".to_string()],
            Prefix::Milli => vec!["milli".to_string()],
            Prefix::Nano => vec!["nano".to_string()],
            Prefix::Peta => vec!["peta".to_string()],
            Prefix::Pico => vec!["pico".to_string()],
            Prefix::Tebi => vec!["tebi".to_string()],
            Prefix::Tera => vec!["tera".to_string()],
            Prefix::Yocto => vec!["yocto".to_string()],
            Prefix::Yotta => vec!["yotta".to_string()],
            Prefix::Zepto => vec!["zepto".to_string()],
            Prefix::Zetta => vec!["zetta".to_string()],
        }
    }

    pub fn primary_code(&self) -> String {
        match *self {
            Prefix::Atto => "a".to_string(),
            Prefix::Centi => "c".to_string(),
            Prefix::Deci => "d".to_string(),
            Prefix::Deka => "da".to_string(),
            Prefix::Exa => "E".to_string(),
            Prefix::Femto => "f".to_string(),
            Prefix::Gibi => "Gi".to_string(),
            Prefix::Giga => "G".to_string(),
            Prefix::Hecto => "h".to_string(),
            Prefix::Kibi => "Ki".to_string(),
            Prefix::Kilo => "k".to_string(),
            Prefix::Mebi => "Mi".to_string(),
            Prefix::Mega => "M".to_string(),
            Prefix::Micro => "u".to_string(),
            Prefix::Milli => "m".to_string(),
            Prefix::Nano => "n".to_string(),
            Prefix::Peta => "P".to_string(),
            Prefix::Pico => "p".to_string(),
            Prefix::Tebi => "Ti".to_string(),
            Prefix::Tera => "T".to_string(),
            Prefix::Yocto => "y".to_string(),
            Prefix::Yotta => "Y".to_string(),
            Prefix::Zepto => "z".to_string(),
            Prefix::Zetta => "Z".to_string(),
        }
    }

    pub fn print_symbol(&self) -> Option<String> {
        match *self {
            Prefix::Micro => Some("μ".to_string()),
            _ => Some(self.primary_code())
        }
    }

    pub fn secondary_code(&self) -> String {
        match *self {
            Prefix::Atto => "A".to_string(),
            Prefix::Centi => "C".to_string(),
            Prefix::Deci => "D".to_string(),
            Prefix::Deka => "DA".to_string(),
            Prefix::Exa => "EX".to_string(),
            Prefix::Femto => "F".to_string(),
            Prefix::Gibi => "GIB".to_string(),
            Prefix::Giga => "GA".to_string(),
            Prefix::Hecto => "H".to_string(),
            Prefix::Kibi => "KIB".to_string(),
            Prefix::Kilo => "K".to_string(),
            Prefix::Mebi => "MIB".to_string(),
            Prefix::Mega => "MA".to_string(),
            Prefix::Micro => "U".to_string(),
            Prefix::Milli => "M".to_string(),
            Prefix::Nano => "N".to_string(),
            Prefix::Peta => "PT".to_string(),
            Prefix::Pico => "P".to_string(),
            Prefix::Tebi => "TIB".to_string(),
            Prefix::Tera => "TR".to_string(),
            Prefix::Yocto => "YO".to_string(),
            Prefix::Yotta => "YA".to_string(),
            Prefix::Zepto => "ZO".to_string(),
            Prefix::Zetta => "ZA".to_string(),
        }
    }

    pub fn unit_type(&self) -> UnitType {
        UnitType::Prefix
    }

    pub fn scalar(&self) -> f64 {
        self.definition().scalar()
    }

    pub fn magnitude(&self) -> f64 {
        self.definition().magnitude()
    }

    // TODO: is ok?
    pub fn calculate_scalar(&self, magnitude: f64) -> f64 {
        self.definition().calculate_scalar(magnitude)
    }

    // TODO: is ok?
    pub fn calculate_magnitude(&self, scalar: f64) -> f64 {
        self.definition().calculate_magnitude(scalar)
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "{}", self.primary_code())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Prefix;
    use composition::Composition;

    #[test]
    fn validate_scalar_atto() {
        let prefix = Prefix::Atto;
        assert_floats_eq(prefix.scalar(), 0.000_000_000_000_000_001);
    }

    #[test]
    fn validate_scalar_centi() {
        let prefix = Prefix::Centi;
        assert_floats_eq(prefix.scalar(), 0.01);
    }

    #[test]
    fn validate_scalar_deci() {
        let prefix = Prefix::Deci;
        assert_floats_eq(prefix.scalar(), 0.1);
    }

    #[test]
    fn validate_scalar_deka() {
        let prefix = Prefix::Deka;
        assert_floats_eq(prefix.scalar(), 10.0);
    }

    #[test]
    fn validate_composition() {
        let prefix = Prefix::Kilo;
        let composition = Composition::new_unity();
        assert_eq!(prefix.composition(), composition)
    }

    #[test]
    fn validate_display() {
        let prefix = Prefix::Kilo;
        assert_eq!(&prefix.to_string(), "k")
    }

    // Because the precision of floats can vary, using assert_eq! with float values
    // is not recommended; clippy's recommendation is to calculate the absolute
    // value of the difference and make sure that it's under some acceptable
    // threshold.
    fn assert_floats_eq(actual: f64, expected: f64) {
        let error_threshold = ::std::f32::EPSILON as f64;
        let difference = actual - expected;

        assert!(difference.abs() < error_threshold, "Actual: {}, Expected: {}, Diff: {}", actual, expected, difference);
    }
}
