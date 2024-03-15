use nom::{branch::alt, bytes::complete::tag, multi::many0, sequence::pair, IResult, Parser};

use super::component::Component;

#[derive(Debug, PartialEq)]
pub(super) enum Term<'i> {
    Multi(MultiTerm<'i>),
    Single(Component<'i>),
}

#[derive(Debug, PartialEq)]
pub(super) struct MultiTerm<'i> {
    pub(super) lhs: Component<'i>,
    pub(super) rhs: Vec<(Op, Term<'i>)>,
}

#[derive(Debug, PartialEq)]
pub(super) enum Op {
    Dot,
    Slash,
}

pub(super) fn parse(input: &[u8]) -> IResult<&[u8], Term<'_>> {
    let first_parser = super::component::parse;

    let remainder_chunk_parser =
        pair(alt((tag("."), tag("/"))), parse).map(|(op_bytes, terms)| match &op_bytes[0..1] {
            b"." => (Op::Dot, terms),
            b"/" => (Op::Slash, terms),
            t => unreachable!(
                "expected '.' or '/' but got {}",
                std::str::from_utf8(t).unwrap()
            ),
        });
    let (tail, output) = pair(first_parser, many0(remainder_chunk_parser)).parse(input)?;

    dbg!(&output);

    if output.1.is_empty() {
        Ok((tail, Term::Single(output.0)))
    } else {
        Ok((
            tail,
            Term::Multi(MultiTerm {
                lhs: output.0,
                rhs: output.1,
            }),
        ))
    }
}

fn parse_component_term(input: &[u8]) -> IResult<&[u8], Term<'_>> {
    let (tail, component) = super::component::parse(input)?;

    Ok((tail, Term::Single(component)))
}
