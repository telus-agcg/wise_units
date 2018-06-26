use parser::{definition::Definition, Classification};
use parser::ucum_symbol::UcumSymbol;
use std::fmt;
use unit::Unit;

/// A `Prefix` is essentially a multiplier for an `Atom` within a `Term`; ex.
/// the "c" in "cm" modifies meter by 0.01. The UCUM spec says these should
/// only pertain to metric units, but that rule is not adhered to in
/// `wise_units`.
///
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

impl UcumSymbol for Prefix {
    fn classification(&self) -> Classification {
        Classification::Si
    }

    fn names(&self) -> Vec<&'static str> {
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

    fn primary_code(&self) -> &'static str {
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

    fn print_symbol(&self) -> Option<&'static str> {
        match *self {
            Prefix::Micro => Some("μ"),
            _ => Some(self.primary_code()),
        }
    }

    fn secondary_code(&self) -> Option<&'static str> {
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

    /// The numeric value that each `Prefix` represents.
    ///
    fn definition_value(&self) -> f64 {
        match *self {
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
        }
    }

    fn definition_unit(&self) -> Unit {
        let definition = Definition::default();

        Unit { terms: definition.terms().to_vec() }
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
    use parser::ucum_symbol::UcumSymbol;

    macro_rules! validate_value {
        ($test_name:ident, $variant:ident, $value:expr) => {
            #[test]
            fn $test_name() {
                let prefix = Prefix::$variant;
                assert_relative_eq!(prefix.definition_value(), $value);
                assert_ulps_eq!(prefix.definition_value(), $value);
            }
        };
    }

    macro_rules! validate_values {
        ($($test_name: ident, $variant: ident, $value: expr);+ $(;)*) => {
            $(
                validate_value!($test_name, $variant, $value);
            )+
        };
    }

    validate_values!(
        validate_value_atto, Atto, 1.0e-18;
        validate_value_centi, Centi, 1.0e-2;
        validate_value_deci, Deci, 1.0e-1;
        validate_value_deka, Deka, 1.0e1;
        validate_value_exa, Exa, 1.0e18;
        validate_value_femto, Femto, 1.0e-15;
        validate_value_gibi, Gibi, 1_073_741_824.0;
        validate_value_giga, Giga, 1.0e9;
        validate_value_hecto, Hecto, 1.0e2;
        validate_value_kibi, Kibi, 1024.0;
        validate_value_kilo, Kilo, 1.0e3;
        validate_value_mebi, Mebi, 1_048_576.0;
        validate_value_mega, Mega, 1.0e6;
        validate_value_micro, Micro, 1.0e-6;
        validate_value_milli, Milli, 1.0e-3;
        validate_value_nano, Nano, 1.0e-9;
        validate_value_peta, Peta, 1.0e15;
        validate_value_pico, Pico, 1.0e-12;
        validate_value_tebi, Tebi, 1_099_511_627_776.0;
        validate_value_tera, Tera, 1.0e12;
        validate_value_yocto, Yocto, 1.0e-24;
        validate_value_yotta, Yotta, 1.0e24;
        validate_value_zepto, Zepto, 1.0e-21;
        validate_value_zetta, Zetta, 1.0e21;
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
