use num_traits::Inv;
use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser as _};

#[derive(pest_derive::Parser)]
#[grammar = "parser/terms/unit.pest"]
pub(crate) struct UnitParser;

#[derive(Debug, PartialEq)]
pub(super) enum MainTerm<'input> {
    SlashTerm(Term<'input>),
    Term(Term<'input>),
}
}

#[derive(Debug, PartialEq)]
pub(super) enum Term<'input> {
    Multi {
        lhs: Box<Term<'input>>,
        op: Op,
        rhs: Box<Term<'input>>,
    },
    Single(Component<'input>),
}

#[derive(Debug, PartialEq)]
pub(super) enum Op {
    Dot,
    Slash,
}

#[derive(Debug, PartialEq)]
pub(super) enum Component<'input> {
    Annotatable {
        factor: Option<&'input str>,
        annotatable: Annotatable<'input>,
        annotation: Option<&'input str>,
    },
    Factor {
        factor: &'input str,
        annotation: Option<&'input str>,
    },
    Annotation(&'input str),
    // ParenTerm(Box<Term<'input>>),
}

#[derive(Debug, PartialEq)]
pub(super) enum Annotatable<'input> {
    SimpleUnitExponent {
        simple_unit: &'input str,
        exponent: Exponent<'input>,
    },
    SimpleUnit(&'input str),
}

#[derive(Debug, PartialEq)]
pub(super) enum Exponent<'input> {
    SignDigits {
        sign: &'input str,
        digits: &'input str,
    },
    Digits(&'input str),
}

lazy_static::lazy_static! {
    static ref UNIT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::Left, Op};

        PrattParser::new()
            .op(Op::prefix(Rule::leading_slash) | Op::prefix(Rule::sign) | Op::prefix(Rule::factor))
            .op(Op::infix(Rule::dot, Left) | Op::infix(Rule::slash, Left))
            .op(Op::postfix(Rule::exponent)| Op::postfix(Rule::annotation_string))
    };
}

pub(super) fn parse(expr: &str) -> MainTerm<'_> {
    let pairs = UnitParser::parse(Rule::main_term, expr).unwrap();

    UNIT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::main_term => parse_main_term(primary.into_inner(), &UNIT_PARSER),
            _ => unreachable!(),
        })
        .parse(pairs)
}

fn parse_main_term<'i>(pairs: Pairs<'i, Rule>, pratt: &PrattParser<Rule>) -> MainTerm<'i> {
    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::term => MainTerm::Term(parse_term(primary.into_inner(), pratt)),
            _ => unreachable!(),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::leading_slash => match rhs {
                MainTerm::Term(term) => MainTerm::SlashTerm(term),
                MainTerm::SlashTerm(_) => unreachable!(),
            },
            rule => unreachable!("expected leading_slash, found {rule:?}"),
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
            Rule::simple_unit => Annotatable::SimpleUnit(primary.as_str()),
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
        fn symbol_test() {
            pretty_assertions::assert_eq!(
                parse("m"),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnit("m"),
                    annotation: None
                }))
            );
        }

        #[test]
        fn factor_symbol_test() {
            pretty_assertions::assert_eq!(
                parse("3m"),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: Some("3"),
                    annotatable: Annotatable::SimpleUnit("m"),
                    annotation: None
                }))
            );
        }

        #[test]
        fn symbol_exponent_test() {
            pretty_assertions::assert_eq!(
                parse("m2"),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: "m",
                        exponent: Exponent::Digits("2")
                    },
                    annotation: None
                }))
            );
        }

        #[test]
        fn symbol_negative_exponent_test() {
            pretty_assertions::assert_eq!(
                parse("m-2"),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: "m",
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
        fn symbol_positive_exponent_test() {
            pretty_assertions::assert_eq!(
                parse("m+2"),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: "m",
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
        fn factor_symbol_exponent_test() {
            pretty_assertions::assert_eq!(
                parse("3m2"),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: Some("3"),
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: "m",
                        exponent: Exponent::Digits("2")
                    },
                    annotation: None
                }))
            );
        }

        #[test]
        fn symbol_exponent_annotation_test() {
            pretty_assertions::assert_eq!(
                parse("m2{please_work}"),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: "m",
                        exponent: Exponent::Digits("2")
                    },
                    annotation: Some("please_work")
                }))
            );
        }

        #[test]
        fn factor_symbol_exponent_annotation_test() {
            pretty_assertions::assert_eq!(
                parse("3m2{please_work}"),
                MainTerm::Term(Term::Single(Component::Annotatable {
                    factor: Some("3"),
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: "m",
                        exponent: Exponent::Digits("2")
                    },
                    annotation: Some("please_work")
                }))
            );
        }

        #[test]
        fn annotation_test() {
            pretty_assertions::assert_eq!(
                parse("{please_work}"),
                MainTerm::Term(Term::Single(Component::Annotation("please_work")))
            );
        }

        #[test]
        fn factor_test() {
            pretty_assertions::assert_eq!(
                parse("42"),
                MainTerm::Term(Term::Single(Component::Factor {
                    factor: "42",
                    annotation: None
                }))
            );
        }

        #[test]
        fn factor_annotation_test() {
            pretty_assertions::assert_eq!(
                parse("42{things}"),
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
        fn symbol_2_test() {
            pretty_assertions::assert_eq!(
                parse("m.g"),
                MainTerm::Term(Term::Multi {
                    lhs: Box::new(Term::Single(Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit("m"),
                        annotation: None
                    })),
                    op: Op::Dot,
                    rhs: Box::new(Term::Single(Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit("g"),
                        annotation: None
                    }))
                })
            );
        }

        #[test]
        fn symbol_3_test() {
            pretty_assertions::assert_eq!(
                parse("m.g.K"),
                MainTerm::Term(Term::Multi {
                    lhs: Box::new(Term::Multi {
                        lhs: Box::new(Term::Single(Component::Annotatable {
                            factor: None,
                            annotatable: Annotatable::SimpleUnit("m"),
                            annotation: None
                        })),
                        op: Op::Dot,
                        rhs: Box::new(Term::Single(Component::Annotatable {
                            factor: None,
                            annotatable: Annotatable::SimpleUnit("g"),
                            annotation: None
                        }))
                    }),
                    op: Op::Dot,
                    rhs: Box::new(Term::Single(Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit("K"),
                        annotation: None
                    }))
                })
            );
        }
    }

    mod multi_slash_term {
        use super::*;

        #[test]
        fn symbol_2_test() {
            pretty_assertions::assert_eq!(
                parse("m/g"),
                MainTerm::Term(Term::Multi {
                    lhs: Box::new(Term::Single(Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit("m"),
                        annotation: None
                    })),
                    op: Op::Slash,
                    rhs: Box::new(Term::Single(Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit("g"),
                        annotation: None
                    }))
                })
            );
        }

        #[test]
        fn symbol_3_test() {
            pretty_assertions::assert_eq!(
                parse("m/g/K"),
                MainTerm::Term(Term::Multi {
                    lhs: Box::new(Term::Multi {
                        lhs: Box::new(Term::Single(Component::Annotatable {
                            factor: None,
                            annotatable: Annotatable::SimpleUnit("m"),
                            annotation: None
                        })),
                        op: Op::Slash,
                        rhs: Box::new(Term::Single(Component::Annotatable {
                            factor: None,
                            annotatable: Annotatable::SimpleUnit("g"),
                            annotation: None
                        }))
                    }),
                    op: Op::Slash,
                    rhs: Box::new(Term::Single(Component::Annotatable {
                        factor: None,
                        annotatable: Annotatable::SimpleUnit("K"),
                        annotation: None
                    }))
                })
            );
        }
    }

    mod slash_main_term {
        use super::*;

        #[test]
        fn factor_symbol_exponent_annotation_test() {
            pretty_assertions::assert_eq!(
                parse("/3m2{please_work}"),
                MainTerm::SlashTerm(Term::Single(Component::Annotatable {
                    factor: Some("3"),
                    annotatable: Annotatable::SimpleUnitExponent {
                        simple_unit: "m",
                        exponent: Exponent::Digits("2")
                    },
                    annotation: Some("please_work")
                }))
            );
        }
    }
}
