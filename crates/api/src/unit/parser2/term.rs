use nom::{branch::alt, bytes::complete::tag, combinator::opt, sequence::pair, IResult, Parser};

use super::component::Component;

#[derive(Debug, PartialEq)]
pub(super) struct Term<'i> {
    pub(super) lhs: Component<'i>,
    pub(super) rhs: Option<(Op, Box<Term<'i>>)>,
}

#[derive(Debug, PartialEq)]
pub(super) enum Op {
    Dot,
    Slash,
}

pub(super) fn parse(input: &[u8]) -> IResult<&[u8], Term<'_>> {
    let first_parser = super::component::parse;

    let remainder_chunk_parser =
        pair(alt((tag("."), tag("/"))), parse).map(|(op_bytes, term)| match &op_bytes[0..1] {
            b"." => (Op::Dot, Box::new(term)),
            b"/" => (Op::Slash, Box::new(term)),
            t => unreachable!(
                "expected '.' or '/' but got {}",
                std::str::from_utf8(t).unwrap()
            ),
        });

    pair(first_parser, opt(remainder_chunk_parser))
        .parse(input)
        .map(|(tail, (lhs, rhs))| (tail, Term { lhs, rhs }))
}
