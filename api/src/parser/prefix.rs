pub mod ucum_symbol;

#[cfg(feature = "v2")]
mod v2;

use super::{Error, Visit};
use crate::{
    parser::{symbols::symbol_parser::Rule, ucum_symbol::UcumSymbol},
    Composable,
};
use pest::iterators::Pair;
use std::fmt;

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

impl Composable for Prefix {
    fn composition(&self) -> crate::Composition {
        crate::Composition::new_dimless()
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
