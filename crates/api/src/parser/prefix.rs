mod num_traits;

use std::fmt;

use pest::iterators::Pair;

use crate::{
    parser::{symbols::symbol_parser::Rule, ucum_symbol::UcumSymbol},
    unit::{self, Unit},
    Classification,
};

use super::{Error, Visit};

const DECI: f64 = 1.0e-1;
const CENTI: f64 = 1.0e-2;
const MILLI: f64 = 1.0e-3;
const MICRO: f64 = 1.0e-6;
const NANO: f64 = 1.0e-9;
const PICO: f64 = 1.0e-12;
const FEMTO: f64 = 1.0e-15;
const ATTO: f64 = 1.0e-18;
const ZEPTO: f64 = 1.0e-21;
const YOCTO: f64 = 1.0e-24;

const DEKA: f64 = 1.0e1;
const HECTO: f64 = 1.0e2;
const KILO: f64 = 1.0e3;
const MEGA: f64 = 1.0e6;
const GIGA: f64 = 1.0e9;
const TERA: f64 = 1.0e12;
const PETA: f64 = 1.0e15;
const EXA: f64 = 1.0e18;
const ZETTA: f64 = 1.0e21;
const YOTTA: f64 = 1.0e24;

const KIBI: f64 = 1024.0;
const MEBI: f64 = 1_048_576.0;
const GIBI: f64 = 1_073_741_824.0;
const TEBI: f64 = 1_099_511_627_776.0;

pub(crate) mod u128 {
    pub(crate) const DEKA: u128 = 10;
    pub(crate) const HECTO: u128 = 100;
    pub(crate) const KILO: u128 = 1000;
    pub(crate) const MEGA: u128 = 1_000_000;
    pub(crate) const GIGA: u128 = 1_000_000_000;
    pub(crate) const TERA: u128 = 1_000_000_000_000;
    pub(crate) const PETA: u128 = 1_000_000_000_000_000;
    pub(crate) const EXA: u128 = 1_000_000_000_000_000_000;
    pub(crate) const ZETTA: u128 = 1_000_000_000_000_000_000_000;
    pub(crate) const YOTTA: u128 = 1_000_000_000_000_000_000_000_000;

    pub(crate) const KIBI: u128 = 1024;
    pub(crate) const MEBI: u128 = 1_048_576;
    pub(crate) const GIBI: u128 = 1_073_741_824;
    pub(crate) const TEBI: u128 = 1_099_511_627_776;
}

/// A `Prefix` is essentially a multiplier for an `Atom` within a `Term`; ex.
/// the "c" in "cm" modifies meter by 0.01. The UCUM spec says these should
/// only pertain to metric units, but that rule is not adhered to in
/// `wise_units`.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl UcumSymbol for Prefix {
    fn classification(&self) -> Classification {
        Classification::Si
    }

    fn names(&self) -> Vec<&'static str> {
        match *self {
            Self::Atto => vec!["atto"],
            Self::Centi => vec!["centi"],
            Self::Deci => vec!["deci"],
            Self::Deka => vec!["deka"],
            Self::Exa => vec!["exa"],
            Self::Femto => vec!["femto"],
            Self::Gibi => vec!["gibi"],
            Self::Giga => vec!["giga"],
            Self::Hecto => vec!["hecto"],
            Self::Kibi => vec!["kibi"],
            Self::Kilo => vec!["kilo"],
            Self::Mebi => vec!["mebi"],
            Self::Mega => vec!["mega"],
            Self::Micro => vec!["micro"],
            Self::Milli => vec!["milli"],
            Self::Nano => vec!["nano"],
            Self::Peta => vec!["peta"],
            Self::Pico => vec!["pico"],
            Self::Tebi => vec!["tebi"],
            Self::Tera => vec!["tera"],
            Self::Yocto => vec!["yocto"],
            Self::Yotta => vec!["yotta"],
            Self::Zepto => vec!["zepto"],
            Self::Zetta => vec!["zetta"],
        }
    }

    fn primary_code(&self) -> &'static str {
        match *self {
            Self::Atto => "a",
            Self::Centi => "c",
            Self::Deci => "d",
            Self::Deka => "da",
            Self::Exa => "E",
            Self::Femto => "f",
            Self::Gibi => "Gi",
            Self::Giga => "G",
            Self::Hecto => "h",
            Self::Kibi => "Ki",
            Self::Kilo => "k",
            Self::Mebi => "Mi",
            Self::Mega => "M",
            Self::Micro => "u",
            Self::Milli => "m",
            Self::Nano => "n",
            Self::Peta => "P",
            Self::Pico => "p",
            Self::Tebi => "Ti",
            Self::Tera => "T",
            Self::Yocto => "y",
            Self::Yotta => "Y",
            Self::Zepto => "z",
            Self::Zetta => "Z",
        }
    }

    #[allow(clippy::non_ascii_literal)]
    fn print_symbol(&self) -> Option<&'static str> {
        match *self {
            Self::Micro => Some("μ"),
            _ => Some(self.primary_code()),
        }
    }

    fn secondary_code(&self) -> Option<&'static str> {
        let code = match *self {
            Self::Atto => "A",
            Self::Centi => "C",
            Self::Deci => "D",
            Self::Deka => "DA",
            Self::Exa => "EX",
            Self::Femto => "F",
            Self::Gibi => "GIB",
            Self::Giga => "GA",
            Self::Hecto => "H",
            Self::Kibi => "KIB",
            Self::Kilo => "K",
            Self::Mebi => "MIB",
            Self::Mega => "MA",
            Self::Micro => "U",
            Self::Milli => "M",
            Self::Nano => "N",
            Self::Peta => "PT",
            Self::Pico => "P",
            Self::Tebi => "TIB",
            Self::Tera => "TR",
            Self::Yocto => "YO",
            Self::Yotta => "YA",
            Self::Zepto => "ZO",
            Self::Zetta => "ZA",
        };

        Some(code)
    }

    /// The numeric value that each `Prefix` represents.
    ///
    fn definition_value(&self) -> f64 {
        match *self {
            Self::Atto => ATTO,
            Self::Centi => CENTI,
            Self::Deci => DECI,
            Self::Deka => DEKA,
            Self::Exa => EXA,
            Self::Femto => FEMTO,
            Self::Gibi => GIBI,
            Self::Giga => GIGA,
            Self::Hecto => HECTO,
            Self::Kibi => KIBI,
            Self::Kilo => KILO,
            Self::Mebi => MEBI,
            Self::Mega => MEGA,
            Self::Micro => MICRO,
            Self::Milli => MILLI,
            Self::Nano => NANO,
            Self::Peta => PETA,
            Self::Pico => PICO,
            Self::Tebi => TEBI,
            Self::Tera => TERA,
            Self::Yocto => YOCTO,
            Self::Yotta => YOTTA,
            Self::Zepto => ZEPTO,
            Self::Zetta => ZETTA,
        }
    }

    fn definition_unit(&self) -> Unit {
        unit::UNITY
    }
}

impl Visit<Rule> for Prefix {
    fn visit(pair: Pair<'_, Rule>) -> Result<Self, Error> {
        let prefix = match pair.as_rule() {
            Rule::pri_atto | Rule::sec_atto => Self::Atto,
            Rule::pri_centi | Rule::sec_centi => Self::Centi,
            Rule::pri_deci | Rule::sec_deci => Self::Deci,
            Rule::pri_deka | Rule::sec_deka => Self::Deka,
            Rule::pri_exa | Rule::sec_exa => Self::Exa,
            Rule::pri_femto | Rule::sec_femto => Self::Femto,
            Rule::pri_gibi | Rule::sec_gibi => Self::Gibi,
            Rule::pri_giga | Rule::sec_giga => Self::Giga,
            Rule::pri_hecto | Rule::sec_hecto => Self::Hecto,
            Rule::pri_kibi | Rule::sec_kibi => Self::Kibi,
            Rule::pri_kilo | Rule::sec_kilo => Self::Kilo,
            Rule::pri_mebi | Rule::sec_mebi => Self::Mebi,
            Rule::pri_mega | Rule::sec_mega => Self::Mega,
            Rule::pri_micro | Rule::sec_micro => Self::Micro,
            Rule::pri_milli | Rule::sec_milli => Self::Milli,
            Rule::pri_nano | Rule::sec_nano => Self::Nano,
            Rule::pri_peta | Rule::sec_peta => Self::Peta,
            Rule::pri_tebi | Rule::sec_tebi => Self::Tebi,
            Rule::pri_tera | Rule::sec_tera => Self::Tera,
            Rule::pri_yocto | Rule::sec_yocto => Self::Yocto,
            Rule::pri_yotta | Rule::sec_yotta => Self::Yotta,
            Rule::pri_zepto | Rule::sec_zepto => Self::Zepto,
            Rule::pri_zetta | Rule::sec_zetta => Self::Zetta,
            _ => {
                eprintln!("prefix wat");
                return Err(Error::UnknownUnitString(pair.as_str().to_string()));
            }
        };

        Ok(prefix)
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.primary_code())
    }
}

#[cfg(test)]
mod tests {
    use super::Prefix;
    use crate::parser::ucum_symbol::UcumSymbol;
    use approx::{assert_relative_eq, assert_ulps_eq};

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
        validate_value_atto,  Atto,  1.0e-18;
        validate_value_centi, Centi, 1.0e-2;
        validate_value_deci,  Deci,  1.0e-1;
        validate_value_deka,  Deka,  1.0e1;
        validate_value_exa,   Exa,   1.0e18;
        validate_value_femto, Femto, 1.0e-15;
        validate_value_gibi,  Gibi,  1_073_741_824.0;
        validate_value_giga,  Giga,  1.0e9;
        validate_value_hecto, Hecto, 1.0e2;
        validate_value_kibi,  Kibi,  1024.0;
        validate_value_kilo,  Kilo,  1.0e3;
        validate_value_mebi,  Mebi,  1_048_576.0;
        validate_value_mega,  Mega,  1.0e6;
        validate_value_micro, Micro, 1.0e-6;
        validate_value_milli, Milli, 1.0e-3;
        validate_value_nano,  Nano,  1.0e-9;
        validate_value_peta,  Peta,  1.0e15;
        validate_value_pico,  Pico,  1.0e-12;
        validate_value_tebi,  Tebi,  1_099_511_627_776.0;
        validate_value_tera,  Tera,  1.0e12;
        validate_value_yocto, Yocto, 1.0e-24;
        validate_value_yotta, Yotta, 1.0e24;
        validate_value_zepto, Zepto, 1.0e-21;
        validate_value_zetta, Zetta, 1.0e21;
    );

    #[test]
    fn validate_display() {
        let prefix = Prefix::Kilo;
        assert_eq!(&prefix.to_string(), "k");
    }
}
