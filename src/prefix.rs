use atom::Atom;
use classification::Classification;
use composition::Composition;
use definition::Definition;
use measurable::Measurable;
use std::fmt;
use term::Term;
use unit::Unit;

#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
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
    pub fn classification(&self) -> Classification { Classification::SI }

    pub fn definition(&self) -> Definition {
        let term = Term::new(Some(Atom::TheUnity), None);
        let unit = Unit { terms: vec![term] };

        let value = match *self {
            Prefix::Atto  => 1.0e-18,
            Prefix::Centi => 1.0e-2,
            Prefix::Deci  => 1.0e-1,
            Prefix::Deka  => 1.0e1,
            Prefix::Exa   => 1.0e18,
            Prefix::Femto => 1.0e-15,
            Prefix::Gibi  => 1_073_741_824.0,
            Prefix::Giga  => 1.0e9,
            Prefix::Hecto => 1.0e2,
            Prefix::Kibi  => 1024.0,
            Prefix::Kilo  => 1.0e3,
            Prefix::Mebi  => 1_048_576.0,
            Prefix::Mega  => 1.0e6,
            Prefix::Micro => 1.0e-6,
            Prefix::Milli => 1.0e-3,
            Prefix::Nano  => 1.0e-9,
            Prefix::Peta  => 1.0e15,
            Prefix::Pico  => 1.0e-12,
            Prefix::Tebi  => 1_099_511_627_776.0,
            Prefix::Tera  => 1.0e12,
            Prefix::Yocto => 1.0e-24,
            Prefix::Yotta => 1.0e24,
            Prefix::Zepto => 1.0e-21,
            Prefix::Zetta => 1.0e21,
        };

        Definition {
            value: value,
            unit: unit,
        }
    }

    pub fn composition(&self) -> Option<Composition> { None }

    pub fn names(&self) -> Vec<&'static str> {
        match *self {
            Prefix::Atto  => vec!["atto"],
            Prefix::Centi => vec!["centi"],
            Prefix::Deci  => vec!["deci"],
            Prefix::Deka  => vec!["deka"],
            Prefix::Exa   => vec!["exa"],
            Prefix::Femto => vec!["femto"],
            Prefix::Gibi  => vec!["gibi"],
            Prefix::Giga  => vec!["giga"],
            Prefix::Hecto => vec!["hecto"],
            Prefix::Kibi  => vec!["kibi"],
            Prefix::Kilo  => vec!["kilo"],
            Prefix::Mebi  => vec!["mebi"],
            Prefix::Mega  => vec!["mega"],
            Prefix::Micro => vec!["micro"],
            Prefix::Milli => vec!["milli"],
            Prefix::Nano  => vec!["nano"],
            Prefix::Peta  => vec!["peta"],
            Prefix::Pico  => vec!["pico"],
            Prefix::Tebi  => vec!["tebi"],
            Prefix::Tera  => vec!["tera"],
            Prefix::Yocto => vec!["yocto"],
            Prefix::Yotta => vec!["yotta"],
            Prefix::Zepto => vec!["zepto"],
            Prefix::Zetta => vec!["zetta"],
        }
    }

    pub fn primary_code(&self) -> &'static str {
        match *self {
            Prefix::Atto  => "a",
            Prefix::Centi => "c",
            Prefix::Deci  => "d",
            Prefix::Deka  => "da",
            Prefix::Exa   => "E",
            Prefix::Femto => "f",
            Prefix::Gibi  => "Gi",
            Prefix::Giga  => "G",
            Prefix::Hecto => "h",
            Prefix::Kibi  => "Ki",
            Prefix::Kilo  => "k",
            Prefix::Mebi  => "Mi",
            Prefix::Mega  => "M",
            Prefix::Micro => "u",
            Prefix::Milli => "m",
            Prefix::Nano  => "n",
            Prefix::Peta  => "P",
            Prefix::Pico  => "p",
            Prefix::Tebi  => "Ti",
            Prefix::Tera  => "T",
            Prefix::Yocto => "y",
            Prefix::Yotta => "Y",
            Prefix::Zepto => "z",
            Prefix::Zetta => "Z",
        }
    }

    pub fn print_symbol(&self) -> Option<&'static str> {
        match *self {
            Prefix::Micro => Some("μ"),
            _ => Some(self.primary_code()),
        }
    }

    pub fn secondary_code(&self) -> &'static str {
        match *self {
            Prefix::Atto  => "A",
            Prefix::Centi => "C",
            Prefix::Deci  => "D",
            Prefix::Deka  => "DA",
            Prefix::Exa   => "EX",
            Prefix::Femto => "F",
            Prefix::Gibi  => "GIB",
            Prefix::Giga  => "GA",
            Prefix::Hecto => "H",
            Prefix::Kibi  => "KIB",
            Prefix::Kilo  => "K",
            Prefix::Mebi  => "MIB",
            Prefix::Mega  => "MA",
            Prefix::Micro => "U",
            Prefix::Milli => "M",
            Prefix::Nano  => "N",
            Prefix::Peta  => "PT",
            Prefix::Pico  => "P",
            Prefix::Tebi  => "TIB",
            Prefix::Tera  => "TR",
            Prefix::Yocto => "YO",
            Prefix::Yotta => "YA",
            Prefix::Zepto => "ZO",
            Prefix::Zetta => "ZA",
        }
    }

    pub fn scalar(&self) -> f64 { self.definition().scalar() }

    pub fn magnitude(&self) -> f64 { self.definition().magnitude() }

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
            _ => write!(f, "{}", self.primary_code()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Prefix;

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
        assert_eq!(prefix.composition(), None)
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

        assert!(
            difference.abs() < error_threshold,
            "Actual: {}, Expected: {}, Diff: {}",
            actual,
            expected,
            difference
        );
    }

    #[cfg(feature = "with_serde")]
    mod with_serde {
        use super::super::Prefix;
        use serde_json;

        #[test]
        fn validate_serialization() {
            let j = serde_json::to_string(&Prefix::Kilo)
                .expect("Couldn't convert Prefix to JSON String");

            assert_eq!("\"Kilo\"", j);
        }

        #[test]
        fn validate_deserialization() {
            let k =
                serde_json::from_str("\"Kilo\"").expect("Couldn't convert JSON String to Prefix");

            assert_eq!(Prefix::Kilo, k);
        }
    }
}
