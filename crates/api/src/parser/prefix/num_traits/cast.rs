use approx::ulps_eq;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::{parser::prefix, Prefix, UcumSymbol};

impl ToPrimitive for Prefix {
    fn to_u64(&self) -> Option<u64> {
        self.to_u128().as_ref().and_then(ToPrimitive::to_u64)
    }

    fn to_i64(&self) -> Option<i64> {
        self.to_u128().as_ref().and_then(ToPrimitive::to_i64)
    }

    fn to_i128(&self) -> Option<i128> {
        self.to_u128().as_ref().and_then(ToPrimitive::to_i128)
    }

    fn to_u128(&self) -> Option<u128> {
        match self {
            Self::Atto
            | Self::Centi
            | Self::Deci
            | Self::Femto
            | Self::Micro
            | Self::Milli
            | Self::Nano
            | Self::Pico
            | Self::Yocto
            | Self::Zepto => Some(0),
            Self::Deka => Some(prefix::u128::DEKA),
            Self::Exa => Some(prefix::u128::EXA),
            Self::Gibi => Some(prefix::u128::GIBI),
            Self::Giga => Some(prefix::u128::GIGA),
            Self::Hecto => Some(prefix::u128::HECTO),
            Self::Kibi => Some(prefix::u128::KIBI),
            Self::Kilo => Some(prefix::u128::KILO),
            Self::Mebi => Some(prefix::u128::MEBI),
            Self::Mega => Some(prefix::u128::MEGA),
            Self::Peta => Some(prefix::u128::PETA),
            Self::Tebi => Some(prefix::u128::TEBI),
            Self::Tera => Some(prefix::u128::TERA),
            Self::Yotta => Some(prefix::u128::YOTTA),
            Self::Zetta => Some(prefix::u128::ZETTA),
        }
    }

    fn to_f64(&self) -> Option<f64> {
        Some(self.definition_value())
    }
}

impl FromPrimitive for Prefix {
    fn from_u64(n: u64) -> Option<Self> {
        FromPrimitive::from_u128(From::from(n))
    }

    fn from_i64(n: i64) -> Option<Self> {
        FromPrimitive::from_i128(From::from(n))
    }

    #[allow(clippy::cast_sign_loss)]
    fn from_i128(n: i128) -> Option<Self> {
        FromPrimitive::from_u128(n as u128)
    }

    fn from_u128(n: u128) -> Option<Self> {
        match n {
            prefix::u128::DEKA => Some(Self::Deka),
            prefix::u128::HECTO => Some(Self::Hecto),
            prefix::u128::KILO => Some(Self::Kilo),
            prefix::u128::MEGA => Some(Self::Mega),
            prefix::u128::GIGA => Some(Self::Giga),
            prefix::u128::TERA => Some(Self::Tera),
            prefix::u128::PETA => Some(Self::Peta),
            prefix::u128::EXA => Some(Self::Exa),
            prefix::u128::ZETTA => Some(Self::Zetta),
            prefix::u128::YOTTA => Some(Self::Yotta),
            _ => None,
        }
    }

    fn from_f64(n: f64) -> Option<Self> {
        macro_rules! try_translate {
            ($constant:ident => $variant:ident) => {
                if ulps_eq!(n, prefix::$constant) {
                    return Some(Self::$variant);
                }
            };
        }

        try_translate!(DECI => Deci); //   1e-1
        try_translate!(DEKA => Deka); //   1e1
        try_translate!(CENTI => Centi); // 1e-2
        try_translate!(HECTO => Hecto); // 1e2
        try_translate!(MILLI => Milli); // 1e-3
        try_translate!(KILO => Kilo); //   1e3
        try_translate!(MICRO => Micro); // 1e-6
        try_translate!(MEGA => Mega); //   1e6
        try_translate!(NANO => Nano); //   1e-9
        try_translate!(GIGA => Giga); //   1e9
        try_translate!(PICO => Pico); //   1e-12
        try_translate!(TERA => Tera); //   1e12
        try_translate!(FEMTO => Femto); // 1e-15
        try_translate!(PETA => Peta); //   1e15
        try_translate!(ATTO => Atto); //   1e-18
        try_translate!(EXA => Exa); //     1e18
        try_translate!(ZEPTO => Zepto); // 1e-21
        try_translate!(ZETTA => Zetta); // 1e21
        try_translate!(YOCTO => Yocto); // 1e-24
        try_translate!(YOTTA => Yotta); // 1e24

        try_translate!(KIBI => Kibi);
        try_translate!(MEBI => Mebi);
        try_translate!(GIBI => Gibi);
        try_translate!(TEBI => Tebi);

        None
    }
}
