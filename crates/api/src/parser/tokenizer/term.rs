use pest::{iterators::Pairs, pratt_parser::PrattParser};

use super::{Component, Parse, Rule};

#[derive(Debug, PartialEq)]
pub(in crate::parser) enum Term<'i> {
    Multi {
        lhs: Box<Term<'i>>,
        op: Op,
        rhs: Box<Term<'i>>,
    },
    Single(Component<'i>),
}

impl<'i> Parse<'i> for Term<'i> {
    fn parse(pairs: Pairs<'i, Rule>, pratt: &PrattParser<Rule>) -> Self {
        pratt
            .map_primary(|primary| match primary.as_rule() {
                Rule::component => Term::Single(Component::parse(primary.into_inner(), pratt)),
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
}

impl TryFrom<Term<'_>> for crate::Unit {
    type Error = crate::parser::Error;

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
pub(in crate::parser) enum Op {
    Dot,
    Slash,
}
