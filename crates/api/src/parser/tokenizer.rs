mod annotatable;
mod component;
mod main_term;
mod simple_unit;
mod term;

use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser as _};

use crate::parser;

use self::{
    annotatable::Annotatable, component::Component, main_term::MainTerm, simple_unit::SimpleUnit,
    term::Term,
};

#[derive(pest_derive::Parser)]
#[grammar = "parser/unit.pest"]
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
    fn try_parse(
        pairs: Pairs<'i, Rule>,
        pratt: &PrattParser<Rule>,
    ) -> Result<Self, crate::parser::Error>;
}

pub(super) fn parse(expr: &str) -> Result<MainTerm<'_>, parser::Error> {
    let pairs = UnitParser::parse(Rule::main_term, expr)?;

    UNIT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::main_term => MainTerm::try_parse(primary.into_inner(), &UNIT_PARSER),
            rule => unreachable!("expected main_term, found {rule:?}"),
        })
        .parse(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod single_term {
        use crate::parser::tokenizer::annotatable::Exponent;

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
        use crate::parser::tokenizer::term::{MultiTerm, Op};

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
        use crate::parser::tokenizer::term::{MultiTerm, Op};

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
        use crate::parser::tokenizer::annotatable::Exponent;

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
