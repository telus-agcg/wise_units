use nom::{
    character::complete::char,
    combinator::{all_consuming, opt},
    sequence::pair,
    IResult,
};

use super::term::Term;

#[derive(Debug, PartialEq)]
pub(in crate::unit) enum MainTerm<'i> {
    SlashTerm(Term<'i>),
    Term(Term<'i>),
}

impl TryFrom<MainTerm<'_>> for crate::Unit {
    type Error = ();

    fn try_from(main_term: MainTerm<'_>) -> Result<Self, Self::Error> {
        match main_term {
            MainTerm::SlashTerm(term) => {
                // let unit = Self::try_from(term)?;
                let unit = Self::try_from(term).unwrap();
                Ok(num_traits::Inv::inv(unit))
            }
            MainTerm::Term(term) => Self::try_from(term),
        }
    }
}

pub(super) fn parse(input: &[u8]) -> IResult<&[u8], MainTerm<'_>> {
    let (tail, (maybe_slash, term)) =
        all_consuming(pair(opt(char('/')), super::term::parse))(input)?;

    if maybe_slash.is_some() {
        Ok((tail, MainTerm::SlashTerm(term)))
    } else {
        Ok((tail, MainTerm::Term(term)))
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{
        unit::{
            self,
            parser2::{
                annotatable::Annotatable,
                component::Component,
                simple_unit::SimpleUnit,
                term::{Op, Term},
            },
        },
        Atom,
    };

    use super::*;

    #[test]
    fn single_atom_symbol_test() {
        let sorted = unit::testing::all_atom_symbols_sorted();

        for atom_symbol in &sorted {
            validate_parse!(
                unit_str: atom_symbol,
                parser: parse,
                expected: MainTerm::Term(Term {
                    lhs: Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom(
                                             unit::parser::simple_unit::atom_symbol_to_atom(atom_symbol)
                                     )),annotation: None
                    },
                    rhs: None,
                })
            );
        }
    }

    #[test]
    #[allow(clippy::string_lit_as_bytes)]
    fn two_meter_test() {
        validate_parse!(
            unit_str: "2m",
            parser: parse,
            expected: MainTerm::Term(Term {
                lhs: Component::Annotatable{
                    factor: Some(2),
                    annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom(
                             Atom::Meter
                     )),annotation: None
                },
                rhs: None
            })
        );
    }

    #[test]
    fn prefixed_metric_atom_symbol_test() {
        let sorted = unit::testing::metric_atom_symbols_sorted();

        for atom_symbol in &sorted {
            for prefix_symbol in &unit::testing::PREFIX_SYMBOLS {
                let unit = format!("{prefix_symbol}{atom_symbol}");
                let prefix = unit::parser::simple_unit::prefix_symbol_to_prefix(prefix_symbol);
                let atom = unit::parser::simple_unit::atom_symbol_to_atom(atom_symbol);

                validate_parse!(
                    unit_str: unit,
                    parser: parse,
                    expected: MainTerm::Term(Term {
                        lhs: Component::Annotatable {
                            factor: None,
                            annotatable: Annotatable::SimpleUnit(
                                SimpleUnit::PrefixAtom { prefix, atom },
                                ),
                                annotation: None
                        },
                        rhs: None,
                    })
                );
            }
        }
    }

    #[test]
    #[allow(clippy::string_lit_as_bytes)]
    fn meter_dot_gram_test() {
        validate_parse!(
            unit_str: "m.g",
            parser: parse,
            expected: MainTerm::Term(Term {
                lhs: Component::Annotatable{
                    factor: None,
                    annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom(
                             Atom::Meter
                     )),annotation: None
                },
                rhs: Some((
                    Op::Dot,
                    Box::new(Term {
                        lhs: Component::Annotatable{
                            factor: None,
                            annotatable: Annotatable::SimpleUnit(
                                 SimpleUnit::Atom(Atom::Gram)
                             ),
                             annotation: None
                        },
                        rhs: None
                    })
                ))
            })
        );
    }

    // NOTE: This test takes some time (~25s right now) due to all of the permutations it tests.
    // Could be worth moving to an acceptance test...
    #[test]
    fn term_dot_term_test() {
        let sorted = unit::testing::all_atom_symbols_sorted();

        for (lhs, rhs) in sorted.iter().tuple_combinations() {
            let unit = format!("{lhs}.{rhs}");

            validate_parse!(
                unit_str: unit,
                parser: parse,
                expected: MainTerm::Term(Term{
                    lhs: Component::Annotatable{ factor: None, annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom(
                                 unit::parser::simple_unit::atom_symbol_to_atom(lhs),
                                 )),annotation: None},
                    rhs: Some((
                            Op::Dot,
                            Box::new(Term {
                                lhs: Component::Annotatable{ factor: None, annotatable: Annotatable::SimpleUnit(
                                         SimpleUnit::Atom(unit::parser::simple_unit::atom_symbol_to_atom(rhs))
                                     ), annotation: None
                                },
                                rhs: None,
                            })
                    ))
                })
            );
        }
    }

    #[test]
    fn meter_dot_gram_dot_kelvin_test() {
        let (tail, output) = parse(b"m.g.K").unwrap();
        assert!(tail.is_empty());

        pretty_assertions::assert_eq!(
            output,
            MainTerm::Term(Term {
                lhs: Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom(Atom::Meter)),
                    annotation: None
                },
                rhs: Some((
                    Op::Dot,
                    Box::new(Term {
                        lhs: Component::Annotatable {
                            factor: None,
                            annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom(Atom::Gram)),
                            annotation: None
                        },
                        rhs: Some((
                            Op::Dot,
                            Box::new(Term {
                                lhs: Component::Annotatable {
                                    factor: None,
                                    annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom(
                                        Atom::Kelvin
                                    )),
                                    annotation: None
                                },
                                rhs: None
                            })
                        ))
                    })
                ))
            })
        );
    }

    // #[test]
    // fn term_dot_term_dot_term_test() {
    //     let sorted = unit::testing::all_atom_symbols_sorted();
    //
    //     for (lhs, mid, rhs) in sorted.iter().tuple_combinations() {
    //         let unit = format!("{lhs}.{mid}.{rhs}");
    //         eprintln!("Validating {lhs}+{mid}+{rhs}");
    //
    //         let (tail, main_term) = parse_main_term(unit.as_bytes()).unwrap();
    //         assert!(
    //             tail.is_empty(),
    //             "tail was: {}",
    //             std::str::from_utf8(tail).unwrap()
    //         );
    //
    //         pretty_assertions::assert_eq!(
    //             main_term,
    //             MainTerm::Term(Term::Multi(MultiTerm {
    //                 lhs: Component::Annotatable(Annotatable::SimpleUnit(SimpleUnit::Atom(
    //                     unit::parser::simple_unit::atom_symbol_to_atom(lhs),
    //                 ))),
    //                 rhs: vec![(
    //                     Op::Dot,
    //                     Term::Single(Component::Annotatable(Annotatable::SimpleUnit(
    //                         SimpleUnit::Atom(unit::parser::simple_unit::atom_symbol_to_atom(
    //                             rhs
    //                         ))
    //                     )))
    //                 )]
    //             }))
    //         );
    //     }
    // }
}
