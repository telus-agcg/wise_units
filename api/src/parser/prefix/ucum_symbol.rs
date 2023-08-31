use crate::{Classification, Prefix, UcumSymbol, Unit};

pub const ATTO: f64 = 1.0e-18;
pub const CENTI: f64 = 1.0e-2;
pub const DECI: f64 = 1.0e-1;
pub const DEKA: f64 = 1.0e1;
pub const EXA: f64 = 1.0e18;
pub const FEMTO: f64 = 1.0e-15;
pub const GIBI: f64 = 1_073_741_824.0;
pub const GIGA: f64 = 1.0e9;
pub const HECTO: f64 = 1.0e2;
pub const KIBI: f64 = 1024.0;
pub const KILO: f64 = 1.0e3;
pub const MEBI: f64 = 1_048_576.0;
pub const MEGA: f64 = 1.0e6;
pub const MICRO: f64 = 1.0e-6;
pub const MILLI: f64 = 1.0e-3;
pub const NANO: f64 = 1.0e-9;
pub const PETA: f64 = 1.0e15;
pub const PICO: f64 = 1.0e-12;
pub const TEBI: f64 = 1_099_511_627_776.0;
pub const TERA: f64 = 1.0e12;
pub const YOCTO: f64 = 1.0e-24;
pub const YOTTA: f64 = 1.0e24;
pub const ZEPTO: f64 = 1.0e-21;
pub const ZETTA: f64 = 1.0e21;

impl UcumSymbol for Prefix {
    #[inline]
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
        Unit::new_unity()
    }
}
