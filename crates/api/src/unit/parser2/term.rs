use nom::{branch::alt, bytes::complete::tag, combinator::opt, sequence::pair, IResult, Parser};

use super::component::Component;

#[derive(Debug, PartialEq)]
pub(in crate::unit) struct Term<'i> {
    pub(super) lhs: Component<'i>,
    pub(super) rhs: Option<(Op, Box<Term<'i>>)>,
}

impl<'i> Term<'i> {
    fn flatten(self) -> Vec<(Op, Component<'i>)> {
        if let Some(r) = self.rhs {
            let mut output: Vec<_> = r.1.flatten();
            output.insert(0, (Op::Dot, self.lhs));
            output
        } else {
            vec![(Op::Dot, self.lhs)]
        }
    }
}

impl TryFrom<Term<'_>> for crate::Unit {
    type Error = ();

    fn try_from(term: Term<'_>) -> Result<Self, Self::Error> {
        let mut terms = vec![];

        for (op, component) in term.flatten() {
            let mut unit = Self::try_from(component).unwrap();

            if op == Op::Slash {
                unit = num_traits::Inv::inv(unit);
            }

            // TODO: Seems like this could be better.
            terms.extend(unit.into_terms().iter().cloned());
        }

        Ok(Self::new(terms))
    }
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
