use super::{exponent, simple_unit};
use crate::{
    atom_collection::AtomCollection,
    parse::{Parse, ParseResult},
    prefix_collection::PrefixCollection,
    tokens::Annotatable,
};

pub(super) struct Parser;

impl<'input> Parse<'input, Annotatable> for Parser {
    fn parse(
        input: &'input str,
        prefixes: &'static PrefixCollection,
        atoms: &'static AtomCollection,
    ) -> ParseResult<'input, Annotatable> {
        let (simple_unit, su_tail) =
            simple_unit::Parser::parse(input, prefixes, atoms).map_err(|_| input)?;

        let (exponent, tail) = exponent::parse(su_tail)
            .map_or_else(|_| (None, su_tail), |(e, e_tail)| (Some(e), e_tail));

        Ok((
            Annotatable {
                simple_unit,
                exponent,
            },
            tail,
        ))
    }
}

pub(super) fn parse_atom_first<'input>(
    input: &'input str,
    atoms: &'static AtomCollection,
) -> ParseResult<'input, Annotatable> {
    let (simple_unit, su_tail) = simple_unit::parse_atom_first(input, atoms).map_err(|_| input)?;

    let (exponent, tail) =
        exponent::parse(su_tail).map_or_else(|_| (None, su_tail), |(e, e_tail)| (Some(e), e_tail));

    Ok((
        Annotatable {
            simple_unit,
            exponent,
        },
        tail,
    ))
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{
        base_units,
        prefixes::{self, PREFIXES},
        tokens::{Annotatable, AtomSymbol, SimpleUnit},
        units::UNITS,
        Parse,
    };

    #[test]
    fn test_parse_2km() {
        assert_eq!(
            Parser::parse("2km", &PREFIXES, &UNITS),
            Ok((
                Annotatable {
                    simple_unit: SimpleUnit {
                        factor: Some(2),
                        prefix_symbol: Some(&prefixes::KILO),
                        atom_symbol: &AtomSymbol::Meter(base_units::Meter),
                    },
                    exponent: None
                },
                ""
            ))
        );
    }

    #[test]
    fn test_parse_m() {
        assert_eq!(
            Parser::parse("m", &PREFIXES, &UNITS),
            Ok((
                Annotatable {
                    simple_unit: SimpleUnit {
                        factor: None,
                        prefix_symbol: None,
                        atom_symbol: &AtomSymbol::Meter(base_units::Meter)
                    },
                    exponent: None
                },
                ""
            ))
        );
    }

    #[test]
    fn test_parse_2km2() {
        assert_eq!(
            Parser::parse("2km2", &PREFIXES, &UNITS),
            Ok((
                Annotatable {
                    simple_unit: SimpleUnit {
                        factor: Some(2),
                        prefix_symbol: Some(&prefixes::KILO),
                        atom_symbol: &AtomSymbol::Meter(base_units::Meter)
                    },
                    exponent: Some(2)
                },
                ""
            ))
        );
    }

    #[test]
    fn test_parse_m2() {
        assert_eq!(
            Parser::parse("m2", &PREFIXES, &UNITS),
            Ok((
                Annotatable {
                    simple_unit: SimpleUnit {
                        factor: None,
                        prefix_symbol: None,
                        atom_symbol: &AtomSymbol::Meter(base_units::Meter)
                    },
                    exponent: Some(2)
                },
                ""
            ))
        );
    }

    #[test]
    fn test_parse_2km_and_garbage() {
        assert_eq!(
            Parser::parse("2km!@#", &PREFIXES, &UNITS),
            Ok((
                Annotatable {
                    simple_unit: SimpleUnit {
                        factor: Some(2),
                        prefix_symbol: Some(&prefixes::KILO),
                        atom_symbol: &AtomSymbol::Meter(base_units::Meter)
                    },
                    exponent: None
                },
                "!@#"
            ))
        );
    }

    #[test]
    fn test_parse_m_and_garbage() {
        assert_eq!(
            Parser::parse("m!@#", &PREFIXES, &UNITS),
            Ok((
                Annotatable {
                    simple_unit: SimpleUnit {
                        factor: None,
                        prefix_symbol: None,
                        atom_symbol: &AtomSymbol::Meter(base_units::Meter)
                    },
                    exponent: None
                },
                "!@#"
            ))
        );

        assert_eq!(
            Parser::parse("2km2!@#", &PREFIXES, &UNITS),
            Ok((
                Annotatable {
                    simple_unit: SimpleUnit {
                        factor: Some(2),
                        prefix_symbol: Some(&prefixes::KILO),
                        atom_symbol: &AtomSymbol::Meter(base_units::Meter)
                    },
                    exponent: Some(2)
                },
                "!@#"
            ))
        );

        assert_eq!(
            Parser::parse("m2!@#", &PREFIXES, &UNITS),
            Ok((
                Annotatable {
                    simple_unit: SimpleUnit {
                        factor: None,
                        prefix_symbol: None,
                        atom_symbol: &AtomSymbol::Meter(base_units::Meter)
                    },
                    exponent: Some(2)
                },
                "!@#"
            ))
        );

        assert_eq!(Parser::parse("2", &PREFIXES, &UNITS), Err("2"));
        assert_eq!(Parser::parse("", &PREFIXES, &UNITS), Err(""));
    }
}
