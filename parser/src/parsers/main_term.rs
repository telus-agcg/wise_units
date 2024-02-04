use super::term;
use crate::{
    atom_collection::AtomCollection,
    parse::{Parse, ParseResult},
    prefix_collection::PrefixCollection,
    tokens::MainTerm,
};

pub struct Parser;

impl<'input> Parse<'input, MainTerm<'input>> for Parser {
    fn parse(
        input: &'input str,
        prefixes: &'static PrefixCollection,
        atoms: &'static AtomCollection,
    ) -> ParseResult<'input, MainTerm<'input>> {
        let mut leading_slash = false;

        let tail = input.strip_prefix('/').map_or(input, |stripped| {
            leading_slash = true;
            stripped
        });

        term::Parser::parse(tail, prefixes, atoms)
            .map(|(term, new_tail)| {
                let mt = MainTerm {
                    leading_slash,
                    term,
                };

                (mt, new_tail)
            })
            .map_err(|_| input)
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{
        base_units,
        prefixes::{self, PREFIXES},
        tokens::{Annotatable, AtomSymbol, Component, MainTerm, SimpleUnit, Term},
        units::UNITS,
        Parse,
    };

    #[test]
    fn test_parse_no_slash() {
        let expected = MainTerm {
            term: Term::Basic(Box::new(Component::Factor(2))),
            leading_slash: false,
        };
        assert_eq!(
            Parser::parse("2", &PREFIXES, &UNITS),
            Ok((expected.clone(), ""))
        );
        assert_eq!(
            Parser::parse("2!@#$", &PREFIXES, &UNITS),
            Ok((expected, "!@#$"))
        );

        let component = Component::Annotatable(Annotatable {
            simple_unit: SimpleUnit {
                factor: Some(2),
                prefix_symbol: Some(&prefixes::KILO),
                atom_symbol: &AtomSymbol::Meter(base_units::Meter),
            },
            exponent: Some(2),
        });

        let term = Term::Basic(Box::new(component));

        let expected = MainTerm {
            term,
            leading_slash: false,
        };

        assert_eq!(
            Parser::parse("2km2", &PREFIXES, &UNITS),
            Ok((expected.clone(), ""))
        );
        assert_eq!(
            Parser::parse("2km2!@#$", &PREFIXES, &UNITS),
            Ok((expected, "!@#$"))
        );
    }

    #[test]
    fn test_parse_with_slash() {
        let expected = MainTerm {
            term: Term::Basic(Box::new(Component::Factor(0))),
            leading_slash: true,
        };
        assert_eq!(
            Parser::parse("/0", &PREFIXES, &UNITS),
            Ok((expected.clone(), ""))
        );
        assert_eq!(
            Parser::parse("/0!@#$", &PREFIXES, &UNITS),
            Ok((expected, "!@#$"))
        );
    }
}
