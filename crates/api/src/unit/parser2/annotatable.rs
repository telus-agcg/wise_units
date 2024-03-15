use nom::{branch::alt, character::complete::char, combinator::opt, IResult};

use crate::term;

use super::simple_unit::SimpleUnit;

#[derive(Debug, PartialEq)]
pub(super) enum Annotatable {
    SimpleUnitExponent {
        simple_unit: SimpleUnit,
        exponent: term::Exponent,
    },
    SimpleUnit(SimpleUnit),
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

fn parse_exponent(input: &[u8]) -> IResult<&[u8], term::Exponent> {
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
