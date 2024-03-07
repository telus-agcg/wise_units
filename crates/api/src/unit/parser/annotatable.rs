use std::num::ParseIntError;

use pest::{iterators::Pairs, pratt_parser::PrattParser};

use crate::parser::term;

use super::{Parse, Rule, SimpleUnit};

#[derive(Debug, PartialEq)]
pub(in crate::unit) enum Annotatable<'i> {
    SimpleUnitExponent {
        simple_unit: SimpleUnit<'i>,
        exponent: Exponent<'i>,
    },
    SimpleUnit(SimpleUnit<'i>),
}

impl<'i> Parse<'i> for Annotatable<'i> {
    fn parse(pairs: Pairs<'i, Rule>, pratt: &PrattParser<Rule>) -> Self {
        pratt
            .map_primary(|primary| match primary.as_rule() {
                Rule::simple_unit => {
                    Annotatable::SimpleUnit(SimpleUnit::parse(primary.into_inner(), pratt))
                }
                rule => unreachable!("expected term, found {rule:?}"),
            })
            .map_postfix(|annotatable, op| match op.as_rule() {
                Rule::exponent => match annotatable {
                    Annotatable::SimpleUnit(simple_unit) => Annotatable::SimpleUnitExponent {
                        simple_unit,
                        exponent: Exponent::parse(op.into_inner(), pratt),
                    },
                    ann @ Annotatable::SimpleUnitExponent { .. } => {
                        unreachable!("expected simple unit, got {ann:#?}")
                    }
                },
                rule => unreachable!("expected factor, found {rule:?}"),
            })
            .parse(pairs)
    }
}

#[derive(Debug, PartialEq)]
pub(in crate::unit) enum Exponent<'i> {
    SignDigits { sign: &'i str, digits: &'i str },
    Digits(&'i str),
}

impl<'i> Parse<'i> for Exponent<'i> {
    fn parse(pairs: Pairs<'i, Rule>, pratt: &PrattParser<Rule>) -> Self {
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
}

impl<'i> TryFrom<Exponent<'i>> for term::Exponent {
    type Error = ParseIntError;

    fn try_from(value: Exponent<'i>) -> Result<Self, Self::Error> {
        match value {
            Exponent::SignDigits { sign, digits } => {
                let n = digits.parse::<Self>()?;

                // This is ok because the parser already sussed out bad signs.
                match sign {
                    "-" => Ok(-n),
                    "+" => Ok(n),
                    _ => unreachable!("unknown sign token: {sign}"),
                }
            }
            Exponent::Digits(digits) => digits.parse::<Self>(),
        }
    }
}
