mod annotatable;
mod component;
mod error;
mod main_term;
mod simple_unit;
mod term;

use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser as _};

pub use self::error::Error;

use self::{
    annotatable::Annotatable, component::Component, main_term::MainTerm, simple_unit::SimpleUnit,
    term::Term,
};

#[derive(pest_derive::Parser)]
#[grammar = "unit/parser/unit.pest"]
pub(crate) struct UnitParser;

lazy_static::lazy_static! {
    static ref UNIT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::Left, Op};

        PrattParser::new()
            .op(Op::prefix(Rule::leading_slash) | Op::prefix(Rule::sign) | Op::prefix(Rule::factor) | Op::prefix(Rule::prefix_symbol))
            .op(Op::infix(Rule::dot, Left) | Op::infix(Rule::slash, Left))
            .op(Op::postfix(Rule::exponent)| Op::postfix(Rule::annotation_string) | Op::postfix(Rule::EOI))
    };
}

trait Parse<'i> {
    fn parse(pairs: Pairs<'i, Rule>, pratt: &PrattParser<Rule>) -> Self;
}

trait TryParse<'i>: Sized {
    fn try_parse(pairs: Pairs<'i, Rule>, pratt: &PrattParser<Rule>) -> Result<Self, Error>;
}

pub(super) fn parse(expr: &str) -> Result<MainTerm<'_>, Error> {
    let pairs = UnitParser::parse(Rule::main_term, expr)?;

    UNIT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::main_term => MainTerm::try_parse(primary.into_inner(), &UNIT_PARSER),
            rule => unreachable!("expected main_term, found {rule:?}"),
        })
        .parse(pairs)
}

#[cfg(test)]
mod pratt_tests {
    use super::*;

    mod single_term {
        use crate::unit::parser::{self, annotatable::Exponent};

        use super::*;

        #[test]
        fn atom_test() {
            // metric atom
            pretty_assertions::assert_eq!(
                parse("m").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("m")),
                    annotation: None
                }))
            );

            // non-metric atom
            pretty_assertions::assert_eq!(
                parse("[pi]").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("[pi]")),
                    annotation: None
                }))
            );
        }

        #[test]
        fn factor_atom_test() {
            // metric atom
            pretty_assertions::assert_eq!(
                parse("3m").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: Some("3"),
                    annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("m")),
                    annotation: None
                }))
            );

            // non-metric atom
            pretty_assertions::assert_eq!(
                parse("3[pi]").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: Some("3"),
                    annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("[pi]")),
                    annotation: None
                }))
            );
        }

        #[test]
        fn prefix_atom_test() {
            // metric atom
            pretty_assertions::assert_eq!(
                parse("har").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnit(SimpleUnit::PrefixAtom {
                        prefix_symbol: "h",
                        atom_symbol: "ar"
                    }),
                    annotation: None
                }))
            );
        }

        #[test]
        fn atom_exponent_test() {
            // metric atom
            pretty_assertions::assert_eq!(
                parse("m2").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: SimpleUnit::Atom("m"),
                        exponent: Exponent::Digits("2")
                    },
                    annotation: None
                }))
            );

            // non-metric atom
            pretty_assertions::assert_eq!(
                parse("[pi]2").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: SimpleUnit::Atom("[pi]"),
                        exponent: Exponent::Digits("2")
                    },
                    annotation: None
                }))
            );
        }

        #[test]
        fn atom_negative_exponent_test() {
            // metric atom
            pretty_assertions::assert_eq!(
                parse("m-2").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: SimpleUnit::Atom("m"),
                        exponent: Exponent::SignDigits {
                            sign: "-",
                            digits: "2"
                        }
                    },
                    annotation: None
                }))
            );

            // non-metric atom
            pretty_assertions::assert_eq!(
                parse("[pi]-2").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: SimpleUnit::Atom("[pi]"),
                        exponent: Exponent::SignDigits {
                            sign: "-",
                            digits: "2"
                        }
                    },
                    annotation: None
                }))
            );
        }

        #[test]
        fn atom_positive_exponent_test() {
            // metric atom
            pretty_assertions::assert_eq!(
                parse("m+2").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: SimpleUnit::Atom("m"),
                        exponent: Exponent::SignDigits {
                            sign: "+",
                            digits: "2"
                        }
                    },
                    annotation: None
                }))
            );

            // non-metric atom
            pretty_assertions::assert_eq!(
                parse("[pi]+2").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: SimpleUnit::Atom("[pi]"),
                        exponent: Exponent::SignDigits {
                            sign: "+",
                            digits: "2"
                        }
                    },
                    annotation: None
                }))
            );
        }

        #[test]
        fn factor_atom_exponent_test() {
            // metric atom
            pretty_assertions::assert_eq!(
                parse("3m2").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: Some("3"),
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: SimpleUnit::Atom("m"),
                        exponent: Exponent::Digits("2")
                    },
                    annotation: None
                }))
            );

            // non-metric atom
            pretty_assertions::assert_eq!(
                parse("3[pi]2").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: Some("3"),
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: SimpleUnit::Atom("[pi]"),
                        exponent: Exponent::Digits("2")
                    },
                    annotation: None
                }))
            );
        }

        #[test]
        fn atom_exponent_annotation_test() {
            // metric atom
            pretty_assertions::assert_eq!(
                parse("m2{please_work}").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: SimpleUnit::Atom("m"),
                        exponent: Exponent::Digits("2")
                    },
                    annotation: Some("please_work")
                }))
            );

            // non-metric atom
            pretty_assertions::assert_eq!(
                parse("[pi]2{please_work}").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: SimpleUnit::Atom("[pi]"),
                        exponent: Exponent::Digits("2")
                    },
                    annotation: Some("please_work")
                }))
            );
        }

        #[test]
        fn factor_atom_exponent_annotation_test() {
            // metric atom
            pretty_assertions::assert_eq!(
                parse("3m2{please_work}").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: Some("3"),
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: SimpleUnit::Atom("m"),
                        exponent: Exponent::Digits("2")
                    },
                    annotation: Some("please_work")
                }))
            );

            // non-metric atom
            pretty_assertions::assert_eq!(
                parse("3[pi]2{please_work}").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: Some("3"),
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: SimpleUnit::Atom("[pi]"),
                        exponent: Exponent::Digits("2")
                    },
                    annotation: Some("please_work")
                }))
            );
        }

        #[test]
        fn factor_prefix_atom_exponent_annotation_test() {
            // prefix + metric atom
            pretty_assertions::assert_eq!(
                parse("3km2{please_work}").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: Some("3"),
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: SimpleUnit::PrefixAtom {
                            prefix_symbol: "k",
                            atom_symbol: "m"
                        },
                        exponent: Exponent::Digits("2")
                    },
                    annotation: Some("please_work")
                }))
            );

            // prefix + non-metric atom
            pretty_assertions::assert_eq!(
                parse("3k[pi]2{please_work}").unwrap_err(),
                parser::Error::UnableToParseUnit(pest::error::Error::new_from_pos(
                    pest::error::ErrorVariant::ParsingError {
                        positives: vec![Rule::metric_atom_symbol],
                        negatives: vec![]
                    },
                    pest::Position::new("3k[pi]2{please_work}", 2).unwrap()
                ))
            );
        }

        #[test]
        fn annotation_test() {
            pretty_assertions::assert_eq!(
                parse("{please_work}").unwrap(),
                MainTerm::Term(Term::Single(Component::Annotation("please_work")))
            );
        }

        #[test]
        fn factor_test() {
            pretty_assertions::assert_eq!(
                parse("42").unwrap(),
                MainTerm::Term(Term::Single(Component::Factor {
                    factor: "42",
                    annotation: None
                }))
            );
        }

        #[test]
        fn factor_annotation_test() {
            pretty_assertions::assert_eq!(
                parse("42{things}").unwrap(),
                MainTerm::Term(Term::Single(Component::Factor {
                    factor: "42",
                    annotation: Some("things")
                }))
            );
        }
    }

    mod multi_dot_term {
        use crate::unit::parser::term::{MultiTerm, Op};

        use super::*;

        #[test]
        fn atom_2_test() {
            pretty_assertions::assert_eq!(
                parse("m.g").unwrap(),
                MainTerm::Term(Term::Multi(MultiTerm {
                    lhs: Box::new(Term::Single(Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("m")),
                        annotation: None
                    })),
                    op: Op::Dot,
                    rhs: Box::new(Term::Single(Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("g")),
                        annotation: None
                    }))
                }))
            );
        }

        #[test]
        fn atom_3_test() {
            pretty_assertions::assert_eq!(
                parse("m.g.K").unwrap(),
                MainTerm::Term(Term::Multi(MultiTerm {
                    lhs: Box::new(Term::Multi(MultiTerm {
                        lhs: Box::new(Term::Single(Component::Annotatable {
                            factor: None,
                            annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("m")),
                            annotation: None
                        })),
                        op: Op::Dot,
                        rhs: Box::new(Term::Single(Component::Annotatable {
                            factor: None,
                            annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("g")),
                            annotation: None
                        }))
                    })),
                    op: Op::Dot,
                    rhs: Box::new(Term::Single(Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("K")),
                        annotation: None
                    }))
                }))
            );
        }
    }

    mod multi_slash_term {
        use crate::unit::parser::term::{MultiTerm, Op};

        use super::*;

        #[test]
        fn atom_2_test() {
            pretty_assertions::assert_eq!(
                parse("m/g").unwrap(),
                MainTerm::Term(Term::Multi(MultiTerm {
                    lhs: Box::new(Term::Single(Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("m")),
                        annotation: None
                    })),
                    op: Op::Slash,
                    rhs: Box::new(Term::Single(Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("g")),
                        annotation: None
                    }))
                }))
            );
        }

        #[test]
        fn atom_3_test() {
            pretty_assertions::assert_eq!(
                parse("m/g/K").unwrap(),
                MainTerm::Term(Term::Multi(MultiTerm {
                    lhs: Box::new(Term::Multi(MultiTerm {
                        lhs: Box::new(Term::Single(Component::Annotatable {
                            factor: None,
                            annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("m")),
                            annotation: None
                        })),
                        op: Op::Slash,
                        rhs: Box::new(Term::Single(Component::Annotatable {
                            factor: None,
                            annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("g")),
                            annotation: None
                        }))
                    })),
                    op: Op::Slash,
                    rhs: Box::new(Term::Single(Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("K")),
                        annotation: None
                    }))
                }))
            );
        }
    }

    mod slash_main_term {
        use crate::unit::parser::annotatable::Exponent;

        use super::*;

        #[test]
        fn factor_atom_exponent_annotation_test() {
            pretty_assertions::assert_eq!(
                parse("/3m2{please_work}").unwrap(),
                MainTerm::SlashTerm(Term::Single(Component::Annotatable {
                    factor: Some("3"),
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: SimpleUnit::Atom("m"),
                        exponent: Exponent::Digits("2")
                    },
                    annotation: Some("please_work")
                }))
            );
        }
    }
}

// #[cfg(test)]
// mod base_parser_tests {
//     use pest::{consumes_to, fails_with, parses_to};
//
//     use super::*;
//
//     #[test]
//     fn validate_prefixes() {
//         parses_to! {
//             parser: UnitParser,
//             input: "k",
//             rule: Rule::prefix_symbol,
//             tokens: [prefix_symbol(0, 1)]
//         }
//
//         fails_with! {
//             parser: UnitParser,
//             input: "i",
//             rule: Rule::prefix_symbol,
//             positives: vec![Rule::prefix_symbol],
//             negatives: vec![],
//             pos: 0
//         }
//
//         parses_to! {
//             parser: UnitParser,
//             input: "a",
//             rule: Rule::prefix_symbol,
//             tokens: [prefix_symbol(0, 1)]
//         }
//     }
//
//     // TODO: Should test metric vs non-metric atoms.
//     #[test]
//     fn validate_atoms() {
//         for atom_symbol in crate::unit::testing::all_atom_symbols() {
//             let pairs = UnitParser::parse(Rule::any_atom_symbol, "m");
//             assert!(pairs.is_ok());
//
//             parses_to! {
//                 parser: UnitParser,
//                 input: atom_symbol,
//                 rule: Rule::any_atom_symbol,
//                 tokens: [metric_atom_symbol(0, 1)]
//             }
//         }
//     }
// }
//
// #[cfg(test)]
// // Allowing manual_assert because of pest's `parses_to` macro.
// // #[allow(clippy::cognitive_complexity, clippy::manual_assert)]
// mod old_term_parser_tests {
//     use pest::{consumes_to, parses_to, Parser};
//
//     use super::*;
//
//     #[test]
//     fn parse_sign() {
//         parses_to! {
//             parser: UnitParser,
//             input: "+",
//             rule: Rule::sign,
//             tokens: [
//                 sign(0, 1)
//             ]
//         }
//
//         parses_to! {
//             parser: UnitParser,
//             input: "-",
//             rule: Rule::sign,
//             tokens: [
//                 sign(0, 1)
//             ]
//         }
//     }
//
//     #[test]
//     fn validate_digits() {
//         let pairs = UnitParser::parse(Rule::digits, "0");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::digits, "01");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::digits, "123450");
//         assert!(pairs.is_ok());
//
//         // Looks like it stops parsing at the !, but doesn't error.
//         parses_to! {
//             parser: UnitParser,
//             input: "123456!@#",
//             rule: Rule::digits,
//             tokens: [digits(0, 6)]
//         }
//
//         let pairs = UnitParser::parse(Rule::digits, "!@#123450");
//         assert!(pairs.is_err());
//     }
//
//     #[test]
//     fn validate_factor() {
//         let pairs = UnitParser::parse(Rule::factor, "0");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::factor, "01");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::factor, "123450");
//         assert!(pairs.is_ok());
//
//         parses_to! {
//             parser: UnitParser,
//             input: "123456!@#",
//             rule: Rule::factor,
//             tokens: [factor(0, 6)]
//         }
//     }
//
//     #[test]
//     fn validate_exponent() {
//         let pairs = UnitParser::parse(Rule::exponent, "+0");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::exponent, "-0");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::exponent, "123");
//         assert!(pairs.is_ok());
//
//         parses_to! {
//             parser: UnitParser,
//             input: "-123",
//             rule: Rule::exponent,
//             tokens: [exponent(0, 4, [sign(0, 1), digits(1, 4)])]
//         }
//     }
//
//     #[test]
//     fn validate_simple_unit() {
//         let pairs = UnitParser::parse(Rule::simple_unit, "km");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::simple_unit, "m");
//         assert!(pairs.is_ok());
//
//         parses_to! {
//             parser: UnitParser,
//             input: "km",
//             rule: Rule::simple_unit,
//             tokens: [
//                 simple_unit(0, 2)
//             ]
//         }
//     }
//
//     #[test]
//     fn validate_simple_unit_with_exponent() {
//         let pairs = UnitParser::parse(Rule::simple_unit, "km2");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::simple_unit, "m-1");
//         assert!(pairs.is_ok());
//
//         parses_to! {
//             parser: UnitParser,
//             input: "km2",
//             rule: Rule::annotatable,
//             tokens: [
//                 annotatable(0, 3, [
//                       simple_unit(0, 2),
//                       exponent(2, 3, [digits(2, 3)])
//                 ])
//             ]
//         }
//     }
//
//     #[test]
//     fn validate_annotatable() {
//         let pairs = UnitParser::parse(Rule::annotatable, "km2");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::annotatable, "km-2");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::annotatable, "km+2");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::annotatable, "km");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::annotatable, "m");
//         assert!(pairs.is_ok());
//     }
//
//     #[test]
//     fn validate_annotation_group() {
//         let pairs = UnitParser::parse(Rule::annotation_group, "{d'io}");
//         assert!(pairs.is_ok());
//
//         parses_to! {
//             parser: UnitParser,
//             input: "{tot'nit}",
//             rule: Rule::annotation_group,
//             tokens: [
//                 annotation(1, 8)
//             ]
//         };
//
//         parses_to! {
//             parser: UnitParser,
//             input: "{!\"#$%&'()*+,-./01234567890:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz|~}",
//             rule: Rule::annotation_group,
//             tokens: [
//                 annotation(1, 94)
//             ]
//         };
//
//         let pairs = UnitParser::parse(Rule::annotation_group, "k");
//         assert!(pairs.is_err());
//     }
//
//     #[test]
//     fn validate_basic_component() {
//         let pairs = UnitParser::parse(Rule::basic_component, "km{stuff}");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::basic_component, "km");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::basic_component, "{stuff}");
//         assert!(pairs.is_ok());
//
//         parses_to! {
//             parser: UnitParser,
//             input: "{tot'nit}",
//             rule: Rule::basic_component,
//             tokens: [
//                 basic_component(0, 9, [
//                     annotation(1, 8)
//                 ])
//             ]
//         };
//
//         let pairs = UnitParser::parse(Rule::basic_component, "234");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::basic_component, "(m.s)");
//         assert!(pairs.is_ok());
//     }
//
//     #[test]
//     fn validate_component_with_factor() {
//         let pairs = UnitParser::parse(Rule::component, "100km");
//         assert!(pairs.is_ok());
//
//         parses_to! {
//             parser: UnitParser,
//             input: "2km",
//             rule: Rule::component,
//             tokens: [
//                 component(0, 3, [
//                       factor(0, 1),
//                       basic_component(1, 3, [
//                               annotatable(1, 3, [
//                                       simple_unit(1, 3)
//                               ])
//                       ])
//                 ])
//             ]
//         };
//
//         parses_to! {
//             parser: UnitParser,
//             input: "2km-2{meow}",
//             rule: Rule::component,
//             tokens: [
//                 component(0, 11, [
//                     factor(0, 1),
//                     basic_component(1, 11, [
//                         annotatable(1, 5, [
//                             simple_unit(1, 3),
//                             exponent(3, 5, [
//                                 sign(3, 4),
//                                 digits(4, 5)
//                             ])
//                         ]),
//                         annotation(6, 10)
//                    ])
//                ])
//             ]
//         };
//     }
//
//     #[test]
//     fn validate_component_with_annotation() {
//         parses_to! {
//             parser: UnitParser,
//             input: "2km-2{meow}",
//             rule: Rule::component,
//             tokens: [
//                 component(0, 11, [
//                     factor(0, 1),
//                     basic_component(1, 11, [
//                         annotatable(1, 5, [
//                             simple_unit(1, 3),
//                             exponent(3, 5, [
//                                 sign(3, 4),
//                                 digits(4, 5)
//                             ])
//                         ]),
//                         annotation(6, 10)
//                    ])
//                ])
//             ]
//         };
//     }
//
//     #[test]
//     fn validate_slash_term() {
//         let pairs = UnitParser::parse(Rule::term, "km/s");
//         assert!(pairs.is_ok());
//
//         parses_to! {
//             parser: UnitParser,
//             input: "2km-2{meow}/[acr_us].[in_i]",
//             rule: Rule::term,
//             tokens: [
//                 term(0, 27, [
//                     component(0, 11, [
//                         factor(0, 1),
//                         basic_component(1, 11, [
//                             annotatable(1, 5, [
//                                 simple_unit(1, 3),
//                                 exponent(3, 5, [
//                                     sign(3, 4),
//                                     digits(4, 5)
//                                 ])
//                             ]),
//                             annotation(6, 10)
//                        ])
//                     ]),
//                     slash(11, 12),
//                     term(12, 27, [
//                         component(12, 20, [
//                             basic_component(12, 20, [
//                                 annotatable(12, 20, [
//                                     simple_unit(12, 20)
//                                 ])
//                             ])
//                         ]),
//                         dot(20, 21),
//                         term(21, 27, [
//                             component(21, 27, [
//                                 basic_component(21, 27, [
//                                     annotatable(21, 27, [
//                                         simple_unit(21, 27)
//                                     ])
//                                 ])
//                             ])
//                         ])
//                     ])
//                ])
//             ]
//         };
//     }
//
//     #[test]
//     fn validate_interpret_term_with_dot_term_then_slash_component() {
//         parses_to! {
//             parser: UnitParser,
//             input: "[acr_us].[in_i]/[acr_us]",
//             rule: Rule::term,
//             tokens: [
//                 term(0, 24, [
//                     component(0, 8, [
//                         basic_component(0, 8, [
//                             annotatable(0, 8, [
//                                 simple_unit(0, 8)
//                            ])
//                         ])
//                     ]),
//                     dot(8, 9),
//                     term(9, 24, [
//                         component(9, 15, [
//                             basic_component(9, 15, [
//                                 annotatable(9, 15, [
//                                     simple_unit(9, 15)
//                                 ])
//                             ])
//                         ]),
//                         slash(15, 16),
//                         term(16, 24, [
//                             component(16, 24, [
//                                 basic_component(16, 24, [
//                                     annotatable(16, 24, [
//                                         simple_unit(16, 24)
//                                     ])
//                                 ])
//                             ])
//                         ])
//                     ])
//                 ])
//             ]
//         };
//     }
//
//     #[test]
//     fn validate_main_term() {
//         let pairs = UnitParser::parse(Rule::main_term, "km/s");
//         assert!(pairs.is_ok());
//
//         let pairs = UnitParser::parse(Rule::main_term, "/km.s");
//         assert!(pairs.is_ok());
//
//         parses_to! {
//             parser: UnitParser,
//             input: "/2m",
//             rule: Rule::main_term,
//             tokens: [
//                 main_term(0, 3, [
//                     slash(0, 1),
//                     term(1, 3, [
//                         component(1, 3, [
//                             factor(1, 2),
//                             basic_component(2, 3, [
//                                 annotatable(2, 3, [
//                                     simple_unit(2, 3)
//                                ])
//                             ])
//                         ])
//                     ]),
//                     EOI(3, 3)
//                 ])
//             ]
//         };
//
//         parses_to! {
//             parser: UnitParser,
//             input: "/1",
//             rule: Rule::main_term,
//             tokens: [
//                 main_term(0, 2, [
//                     slash(0, 1),
//                     term(1, 2, [
//                         component(1, 2, [
//                             basic_component(1, 2, [
//                                 factor(1, 2)
//                             ])
//                         ])
//                     ]),
//                     EOI(2, 2)
//                 ])
//             ]
//         }
//     }
// }
