use super::{atom_symbol, factor, prefix_symbol};
use crate::{
    atom_collection::AtomCollection,
    parse::{Parse, ParseResult, ParseSymbol},
    prefix_collection::PrefixCollection,
    tokens::SimpleUnit,
};

pub(super) struct Parser;

impl<'input> Parse<'input, SimpleUnit> for Parser {
    fn parse(
        input: &'input str,
        prefixes: &'static PrefixCollection,
        atoms: &'static AtomCollection,
    ) -> ParseResult<'input, SimpleUnit> {
        let (mut factor, factor_tail) =
            factor::parse(input).map_or_else(|_| (None, input), |(f, t)| (Some(f), t));

        let (mut prefix_symbol, prefix_tail) =
            prefix_symbol::Parser::parse_symbol(factor_tail, prefixes)
                .map_or_else(|_| (None, factor_tail), |(ps, t)| (Some(ps), t));

        let (atom_symbol, tail) = atom_symbol::Parser::parse_symbol(prefix_tail, atoms)
            .or_else(|_| {
                if prefix_symbol.is_some() {
                    prefix_symbol = None;

                    atom_symbol::Parser::parse_symbol(factor_tail, atoms).map_err(|_| input)
                } else {
                    Err(input)
                }
            })
            .or_else(|_| {
                if factor.is_some() {
                    factor = None;

                    atom_symbol::Parser::parse_symbol(input, atoms).map_err(|_| input)
                } else {
                    Err(input)
                }
            })?;

        Ok((
            SimpleUnit {
                factor,
                prefix_symbol,
                atom_symbol,
            },
            tail,
        ))
    }
}

pub(super) fn parse_atom_first<'input>(
    input: &'input str,
    atoms: &'static AtomCollection,
) -> ParseResult<'input, SimpleUnit> {
    let (atom_symbol, tail) = atom_symbol::Parser::parse_symbol(input, atoms).map_err(|_| input)?;

    Ok((
        SimpleUnit {
            factor: None,
            prefix_symbol: None,
            atom_symbol,
        },
        tail,
    ))
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{
        base_units, derived_units,
        prefixes::{self, PREFIXES},
        tokens::{AtomSymbol, SimpleUnit},
        units::UNITS,
        Parse,
    };

    #[test]
    fn test_parse_2km_and_garbage() {
        assert_eq!(
            Parser::parse("2km!@#$", &PREFIXES, &UNITS),
            Ok((
                SimpleUnit {
                    factor: Some(2),
                    prefix_symbol: Some(&prefixes::KILO),
                    atom_symbol: &AtomSymbol::Meter(base_units::Meter),
                },
                "!@#$"
            ))
        );
    }

    #[test]
    fn test_parse_2m_and_garbage() {
        assert_eq!(
            Parser::parse("2m!@#$", &PREFIXES, &UNITS),
            Ok((
                SimpleUnit {
                    factor: Some(2),
                    prefix_symbol: None,
                    atom_symbol: &AtomSymbol::Meter(base_units::Meter),
                },
                "!@#$"
            ))
        );
    }

    #[test]
    fn test_parse_m_and_garbage() {
        assert_eq!(
            Parser::parse("m!#@$", &PREFIXES, &UNITS),
            Ok((
                SimpleUnit {
                    factor: None,
                    prefix_symbol: None,
                    atom_symbol: &AtomSymbol::Meter(base_units::Meter),
                },
                "!#@$"
            ))
        );
    }

    #[test]
    fn test_parse_m123() {
        assert_eq!(
            Parser::parse("m123", &PREFIXES, &UNITS),
            Ok((
                SimpleUnit {
                    factor: None,
                    prefix_symbol: None,
                    atom_symbol: &AtomSymbol::Meter(base_units::Meter),
                },
                "123"
            ))
        );
    }

    #[allow(nonstandard_style)]
    #[test]
    fn test_parse_2Macr_us_and_garbage() {
        assert_eq!(
            Parser::parse("2M[acr_us]!@#$", &PREFIXES, &UNITS),
            Ok((
                SimpleUnit {
                    factor: Some(2),
                    prefix_symbol: Some(&prefixes::MEGA),
                    atom_symbol: &AtomSymbol::AcreAcrUs(derived_units::AcreAcrUs),
                },
                "!@#$"
            ))
        );
    }

    #[test]
    fn test_parse_2acr_us_and_garbage() {
        assert_eq!(
            Parser::parse("2[acr_us]!@#$", &PREFIXES, &UNITS),
            Ok((
                SimpleUnit {
                    factor: Some(2),
                    prefix_symbol: None,
                    atom_symbol: &AtomSymbol::AcreAcrUs(derived_units::AcreAcrUs),
                },
                "!@#$"
            ))
        );
    }

    #[test]
    fn test_parse_acr_us_and_garbage() {
        assert_eq!(
            Parser::parse("[acr_us]!#@$", &PREFIXES, &UNITS),
            Ok((
                SimpleUnit {
                    factor: None,
                    prefix_symbol: None,
                    atom_symbol: &AtomSymbol::AcreAcrUs(derived_units::AcreAcrUs),
                },
                "!#@$"
            ))
        );
    }

    #[test]
    fn test_parse_acr_us123() {
        assert_eq!(
            Parser::parse("[acr_us]123", &PREFIXES, &UNITS),
            Ok((
                SimpleUnit {
                    factor: None,
                    prefix_symbol: None,
                    atom_symbol: &AtomSymbol::AcreAcrUs(derived_units::AcreAcrUs),
                },
                "123"
            ))
        );
    }

    #[test]
    fn test_parse_10star123() {
        assert_eq!(
            Parser::parse("10*123", &PREFIXES, &UNITS),
            Ok((
                SimpleUnit {
                    factor: None,
                    prefix_symbol: None,
                    atom_symbol: &AtomSymbol::TheNumberTenForArbitraryPowers10star(
                        derived_units::TheNumberTenForArbitraryPowers10star
                    ),
                },
                "123"
            ))
        );
    }

    #[test]
    fn test_parse_10caret123() {
        assert_eq!(
            Parser::parse("10^123", &PREFIXES, &UNITS),
            Ok((
                SimpleUnit {
                    factor: None,
                    prefix_symbol: None,
                    atom_symbol: &AtomSymbol::TheNumberTenForArbitraryPowers10caret(
                        derived_units::TheNumberTenForArbitraryPowers10caret
                    ),
                },
                "123"
            ))
        );

        assert_eq!(Parser::parse("0", &PREFIXES, &UNITS), Err("0"));
        assert_eq!(Parser::parse("", &PREFIXES, &UNITS), Err(""));
    }
}
