use crate::{
    v2::{behavior_traits, type_traits::StaticPrefix},
    Prefix, UcumSymbol,
};

impl StaticPrefix<f64> for Prefix {
    fn primary_code(&self) -> &'static str {
        // Just delegate to existing implementation.
        UcumSymbol::primary_code(self)
    }

    fn secondary_code(&self) -> &'static str {
        match *self {
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
        }
    }

    fn name(&self) -> &'static str {
        match *self {
            Self::Atto => "atto",
            Self::Centi => "centi",
            Self::Deci => "deci",
            Self::Deka => "deka",
            Self::Exa => "exa",
            Self::Femto => "femto",
            Self::Gibi => "gibi",
            Self::Giga => "giga",
            Self::Hecto => "hecto",
            Self::Kibi => "kibi",
            Self::Kilo => "kilo",
            Self::Mebi => "mebi",
            Self::Mega => "mega",
            Self::Micro => "micro",
            Self::Milli => "milli",
            Self::Nano => "nano",
            Self::Peta => "peta",
            Self::Pico => "pico",
            Self::Tebi => "tebi",
            Self::Tera => "tera",
            Self::Yocto => "yocto",
            Self::Yotta => "yotta",
            Self::Zepto => "zepto",
            Self::Zetta => "zetta",
        }
    }

    fn print_symbol(&self) -> &'static str {
        match *self {
            Self::Micro => "μ",
            _ => <Self as StaticPrefix<_>>::primary_code(&self),
        }
    }
}

impl behavior_traits::convert::ToScalar<f64> for Prefix {
    fn to_scalar(&self) -> f64 {
        Self::definition_value(self)
    }
}
