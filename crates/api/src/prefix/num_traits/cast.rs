use num_traits::{FromPrimitive, ToPrimitive, Zero};

use crate::{prefix, Prefix, UcumSymbol};

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
            Self::Deci
            | Self::Centi
            | Self::Milli
            | Self::Micro
            | Self::Nano
            | Self::Pico
            | Self::Femto
            | Self::Atto
            | Self::Zepto
            | Self::Yocto => Some(0),
            Self::Deka => Some(prefix::u128::DEKA),   // 1e1
            Self::Hecto => Some(prefix::u128::HECTO), // 1e2
            Self::Kilo => Some(prefix::u128::KILO),   // 1e3
            Self::Mega => Some(prefix::u128::MEGA),   // 1e6
            Self::Giga => Some(prefix::u128::GIGA),   // 1e9
            Self::Tera => Some(prefix::u128::TERA),   // 1e12
            Self::Peta => Some(prefix::u128::PETA),   // 1e15
            Self::Exa => Some(prefix::u128::EXA),     // 1e18
            Self::Zetta => Some(prefix::u128::ZETTA), // 1e21
            Self::Yotta => Some(prefix::u128::YOTTA), // 1e24

            Self::Kibi => Some(prefix::u128::KIBI), // 1K
            Self::Mebi => Some(prefix::u128::MEBI), // 1M
            Self::Gibi => Some(prefix::u128::GIBI), // 1G
            Self::Tebi => Some(prefix::u128::TEBI), // 1T
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

    fn from_i128(n: i128) -> Option<Self> {
        u128::try_from(n).map_or(None, FromPrimitive::from_u128)
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
                // We use YOCTO here as the epsilon to be able to determine the difference between
                // 0.0 and YOCTO; using std::f64::EPSILON doesn't not give enough precision.
                if approx::ulps_eq!(n, prefix::$constant, epsilon = prefix::YOCTO) {
                    return Some(Self::$variant);
                }
            };
        }

        if n.is_zero() {
            return None;
        }

        try_translate!(DEKA => Deka); //   1e1
        try_translate!(HECTO => Hecto); // 1e2
        try_translate!(KILO => Kilo); //   1e3
        try_translate!(MEGA => Mega); //   1e6
        try_translate!(GIGA => Giga); //   1e9
        try_translate!(TERA => Tera); //   1e12
        try_translate!(PETA => Peta); //   1e15
        try_translate!(EXA => Exa); //     1e18
        try_translate!(ZETTA => Zetta); // 1e21
        try_translate!(YOTTA => Yotta); // 1e24

        try_translate!(YOCTO => Yocto); // 1e-24
        try_translate!(ZEPTO => Zepto); // 1e-21
        try_translate!(ATTO => Atto); //   1e-18
        try_translate!(FEMTO => Femto); // 1e-15
        try_translate!(PICO => Pico); //   1e-12
        try_translate!(NANO => Nano); //   1e-9
        try_translate!(MICRO => Micro); // 1e-6
        try_translate!(MILLI => Milli); // 1e-3
        try_translate!(CENTI => Centi); // 1e-2
        try_translate!(DECI => Deci); //   1e-1

        try_translate!(KIBI => Kibi);
        try_translate!(MEBI => Mebi);
        try_translate!(GIBI => Gibi);
        try_translate!(TEBI => Tebi);

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod to_primitive {
        use approx::assert_ulps_eq;

        use super::*;

        #[test]
        fn to_u128_test() {
            assert_eq!(Prefix::Atto.to_u128().unwrap(), 0);
            assert_eq!(Prefix::Centi.to_u128().unwrap(), 0);
            assert_eq!(Prefix::Deci.to_u128().unwrap(), 0);
            assert_eq!(Prefix::Femto.to_u128().unwrap(), 0);
            assert_eq!(Prefix::Micro.to_u128().unwrap(), 0);
            assert_eq!(Prefix::Milli.to_u128().unwrap(), 0);
            assert_eq!(Prefix::Nano.to_u128().unwrap(), 0);
            assert_eq!(Prefix::Pico.to_u128().unwrap(), 0);
            assert_eq!(Prefix::Yocto.to_u128().unwrap(), 0);
            assert_eq!(Prefix::Zepto.to_u128().unwrap(), 0);

            assert_eq!(Prefix::Deka.to_u128().unwrap(), 10);
            assert_eq!(Prefix::Hecto.to_u128().unwrap(), 100);
            assert_eq!(Prefix::Kilo.to_u128().unwrap(), 1000);
            assert_eq!(Prefix::Mega.to_u128().unwrap(), 1_000_000);
            assert_eq!(Prefix::Giga.to_u128().unwrap(), 1_000_000_000);
            assert_eq!(Prefix::Tera.to_u128().unwrap(), 1_000_000_000_000);
            assert_eq!(Prefix::Peta.to_u128().unwrap(), 1_000_000_000_000_000);
            assert_eq!(Prefix::Exa.to_u128().unwrap(), 1_000_000_000_000_000_000);
            assert_eq!(
                Prefix::Zetta.to_u128().unwrap(),
                1_000_000_000_000_000_000_000
            );
            assert_eq!(
                Prefix::Yotta.to_u128().unwrap(),
                1_000_000_000_000_000_000_000_000
            );

            assert_eq!(Prefix::Kibi.to_u128().unwrap(), 1024);
            assert_eq!(Prefix::Mebi.to_u128().unwrap(), 1_048_576);
            assert_eq!(Prefix::Gibi.to_u128().unwrap(), 1_073_741_824);
            assert_eq!(Prefix::Tebi.to_u128().unwrap(), 1_099_511_627_776);
        }

        #[test]
        fn to_i128_test() {
            assert_eq!(Prefix::Atto.to_i128().unwrap(), 0);
            assert_eq!(Prefix::Centi.to_i128().unwrap(), 0);
            assert_eq!(Prefix::Deci.to_i128().unwrap(), 0);
            assert_eq!(Prefix::Femto.to_i128().unwrap(), 0);
            assert_eq!(Prefix::Micro.to_i128().unwrap(), 0);
            assert_eq!(Prefix::Milli.to_i128().unwrap(), 0);
            assert_eq!(Prefix::Nano.to_i128().unwrap(), 0);
            assert_eq!(Prefix::Pico.to_i128().unwrap(), 0);
            assert_eq!(Prefix::Yocto.to_i128().unwrap(), 0);
            assert_eq!(Prefix::Zepto.to_i128().unwrap(), 0);

            assert_eq!(Prefix::Deka.to_i128().unwrap(), 10);
            assert_eq!(Prefix::Hecto.to_i128().unwrap(), 100);
            assert_eq!(Prefix::Kilo.to_i128().unwrap(), 1000);
            assert_eq!(Prefix::Mega.to_i128().unwrap(), 1_000_000);
            assert_eq!(Prefix::Giga.to_i128().unwrap(), 1_000_000_000);
            assert_eq!(Prefix::Tera.to_i128().unwrap(), 1_000_000_000_000);
            assert_eq!(Prefix::Peta.to_i128().unwrap(), 1_000_000_000_000_000);
            assert_eq!(Prefix::Exa.to_i128().unwrap(), 1_000_000_000_000_000_000);
            assert_eq!(
                Prefix::Zetta.to_i128().unwrap(),
                1_000_000_000_000_000_000_000
            );
            assert_eq!(
                Prefix::Yotta.to_i128().unwrap(),
                1_000_000_000_000_000_000_000_000
            );

            assert_eq!(Prefix::Kibi.to_i128().unwrap(), 1024);
            assert_eq!(Prefix::Mebi.to_i128().unwrap(), 1_048_576);
            assert_eq!(Prefix::Gibi.to_i128().unwrap(), 1_073_741_824);
            assert_eq!(Prefix::Tebi.to_i128().unwrap(), 1_099_511_627_776);
        }

        #[test]
        fn to_u64_test() {
            assert_eq!(Prefix::Atto.to_u64().unwrap(), 0);
            assert_eq!(Prefix::Centi.to_u64().unwrap(), 0);
            assert_eq!(Prefix::Deci.to_u64().unwrap(), 0);
            assert_eq!(Prefix::Femto.to_u64().unwrap(), 0);
            assert_eq!(Prefix::Micro.to_u64().unwrap(), 0);
            assert_eq!(Prefix::Milli.to_u64().unwrap(), 0);
            assert_eq!(Prefix::Nano.to_u64().unwrap(), 0);
            assert_eq!(Prefix::Pico.to_u64().unwrap(), 0);
            assert_eq!(Prefix::Yocto.to_u64().unwrap(), 0);
            assert_eq!(Prefix::Zepto.to_u64().unwrap(), 0);

            assert_eq!(Prefix::Deka.to_u64().unwrap(), 10);
            assert_eq!(Prefix::Hecto.to_u64().unwrap(), 100);
            assert_eq!(Prefix::Kilo.to_u64().unwrap(), 1000);
            assert_eq!(Prefix::Mega.to_u64().unwrap(), 1_000_000);
            assert_eq!(Prefix::Giga.to_u64().unwrap(), 1_000_000_000);
            assert_eq!(Prefix::Tera.to_u64().unwrap(), 1_000_000_000_000);
            assert_eq!(Prefix::Peta.to_u64().unwrap(), 1_000_000_000_000_000);
            assert_eq!(Prefix::Exa.to_u64().unwrap(), 1_000_000_000_000_000_000);
            assert!(Prefix::Zetta.to_u64().is_none(),);
            assert!(Prefix::Yotta.to_u64().is_none(),);

            assert_eq!(Prefix::Kibi.to_u64().unwrap(), 1024);
            assert_eq!(Prefix::Mebi.to_u64().unwrap(), 1_048_576);
            assert_eq!(Prefix::Gibi.to_u64().unwrap(), 1_073_741_824);
            assert_eq!(Prefix::Tebi.to_u64().unwrap(), 1_099_511_627_776);
        }

        #[test]
        fn to_i64_test() {
            assert_eq!(Prefix::Atto.to_i64().unwrap(), 0);
            assert_eq!(Prefix::Centi.to_i64().unwrap(), 0);
            assert_eq!(Prefix::Deci.to_i64().unwrap(), 0);
            assert_eq!(Prefix::Femto.to_i64().unwrap(), 0);
            assert_eq!(Prefix::Micro.to_i64().unwrap(), 0);
            assert_eq!(Prefix::Milli.to_i64().unwrap(), 0);
            assert_eq!(Prefix::Nano.to_i64().unwrap(), 0);
            assert_eq!(Prefix::Pico.to_i64().unwrap(), 0);
            assert_eq!(Prefix::Yocto.to_i64().unwrap(), 0);
            assert_eq!(Prefix::Zepto.to_i64().unwrap(), 0);

            assert_eq!(Prefix::Deka.to_i64().unwrap(), 10);
            assert_eq!(Prefix::Hecto.to_i64().unwrap(), 100);
            assert_eq!(Prefix::Kilo.to_i64().unwrap(), 1000);
            assert_eq!(Prefix::Mega.to_i64().unwrap(), 1_000_000);
            assert_eq!(Prefix::Giga.to_i64().unwrap(), 1_000_000_000);
            assert_eq!(Prefix::Tera.to_i64().unwrap(), 1_000_000_000_000);
            assert_eq!(Prefix::Peta.to_i64().unwrap(), 1_000_000_000_000_000);
            assert_eq!(Prefix::Exa.to_i64().unwrap(), 1_000_000_000_000_000_000);
            assert!(Prefix::Zetta.to_i64().is_none(),);
            assert!(Prefix::Yotta.to_i64().is_none(),);

            assert_eq!(Prefix::Kibi.to_i64().unwrap(), 1024);
            assert_eq!(Prefix::Mebi.to_i64().unwrap(), 1_048_576);
            assert_eq!(Prefix::Gibi.to_i64().unwrap(), 1_073_741_824);
            assert_eq!(Prefix::Tebi.to_i64().unwrap(), 1_099_511_627_776);
        }

        #[test]
        fn to_f64_test() {
            assert_ulps_eq!(Prefix::Deci.to_f64().unwrap(), 1e-1);
            assert_ulps_eq!(Prefix::Centi.to_f64().unwrap(), 1e-2);
            assert_ulps_eq!(Prefix::Milli.to_f64().unwrap(), 1e-3);
            assert_ulps_eq!(Prefix::Micro.to_f64().unwrap(), 1e-6);
            assert_ulps_eq!(Prefix::Nano.to_f64().unwrap(), 1e-9);
            assert_ulps_eq!(Prefix::Pico.to_f64().unwrap(), 1e-12);
            assert_ulps_eq!(Prefix::Femto.to_f64().unwrap(), 1e-15);
            assert_ulps_eq!(Prefix::Atto.to_f64().unwrap(), 1e-18);
            assert_ulps_eq!(Prefix::Zepto.to_f64().unwrap(), 1e-21);
            assert_ulps_eq!(Prefix::Yocto.to_f64().unwrap(), 1e-24);

            assert_ulps_eq!(Prefix::Deka.to_f64().unwrap(), 1e1);
            assert_ulps_eq!(Prefix::Hecto.to_f64().unwrap(), 1e2);
            assert_ulps_eq!(Prefix::Kilo.to_f64().unwrap(), 1e3);
            assert_ulps_eq!(Prefix::Mega.to_f64().unwrap(), 1e6);
            assert_ulps_eq!(Prefix::Giga.to_f64().unwrap(), 1e9);
            assert_ulps_eq!(Prefix::Tera.to_f64().unwrap(), 1e12);
            assert_ulps_eq!(Prefix::Peta.to_f64().unwrap(), 1e15);
            assert_ulps_eq!(Prefix::Exa.to_f64().unwrap(), 1e18);
            assert_ulps_eq!(Prefix::Zetta.to_f64().unwrap(), 1e21);
            assert_ulps_eq!(Prefix::Yotta.to_f64().unwrap(), 1e24);

            assert_ulps_eq!(Prefix::Kibi.to_f64().unwrap(), 1024.0);
            assert_ulps_eq!(Prefix::Mebi.to_f64().unwrap(), 1_048_576.0);
            assert_ulps_eq!(Prefix::Gibi.to_f64().unwrap(), 1_073_741_824.0);
            assert_ulps_eq!(Prefix::Tebi.to_f64().unwrap(), 1_099_511_627_776.0);
        }
    } /* to_primitive */

    mod from_primitive {
        use super::*;

        #[test]
        fn from_u128_test() {
            assert!(Prefix::from_u128(0).is_none());

            assert_eq!(Prefix::from_u128(prefix::u128::DEKA).unwrap(), Prefix::Deka);
            assert_eq!(
                Prefix::from_u128(prefix::u128::HECTO).unwrap(),
                Prefix::Hecto
            );
            assert_eq!(Prefix::from_u128(prefix::u128::KILO).unwrap(), Prefix::Kilo);
            assert_eq!(Prefix::from_u128(prefix::u128::MEGA).unwrap(), Prefix::Mega);
            assert_eq!(Prefix::from_u128(prefix::u128::GIGA).unwrap(), Prefix::Giga);
            assert_eq!(Prefix::from_u128(prefix::u128::TERA).unwrap(), Prefix::Tera);
            assert_eq!(Prefix::from_u128(prefix::u128::PETA).unwrap(), Prefix::Peta);
            assert_eq!(Prefix::from_u128(prefix::u128::EXA).unwrap(), Prefix::Exa);
            assert_eq!(
                Prefix::from_u128(prefix::u128::ZETTA).unwrap(),
                Prefix::Zetta
            );
            assert_eq!(
                Prefix::from_u128(prefix::u128::YOTTA).unwrap(),
                Prefix::Yotta
            );
        }

        #[test]
        #[allow(clippy::cast_possible_wrap)]
        fn from_i128_test() {
            assert!(Prefix::from_i128(-1).is_none());

            assert_eq!(
                Prefix::from_i128(prefix::u128::YOTTA as i128).unwrap(),
                Prefix::Yotta
            );
        }

        #[test]
        fn from_f64_test() {
            assert!(Prefix::from_f64(0.0).is_none());

            assert_eq!(Prefix::from_f64(prefix::DECI).unwrap(), Prefix::Deci);
            assert_eq!(Prefix::from_f64(prefix::CENTI).unwrap(), Prefix::Centi);
            assert_eq!(Prefix::from_f64(prefix::MILLI).unwrap(), Prefix::Milli);
            assert_eq!(Prefix::from_f64(prefix::MICRO).unwrap(), Prefix::Micro);
            assert_eq!(Prefix::from_f64(prefix::NANO).unwrap(), Prefix::Nano);
            assert_eq!(Prefix::from_f64(prefix::PICO).unwrap(), Prefix::Pico);
            assert_eq!(Prefix::from_f64(prefix::FEMTO).unwrap(), Prefix::Femto);
            assert_eq!(Prefix::from_f64(prefix::ATTO).unwrap(), Prefix::Atto);
            assert_eq!(Prefix::from_f64(prefix::ZEPTO).unwrap(), Prefix::Zepto);
            assert_eq!(Prefix::from_f64(prefix::YOCTO).unwrap(), Prefix::Yocto);

            assert_eq!(Prefix::from_f64(prefix::DEKA).unwrap(), Prefix::Deka);
            assert_eq!(Prefix::from_f64(prefix::HECTO).unwrap(), Prefix::Hecto);
            assert_eq!(Prefix::from_f64(prefix::KILO).unwrap(), Prefix::Kilo);
            assert_eq!(Prefix::from_f64(prefix::MEGA).unwrap(), Prefix::Mega);
            assert_eq!(Prefix::from_f64(prefix::GIGA).unwrap(), Prefix::Giga);
            assert_eq!(Prefix::from_f64(prefix::TERA).unwrap(), Prefix::Tera);
            assert_eq!(Prefix::from_f64(prefix::PETA).unwrap(), Prefix::Peta);
            assert_eq!(Prefix::from_f64(prefix::EXA).unwrap(), Prefix::Exa);
            assert_eq!(Prefix::from_f64(prefix::ZETTA).unwrap(), Prefix::Zetta);
            assert_eq!(Prefix::from_f64(prefix::YOTTA).unwrap(), Prefix::Yotta);
        }
    } /* from_primitive */
}
