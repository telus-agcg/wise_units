use nom::{branch::alt, character::complete::char, combinator::opt, IResult};

use crate::term::Exponent;

use super::simple_unit::SimpleUnit;

#[derive(Debug, PartialEq)]
pub(in crate::unit) enum Annotatable {
    SimpleUnitExponent {
        simple_unit: SimpleUnit,
        exponent: Exponent,
    },
    SimpleUnit(SimpleUnit),
}

impl Annotatable {
    pub(super) const fn extract_for_term(
        self,
    ) -> (Option<crate::Prefix>, crate::Atom, Option<Exponent>) {
        match self {
            Self::SimpleUnitExponent {
                simple_unit,
                exponent,
            } => {
                let (prefix, atom) = simple_unit.extract_for_term();

                (prefix, atom, Some(exponent))
            }
            Self::SimpleUnit(simple_unit) => {
                let (prefix, atom) = simple_unit.extract_for_term();

                (prefix, atom, None)
            }
        }
    }
}

pub(super) fn parse(input: &[u8]) -> IResult<&[u8], Annotatable> {
    let (tail, simple_unit) = super::simple_unit::parse(input)?;

    match opt(parse_exponent)(tail)? {
        (tail, Some(exponent)) => Ok((
            tail,
            Annotatable::SimpleUnitExponent {
                simple_unit,
                exponent,
            },
        )),
        (tail, None) => Ok((tail, Annotatable::SimpleUnit(simple_unit))),
    }
}

fn parse_exponent(input: &[u8]) -> IResult<&[u8], Exponent> {
    match opt(alt((char('-'), char('+'))))(input)? {
        (tail, Some(sign)) => {
            let (tail, mut exponent) = nom::character::complete::i32(tail)?;

            if sign == '-' {
                exponent = -exponent;
            }

            Ok((tail, exponent))
        }
        (tail, None) => nom::character::complete::i32(tail),
    }
}
