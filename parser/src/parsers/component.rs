use super::{annotatable, annotation, factor, term};
use crate::{
    atom_collection::AtomCollection,
    parse::{Parse, ParseResult},
    prefix_collection::PrefixCollection,
    tokens::Component,
};

pub(super) struct Parser;

impl<'input> Parse<'input, Component<'input>> for Parser {
    fn parse(
        input: &'input str,
        prefixes: &'static PrefixCollection,
        atoms: &'static AtomCollection,
    ) -> ParseResult<'input, Component<'input>> {
        if let Ok((annotatable, ann_tail)) = annotatable::Parser::parse(input, prefixes, atoms) {
            if let Ok((annotation, tail)) = annotation::parse(ann_tail) {
                return Ok((
                    Component::AnnotatedAnnotatable(annotatable, annotation),
                    tail,
                ));
            }
            return Ok((Component::Annotatable(annotatable), ann_tail));
        }

        if let Ok((annotation, tail)) = annotation::parse(input) {
            return Ok((Component::Annotation(annotation), tail));
        }

        if let Ok((factor, tail)) = factor::parse(input) {
            return Ok((Component::Factor(factor), tail));
        }

        parse_nested_term(input, prefixes, atoms)
    }
}

pub fn parse_atom_first<'input>(
    input: &'input str,
    prefixes: &'static PrefixCollection,
    atoms: &'static AtomCollection,
) -> ParseResult<'input, Component<'input>> {
    if let Ok((annotatable, ann_tail)) = annotatable::parse_atom_first(input, atoms) {
        if let Ok((annotation, tail)) = annotation::parse(ann_tail) {
            return Ok((
                Component::AnnotatedAnnotatable(annotatable, annotation),
                tail,
            ));
        }
        return Ok((Component::Annotatable(annotatable), ann_tail));
    }

    if let Ok((annotation, tail)) = annotation::parse(input) {
        return Ok((Component::Annotation(annotation), tail));
    }

    if let Ok((factor, tail)) = factor::parse(input) {
        return Ok((Component::Factor(factor), tail));
    }

    parse_nested_term(input, prefixes, atoms)
}

fn parse_nested_term<'input>(
    input: &'input str,
    prefixes: &'static PrefixCollection,
    atoms: &'static AtomCollection,
) -> ParseResult<'input, Component<'input>> {
    if let Some(stripped_from_open_paren) = input.strip_prefix('(') {
        let (term, tail) =
            term::Parser::parse(stripped_from_open_paren, prefixes, atoms).map_err(|_| input)?;

        tail.strip_prefix(')').map_or_else(
            || Err(input),
            |stripped_from_closed_paren| {
                Ok((Component::NestedTerm(term), stripped_from_closed_paren))
            },
        )
    } else {
        Err(input)
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{
        base_units, derived_units,
        prefixes::{self, PREFIXES},
        tokens::{Annotatable, Annotation, AtomSymbol, Component, SimpleUnit, Term},
        units::UNITS,
        Parse,
    };

    #[test]
    fn test_parse_annotated_annotation() {
        assert_eq!(
            Parser::parse("2km2{things}", &PREFIXES, &UNITS),
            Ok((
                Component::AnnotatedAnnotatable(
                    Annotatable {
                        simple_unit: SimpleUnit {
                            factor: Some(2),
                            prefix_symbol: Some(&prefixes::KILO),
                            atom_symbol: &AtomSymbol::Meter(base_units::Meter)
                        },
                        exponent: Some(2)
                    },
                    Annotation("things")
                ),
                ""
            ))
        );
        assert_eq!(
            Parser::parse("2km2{things}stuff", &PREFIXES, &UNITS),
            Ok((
                Component::AnnotatedAnnotatable(
                    Annotatable {
                        simple_unit: SimpleUnit {
                            factor: Some(2),
                            prefix_symbol: Some(&prefixes::KILO),
                            atom_symbol: &AtomSymbol::Meter(base_units::Meter)
                        },
                        exponent: Some(2)
                    },
                    Annotation("things")
                ),
                "stuff"
            ))
        );
        assert_eq!(
            Parser::parse("2M[acr_us]2{things}", &PREFIXES, &UNITS),
            Ok((
                Component::AnnotatedAnnotatable(
                    Annotatable {
                        simple_unit: SimpleUnit {
                            factor: Some(2),
                            prefix_symbol: Some(&prefixes::MEGA),
                            atom_symbol: &AtomSymbol::AcreAcrUs(derived_units::AcreAcrUs)
                        },
                        exponent: Some(2)
                    },
                    Annotation("things")
                ),
                ""
            ))
        );
        assert_eq!(
            Parser::parse("2km2{things}stuff", &PREFIXES, &UNITS),
            Ok((
                Component::AnnotatedAnnotatable(
                    Annotatable {
                        simple_unit: SimpleUnit {
                            factor: Some(2),
                            prefix_symbol: Some(&prefixes::KILO),
                            atom_symbol: &AtomSymbol::Meter(base_units::Meter)
                        },
                        exponent: Some(2)
                    },
                    Annotation("things")
                ),
                "stuff"
            ))
        );
    }

    #[test]
    fn test_parse_annotatable() {
        assert_eq!(
            Parser::parse("2km2", &PREFIXES, &UNITS),
            Ok((
                Component::Annotatable(Annotatable {
                    simple_unit: SimpleUnit {
                        factor: Some(2),
                        prefix_symbol: Some(&prefixes::KILO),
                        atom_symbol: &AtomSymbol::Meter(base_units::Meter)
                    },
                    exponent: Some(2)
                }),
                ""
            ))
        );
        assert_eq!(
            Parser::parse("2km2asdf", &PREFIXES, &UNITS),
            Ok((
                Component::Annotatable(Annotatable {
                    simple_unit: SimpleUnit {
                        factor: Some(2),
                        prefix_symbol: Some(&prefixes::KILO),
                        atom_symbol: &AtomSymbol::Meter(base_units::Meter)
                    },
                    exponent: Some(2)
                }),
                "asdf"
            ))
        );
    }

    #[test]
    fn test_parse_annotation() {
        assert_eq!(
            Parser::parse("{things}", &PREFIXES, &UNITS),
            Ok((Component::Annotation(Annotation("things")), ""))
        );
        assert_eq!(
            Parser::parse("{things}!@#$", &PREFIXES, &UNITS),
            Ok((Component::Annotation(Annotation("things")), "!@#$"))
        );
    }

    #[test]
    fn test_parse_factor() {
        assert_eq!(
            Parser::parse("2", &PREFIXES, &UNITS),
            Ok((Component::Factor(2), ""))
        );
        assert_eq!(
            Parser::parse("2!@#$", &PREFIXES, &UNITS),
            Ok((Component::Factor(2), "!@#$"))
        );
    }

    #[test]
    fn test_nested_term() {
        assert_eq!(
            Parser::parse("(2km2)", &PREFIXES, &UNITS),
            Ok((
                Component::NestedTerm(Term::Basic(Box::new(Component::Annotatable(Annotatable {
                    simple_unit: SimpleUnit {
                        factor: Some(2),
                        prefix_symbol: Some(&prefixes::KILO),
                        atom_symbol: &AtomSymbol::Meter(base_units::Meter)
                    },
                    exponent: Some(2)
                })))),
                ""
            ))
        );
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(Parser::parse("", &PREFIXES, &UNITS), Err(""));
    }
}
