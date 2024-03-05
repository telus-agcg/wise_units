use num_traits::Inv;
use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser as _};

use crate::parser;

#[derive(pest_derive::Parser)]
#[grammar = "parser/unit.pest"]
pub(crate) struct UnitParser;

#[derive(Debug, PartialEq)]
pub(super) enum MainTerm<'i> {
    SlashTerm(Term<'i>),
    Term(Term<'i>),
}

impl TryFrom<MainTerm<'_>> for crate::Unit {
    type Error = super::Error;

    fn try_from(main_term: MainTerm<'_>) -> Result<Self, Self::Error> {
        match main_term {
            MainTerm::SlashTerm(term) => {
                let unit = Self::try_from(term)?;
                Ok(unit.inv())
            }
            MainTerm::Term(term) => Self::try_from(term),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum Term<'i> {
    Multi {
        lhs: Box<Term<'i>>,
        op: Op,
        rhs: Box<Term<'i>>,
    },
    Single(Component<'i>),
}

impl TryFrom<Term<'_>> for crate::Unit {
    type Error = super::Error;

    fn try_from(term: Term<'_>) -> Result<Self, Self::Error> {
        match term {
            Term::Multi { lhs, op, rhs } => todo!(),
            Term::Single(component) => {
                crate::Term::try_from(component).map(|term| Self::new(vec![term]))
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum Op {
    Dot,
    Slash,
}

#[derive(Debug, PartialEq)]
pub(super) enum Component<'i> {
    Annotatable {
        factor: Option<&'i str>,
        annotatable: Annotatable<'i>,
        annotation: Option<&'i str>,
    },
    Factor {
        factor: &'i str,
        annotation: Option<&'i str>,
    },
    Annotation(&'i str),
    // ParenTerm(Box<Term<'i>>),
}

impl TryFrom<Component<'_>> for crate::Term {
    type Error = super::Error;

    fn try_from(component: Component<'_>) -> Result<Self, Self::Error> {
        match component {
            Component::Annotatable {
                factor,
                annotatable,
                annotation,
            } => {
                let maybe_factor = match factor {
                    Some(string_value) => {
                        let integer = string_value.parse()?;
                        Some(integer)
                    }
                    None => None,
                };

                let (prefix, atom, exponent) = { todo!() };

                Ok(Self {
                    factor: maybe_factor,
                    prefix,
                    atom,
                    exponent,
                    // TODO: Remove .to_string() with PLCC-291
                    annotation: annotation.map(ToString::to_string),
                })
            }
            Component::Factor { factor, annotation } => Ok(Self {
                factor: Some(factor.parse()?),
                prefix: None,
                atom: None,
                exponent: None,
                // TODO: Remove .to_string() with PLCC-291
                annotation: annotation.map(ToString::to_string),
            }),
            Component::Annotation(annotation) => Ok(Self {
                factor: None,
                prefix: None,
                atom: None,
                exponent: None,
                // TODO: Remove .to_string() with PLCC-291
                annotation: Some(annotation.to_string()),
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum Annotatable<'i> {
    SimpleUnitExponent {
        simple_unit: SimpleUnit<'i>,
        exponent: Exponent<'i>,
    },
    SimpleUnit(SimpleUnit<'i>),
}

#[derive(Debug, PartialEq)]
pub(super) enum Exponent<'i> {
    SignDigits { sign: &'i str, digits: &'i str },
    Digits(&'i str),
}

#[derive(Debug, PartialEq)]
pub(super) enum SimpleUnit<'i> {
    PrefixAtom {
        prefix_symbol: &'i str,
        atom_symbol: &'i str,
    },
    Atom(&'i str),
}

lazy_static::lazy_static! {
    static ref UNIT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::Left, Op};

        PrattParser::new()
            .op(Op::prefix(Rule::leading_slash) | Op::prefix(Rule::sign) | Op::prefix(Rule::factor) | Op::prefix(Rule::prefix_symbol))
            .op(Op::infix(Rule::dot, Left) | Op::infix(Rule::slash, Left))
            .op(Op::postfix(Rule::exponent)| Op::postfix(Rule::annotation_string) | Op::postfix(Rule::EOI))
    };
}

pub(super) fn parse(expr: &str) -> Result<MainTerm<'_>, parser::Error> {
    let pairs = UnitParser::parse(Rule::main_term, expr)?;

    UNIT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::main_term => parse_main_term(primary.into_inner(), &UNIT_PARSER),
            rule => unreachable!("expected main_term, found {rule:?}"),
        })
        .parse(pairs)
}

fn parse_main_term<'i>(
    pairs: Pairs<'i, Rule>,
    pratt: &PrattParser<Rule>,
) -> Result<MainTerm<'i>, parser::Error> {
    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::term => Ok(MainTerm::Term(parse_term(primary.into_inner(), pratt))),
            rule => unreachable!("expected term, found {rule:?}"),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::leading_slash => match rhs {
                Ok(MainTerm::Term(term)) => Ok(MainTerm::SlashTerm(term)),
                Ok(MainTerm::SlashTerm(_)) | Err(_) => unreachable!(),
            },
            rule => unreachable!("expected leading_slash, found {rule:?}"),
        })
        .map_postfix(|main_term, op| match op.as_rule() {
            Rule::EOI => main_term,
            rule => Err(parser::Error::UnknownUnitString(format!(
                "expected EOI, found {rule:?}"
            ))),
        })
        .parse(pairs)
}

fn parse_term<'i>(pairs: Pairs<'i, Rule>, pratt: &PrattParser<Rule>) -> Term<'i> {
    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::component => Term::Single(parse_component(primary.into_inner(), pratt)),
            rule => unreachable!("expected component, found {rule:?}"),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::dot => Op::Dot,
                Rule::slash => Op::Slash,
                rule => unreachable!("expected . or /, found {rule:?}"),
            };

            Term::Multi {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .parse(pairs)
}

fn parse_component<'i>(pairs: Pairs<'i, Rule>, pratt: &PrattParser<Rule>) -> Component<'i> {
    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::annotatable => Component::Annotatable {
                factor: None,
                annotatable: parse_annotatable(primary.into_inner(), pratt),
                annotation: None,
            },
            Rule::annotation_component => {
                Component::Annotation(parse_annotation(primary.into_inner()))
            }
            Rule::factor_component => Component::Factor {
                factor: primary.as_str(),
                annotation: None,
            },
            rule => unreachable!("expected inner component, found {rule:?}"),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::factor => match rhs {
                Component::Annotatable {
                    annotatable,
                    annotation,
                    ..
                } => Component::Annotatable {
                    factor: Some(op.as_str()),
                    annotatable,
                    annotation,
                },
                _ => unreachable!(),
            },
            rule => unreachable!("expected factor, found {rule:?}"),
        })
        .map_postfix(|annotatable, op| match op.as_rule() {
            Rule::annotation_string => match annotatable {
                Component::Annotatable {
                    factor,
                    annotatable,
                    ..
                } => Component::Annotatable {
                    factor,
                    annotatable,
                    annotation: Some(op.as_str()),
                },
                Component::Factor { factor, .. } => Component::Factor {
                    factor,
                    annotation: Some(op.as_str()),
                },
                Component::Annotation(_) => unreachable!(),
            },
            rule => unreachable!("expected annotation_string, found {rule:?}"),
        })
        .parse(pairs)
}

fn parse_annotation(mut pairs: Pairs<'_, Rule>) -> &str {
    pairs
        .next()
        .map_or_else(|| unreachable!(), |pair| pair.as_str())
}

fn parse_annotatable<'i>(pairs: Pairs<'i, Rule>, pratt: &PrattParser<Rule>) -> Annotatable<'i> {
    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::simple_unit => {
                Annotatable::SimpleUnit(parse_simple_unit(primary.into_inner(), pratt))
            }
            rule => unreachable!("expected term, found {rule:?}"),
        })
        .map_postfix(|annotatable, op| match op.as_rule() {
            Rule::exponent => match annotatable {
                Annotatable::SimpleUnit(simple_unit) => Annotatable::SimpleUnitExponent {
                    simple_unit,
                    exponent: parse_exponent(op.into_inner(), pratt),
                },
                ann @ Annotatable::SimpleUnitExponent { .. } => {
                    unreachable!("expected simple unit, got {ann:#?}")
                }
            },
            rule => unreachable!("expected factor, found {rule:?}"),
        })
        .parse(pairs)
}

fn parse_simple_unit<'i>(pairs: Pairs<'i, Rule>, pratt: &PrattParser<Rule>) -> SimpleUnit<'i> {
    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::metric_atom_symbol | Rule::non_metric_atom_symbol => {
                SimpleUnit::Atom(primary.as_str())
            }
            rule => unreachable!("expected *atom_symbol, found {rule:?}"),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::prefix_symbol => match rhs {
                SimpleUnit::Atom(atom_symbol) => SimpleUnit::PrefixAtom {
                    prefix_symbol: op.as_str(),
                    atom_symbol,
                },
                SimpleUnit::PrefixAtom { .. } => unreachable!(),
            },
            rule => unreachable!("expected factor, found {rule:?}"),
        })
        .parse(pairs)
}

fn parse_exponent<'i>(pairs: Pairs<'i, Rule>, pratt: &PrattParser<Rule>) -> Exponent<'i> {
    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::digits => Exponent::Digits(primary.as_str()),
            rule => unreachable!("expected digits, found {rule:?}"),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::sign => match rhs {
                Exponent::Digits(digits) => Exponent::SignDigits {
                    sign: op.as_str(),
                    digits,
                },
                Exponent::SignDigits { .. } => unreachable!(),
            },
            rule => unreachable!("expected factor, found {rule:?}"),
        })
        .parse(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod single_term {
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
        use super::*;

        #[test]
        fn atom_2_test() {
            pretty_assertions::assert_eq!(
                parse("m.g").unwrap(),
                MainTerm::Term(Term::Multi {
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
                })
            );
        }

        #[test]
        fn atom_3_test() {
            pretty_assertions::assert_eq!(
                parse("m.g.K").unwrap(),
                MainTerm::Term(Term::Multi {
                    lhs: Box::new(Term::Multi {
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
                    }),
                    op: Op::Dot,
                    rhs: Box::new(Term::Single(Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("K")),
                        annotation: None
                    }))
                })
            );
        }
    }

    mod multi_slash_term {
        use super::*;

        #[test]
        fn atom_2_test() {
            pretty_assertions::assert_eq!(
                parse("m/g").unwrap(),
                MainTerm::Term(Term::Multi {
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
                })
            );
        }

        #[test]
        fn atom_3_test() {
            pretty_assertions::assert_eq!(
                parse("m/g/K").unwrap(),
                MainTerm::Term(Term::Multi {
                    lhs: Box::new(Term::Multi {
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
                    }),
                    op: Op::Slash,
                    rhs: Box::new(Term::Single(Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit(SimpleUnit::Atom("K")),
                        annotation: None
                    }))
                })
            );
        }
    }

    mod slash_main_term {
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
