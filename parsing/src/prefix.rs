use classification::Classification;
use definition::Definition;
use std::fmt;
use term::Term;

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
    pub fn classification(&self) -> Classification {
        Classification::Si
    }

    pub fn definition(&self) -> Definition {
        let term = Term::new(None, None);
        let terms = vec![term];

        let value = match *self {
            Prefix::Atto => 1.0e-18,
            Prefix::Centi => 1.0e-2,
            Prefix::Deci => 1.0e-1,
            Prefix::Deka => 1.0e1,
            Prefix::Exa => 1.0e18,
            Prefix::Femto => 1.0e-15,
            Prefix::Gibi => 1_073_741_824.0,
            Prefix::Giga => 1.0e9,
            Prefix::Hecto => 1.0e2,
            Prefix::Kibi => 1024.0,
            Prefix::Kilo => 1.0e3,
            Prefix::Mebi => 1_048_576.0,
            Prefix::Mega => 1.0e6,
            Prefix::Micro => 1.0e-6,
            Prefix::Milli => 1.0e-3,
            Prefix::Nano => 1.0e-9,
            Prefix::Peta => 1.0e15,
            Prefix::Pico => 1.0e-12,
            Prefix::Tebi => 1_099_511_627_776.0,
            Prefix::Tera => 1.0e12,
            Prefix::Yocto => 1.0e-24,
            Prefix::Yotta => 1.0e24,
            Prefix::Zepto => 1.0e-21,
            Prefix::Zetta => 1.0e21,
        };

        Definition {
            value,
            terms,
            function_set: None,
        }
    }

    pub fn names(&self) -> Vec<&'static str> {
        match *self {
            Prefix::Atto => vec!["atto"],
            Prefix::Centi => vec!["centi"],
            Prefix::Deci => vec!["deci"],
            Prefix::Deka => vec!["deka"],
            Prefix::Exa => vec!["exa"],
            Prefix::Femto => vec!["femto"],
            Prefix::Gibi => vec!["gibi"],
            Prefix::Giga => vec!["giga"],
            Prefix::Hecto => vec!["hecto"],
            Prefix::Kibi => vec!["kibi"],
            Prefix::Kilo => vec!["kilo"],
            Prefix::Mebi => vec!["mebi"],
            Prefix::Mega => vec!["mega"],
            Prefix::Micro => vec!["micro"],
            Prefix::Milli => vec!["milli"],
            Prefix::Nano => vec!["nano"],
            Prefix::Peta => vec!["peta"],
            Prefix::Pico => vec!["pico"],
            Prefix::Tebi => vec!["tebi"],
            Prefix::Tera => vec!["tera"],
            Prefix::Yocto => vec!["yocto"],
            Prefix::Yotta => vec!["yotta"],
            Prefix::Zepto => vec!["zepto"],
            Prefix::Zetta => vec!["zetta"],
        }
    }

    pub fn primary_code(&self) -> &'static str {
        match *self {
            Prefix::Atto => "a",
            Prefix::Centi => "c",
            Prefix::Deci => "d",
            Prefix::Deka => "da",
            Prefix::Exa => "E",
            Prefix::Femto => "f",
            Prefix::Gibi => "Gi",
            Prefix::Giga => "G",
            Prefix::Hecto => "h",
            Prefix::Kibi => "Ki",
            Prefix::Kilo => "k",
            Prefix::Mebi => "Mi",
            Prefix::Mega => "M",
            Prefix::Micro => "u",
            Prefix::Milli => "m",
            Prefix::Nano => "n",
            Prefix::Peta => "P",
            Prefix::Pico => "p",
            Prefix::Tebi => "Ti",
            Prefix::Tera => "T",
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

    pub fn secondary_code(&self) -> Option<&'static str> {
        let code = match *self {
            Prefix::Atto => "A",
            Prefix::Centi => "C",
            Prefix::Deci => "D",
            Prefix::Deka => "DA",
            Prefix::Exa => "EX",
            Prefix::Femto => "F",
            Prefix::Gibi => "GIB",
            Prefix::Giga => "GA",
            Prefix::Hecto => "H",
            Prefix::Kibi => "KIB",
            Prefix::Kilo => "K",
            Prefix::Mebi => "MIB",
            Prefix::Mega => "MA",
            Prefix::Micro => "U",
            Prefix::Milli => "M",
            Prefix::Nano => "N",
            Prefix::Peta => "PT",
            Prefix::Pico => "P",
            Prefix::Tebi => "TIB",
            Prefix::Tera => "TR",
            Prefix::Yocto => "YO",
            Prefix::Yotta => "YA",
            Prefix::Zepto => "ZO",
            Prefix::Zetta => "ZA",
        };

        Some(code)
    }

    pub fn is_metric(&self) -> bool {
        true
    }

    pub fn is_arbitrary(&self) -> bool {
        false
    }

    pub fn is_special(&self) -> bool {
        false
    }

    pub fn scalar(&self) -> f64 {
        self.calculate_scalar(1.0)
    }

    pub fn magnitude(&self) -> f64 {
        self.calculate_magnitude(1.0)
    }

    // TODO: It seems really silly to have to depend on a Definition when all
    // Prefixes really do is just multiple an Atom by some scalar value.
    pub fn calculate_scalar(&self, magnitude: f64) -> f64 {
        self.definition().calculate_scalar(magnitude)
    }

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

    macro_rules! validate_scalar {
        ($test_name:ident, $variant:ident, $value:expr) => {
            #[test]
            fn $test_name() {
                let prefix = Prefix::$variant;
                assert_relative_eq!(prefix.scalar(), $value);
                assert_ulps_eq!(prefix.scalar(), $value);
            }
        };
    }

    macro_rules! validate_scalars {
        ($($test_name: ident, $variant: ident, $value: expr);+ $(;)*) => {
            $(
                validate_scalar!($test_name, $variant, $value);
            )+
        };
    }

    validate_scalars!(
        validate_scalar_atto, Atto, 1.0e-18;
        validate_scalar_centi, Centi, 1.0e-2;
        validate_scalar_deci, Deci, 1.0e-1;
        validate_scalar_deka, Deka, 1.0e1;
        validate_scalar_exa, Exa, 1.0e18;
        validate_scalar_femto, Femto, 1.0e-15;
        validate_scalar_gibi, Gibi, 1_073_741_824.0;
        validate_scalar_giga, Giga, 1.0e9;
        validate_scalar_hecto, Hecto, 1.0e2;
        validate_scalar_kibi, Kibi, 1024.0;
        validate_scalar_kilo, Kilo, 1.0e3;
        validate_scalar_mebi, Mebi, 1_048_576.0;
        validate_scalar_mega, Mega, 1.0e6;
        validate_scalar_micro, Micro, 1.0e-6;
        validate_scalar_milli, Milli, 1.0e-3;
        validate_scalar_nano, Nano, 1.0e-9;
        validate_scalar_peta, Peta, 1.0e15;
        validate_scalar_pico, Pico, 1.0e-12;
        validate_scalar_tebi, Tebi, 1_099_511_627_776.0;
        validate_scalar_tera, Tera, 1.0e12;
        validate_scalar_yocto, Yocto, 1.0e-24;
        validate_scalar_yotta, Yotta, 1.0e24;
        validate_scalar_zepto, Zepto, 1.0e-21;
        validate_scalar_zetta, Zetta, 1.0e21;
    );

    #[test]
    fn validate_display() {
        let prefix = Prefix::Kilo;
        assert_eq!(&prefix.to_string(), "k")
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
