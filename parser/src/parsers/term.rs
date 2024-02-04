use super::component;
use crate::{
    atom_collection::AtomCollection,
    parse::{Parse, ParseResult},
    prefix_collection::PrefixCollection,
    tokens::{Component, Separator, Term},
};

pub(super) struct Parser;

impl<'input> Parse<'input, Term<'input>> for Parser {
    fn parse(
        input: &'input str,
        prefixes: &'static PrefixCollection,
        atoms: &'static AtomCollection,
    ) -> ParseResult<'input, Term<'input>> {
        // TODO: see if this has any overhead
        if input.is_empty() {
            return Err(input);
        }

        // Try parsing the nested stuff to consume the Atom first (i.e. skip the factor and
        // prefix). If we don't, we end up successfully parsing a prefix or factor first when we
        // should be matching a longer-char'ed atom.
        let (component, component_tail) = component::parse_atom_first(input, prefixes, atoms)
            .or_else(|_| component::Parser::parse(input, prefixes, atoms).map_err(|_| input))?;

        if let Some(stripped_from_dot) = component_tail.strip_prefix('.') {
            return parse_dot_term(stripped_from_dot, component, prefixes, atoms);
        } else if let Some(stripped_from_slash) = component_tail.strip_prefix('/') {
            return parse_slash_term(stripped_from_slash, component, prefixes, atoms);
        } else if !component_tail.is_empty() && !component_tail.starts_with(')') {
            // If we're here, we didn't fully parse a component using the atom-first approach, so
            // let's parse the "normal" way.
            let (component, inner_component_tail) =
                component::Parser::parse(input, prefixes, atoms).map_err(|_| input)?;

            if let Some(stripped) = inner_component_tail.strip_prefix('.') {
                return parse_dot_term(stripped, component, prefixes, atoms);
            } else if let Some(stripped) = inner_component_tail.strip_prefix('/') {
                return parse_slash_term(stripped, component, prefixes, atoms);
            }
            return Ok((Term::Basic(Box::new(component)), inner_component_tail));
        }

        Ok((Term::Basic(Box::new(component)), component_tail))
    }
}

fn parse_dot_term<'input>(
    input: &'input str,
    component: Component<'input>,
    prefixes: &'static PrefixCollection,
    atoms: &'static AtomCollection,
) -> ParseResult<'input, Term<'input>> {
    Parser::parse(input, prefixes, atoms).map(|(term, tail)| {
        let t = Term::Combined(Box::new(component), Separator::Dot, Box::new(term));
        (t, tail)
    })
}

fn parse_slash_term<'input>(
    input: &'input str,
    component: Component<'input>,
    prefixes: &'static PrefixCollection,
    atoms: &'static AtomCollection,
) -> ParseResult<'input, Term<'input>> {
    Parser::parse(input, prefixes, atoms).map(|(term, tail)| {
        let t = Term::Combined(Box::new(component), Separator::Slash, Box::new(term));
        (t, tail)
    })
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{
        base_units, derived_units,
        prefixes::{self, PREFIXES},
        tokens::{Annotatable, AtomSymbol, Component, Separator, SimpleUnit, Term},
        units::UNITS,
        Parse,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_component_only() {
        let component = Component::Annotatable(Annotatable {
            simple_unit: SimpleUnit {
                factor: Some(2),
                prefix_symbol: Some(&prefixes::KILO),
                atom_symbol: &AtomSymbol::Meter(base_units::Meter),
            },
            exponent: Some(2),
        });

        let expected = Term::Basic(Box::new(component));
        assert_eq!(Parser::parse("2km2", &PREFIXES, &UNITS), Ok((expected, "")));
    }

    #[test]
    fn test_parse_dot() {
        let term2 = Term::Basic(Box::new(Component::Annotatable(Annotatable {
            simple_unit: SimpleUnit {
                factor: Some(3),
                prefix_symbol: Some(&prefixes::MILLI),
                atom_symbol: &AtomSymbol::Gram(base_units::Gram),
            },
            exponent: Some(3),
        })));

        let expected = Term::Combined(
            Box::new(Component::Annotatable(Annotatable {
                simple_unit: SimpleUnit {
                    factor: Some(2),
                    prefix_symbol: Some(&prefixes::KILO),
                    atom_symbol: &AtomSymbol::Meter(base_units::Meter),
                },
                exponent: Some(2),
            })),
            Separator::Dot,
            Box::new(term2),
        );

        assert_eq!(
            Parser::parse("2km2.3mg3", &PREFIXES, &UNITS),
            Ok((expected.clone(), ""))
        );
        assert_eq!(
            Parser::parse("2km2.3mg3!@#$", &PREFIXES, &UNITS),
            Ok((expected, "!@#$"))
        );
    }

    #[test]
    fn test_parse_dot_nested_dot() {
        let term3 = Term::Basic(Box::new(Component::Annotatable(Annotatable {
            simple_unit: SimpleUnit {
                factor: Some(4),
                prefix_symbol: Some(&prefixes::DECI),
                atom_symbol: &AtomSymbol::AcreAcrUs(derived_units::AcreAcrUs),
            },
            exponent: Some(4),
        })));

        let term2 = Term::Combined(
            Box::new(Component::Annotatable(Annotatable {
                simple_unit: SimpleUnit {
                    factor: Some(3),
                    prefix_symbol: Some(&prefixes::MILLI),
                    atom_symbol: &AtomSymbol::Gram(base_units::Gram),
                },
                exponent: Some(3),
            })),
            Separator::Dot,
            Box::new(term3),
        );

        let expected = Term::Combined(
            Box::new(Component::Annotatable(Annotatable {
                simple_unit: SimpleUnit {
                    factor: Some(2),
                    prefix_symbol: Some(&prefixes::KILO),
                    atom_symbol: &AtomSymbol::Meter(base_units::Meter),
                },
                exponent: Some(2),
            })),
            Separator::Dot,
            Box::new(term2),
        );

        assert_eq!(
            Parser::parse("2km2.3mg3.4d[acr_us]4", &PREFIXES, &UNITS),
            Ok((expected.clone(), ""))
        );
        assert_eq!(
            Parser::parse("2km2.3mg3.4d[acr_us]4!@#", &PREFIXES, &UNITS),
            Ok((expected, "!@#"))
        );
    }

    #[test]
    fn test_parse_dot_nested_slash() {
        let term3 = Term::Basic(Box::new(Component::Annotatable(Annotatable {
            simple_unit: SimpleUnit {
                factor: Some(4),
                prefix_symbol: Some(&prefixes::DECI),
                atom_symbol: &AtomSymbol::AcreAcrUs(derived_units::AcreAcrUs),
            },
            exponent: Some(4),
        })));

        let term2 = Term::Combined(
            Box::new(Component::Annotatable(Annotatable {
                simple_unit: SimpleUnit {
                    factor: Some(3),
                    prefix_symbol: Some(&prefixes::MILLI),
                    atom_symbol: &AtomSymbol::Gram(base_units::Gram),
                },
                exponent: Some(3),
            })),
            Separator::Slash,
            Box::new(term3),
        );

        let expected = Term::Combined(
            Box::new(Component::Annotatable(Annotatable {
                simple_unit: SimpleUnit {
                    factor: Some(2),
                    prefix_symbol: Some(&prefixes::KILO),
                    atom_symbol: &AtomSymbol::Meter(base_units::Meter),
                },
                exponent: Some(2),
            })),
            Separator::Dot,
            Box::new(term2),
        );

        assert_eq!(
            Parser::parse("2km2.3mg3/4d[acr_us]4", &PREFIXES, &UNITS),
            Ok((expected.clone(), ""))
        );
        assert_eq!(
            Parser::parse("2km2.3mg3/4d[acr_us]4!@#", &PREFIXES, &UNITS),
            Ok((expected, "!@#"))
        );
    }

    #[test]
    fn test_parse_slash() {
        let term2 = Term::Basic(Box::new(Component::Annotatable(Annotatable {
            simple_unit: SimpleUnit {
                factor: Some(3),
                prefix_symbol: Some(&prefixes::MILLI),
                atom_symbol: &AtomSymbol::Gram(base_units::Gram),
            },
            exponent: Some(3),
        })));

        let expected = Term::Combined(
            Box::new(Component::Annotatable(Annotatable {
                simple_unit: SimpleUnit {
                    factor: Some(2),
                    prefix_symbol: Some(&prefixes::KILO),
                    atom_symbol: &AtomSymbol::Meter(base_units::Meter),
                },
                exponent: Some(2),
            })),
            Separator::Slash,
            Box::new(term2),
        );

        assert_eq!(
            Parser::parse("2km2/3mg3", &PREFIXES, &UNITS),
            Ok((expected.clone(), ""))
        );
        assert_eq!(
            Parser::parse("2km2/3mg3!@#$", &PREFIXES, &UNITS),
            Ok((expected, "!@#$"))
        );
    }

    #[test]
    fn test_parse_slash_nested_slash() {
        let term3 = Term::Basic(Box::new(Component::Annotatable(Annotatable {
            simple_unit: SimpleUnit {
                factor: Some(4),
                prefix_symbol: Some(&prefixes::DECI),
                atom_symbol: &AtomSymbol::AcreAcrUs(derived_units::AcreAcrUs),
            },
            exponent: Some(4),
        })));

        let term2 = Term::Combined(
            Box::new(Component::Annotatable(Annotatable {
                simple_unit: SimpleUnit {
                    factor: Some(3),
                    prefix_symbol: Some(&prefixes::MILLI),
                    atom_symbol: &AtomSymbol::Gram(base_units::Gram),
                },
                exponent: Some(3),
            })),
            Separator::Slash,
            Box::new(term3),
        );

        let expected = Term::Combined(
            Box::new(Component::Annotatable(Annotatable {
                simple_unit: SimpleUnit {
                    factor: Some(2),
                    prefix_symbol: Some(&prefixes::KILO),
                    atom_symbol: &AtomSymbol::Meter(base_units::Meter),
                },
                exponent: Some(2),
            })),
            Separator::Slash,
            Box::new(term2),
        );

        assert_eq!(
            Parser::parse("2km2/3mg3/4d[acr_us]4", &PREFIXES, &UNITS),
            Ok((expected.clone(), ""))
        );
        assert_eq!(
            Parser::parse("2km2/3mg3/4d[acr_us]4!@#", &PREFIXES, &UNITS),
            Ok((expected, "!@#"))
        );
    }

    #[test]
    fn test_parse_slash_nested_dot() {
        let term3 = Term::Basic(Box::new(Component::Annotatable(Annotatable {
            simple_unit: SimpleUnit {
                factor: Some(4),
                prefix_symbol: Some(&prefixes::DECI),
                atom_symbol: &AtomSymbol::AcreAcrUs(derived_units::AcreAcrUs),
            },
            exponent: Some(4),
        })));

        let term2 = Term::Combined(
            Box::new(Component::Annotatable(Annotatable {
                simple_unit: SimpleUnit {
                    factor: Some(3),
                    prefix_symbol: Some(&prefixes::MILLI),
                    atom_symbol: &AtomSymbol::Gram(base_units::Gram),
                },
                exponent: Some(3),
            })),
            Separator::Dot,
            Box::new(term3),
        );

        let expected = Term::Combined(
            Box::new(Component::Annotatable(Annotatable {
                simple_unit: SimpleUnit {
                    factor: Some(2),
                    prefix_symbol: Some(&prefixes::KILO),
                    atom_symbol: &AtomSymbol::Meter(base_units::Meter),
                },
                exponent: Some(2),
            })),
            Separator::Slash,
            Box::new(term2),
        );

        assert_eq!(
            Parser::parse("2km2/3mg3.4d[acr_us]4", &PREFIXES, &UNITS),
            Ok((expected.clone(), ""))
        );
        assert_eq!(
            Parser::parse("2km2/3mg3.4d[acr_us]4!@#", &PREFIXES, &UNITS),
            Ok((expected, "!@#"))
        );
    }
}
