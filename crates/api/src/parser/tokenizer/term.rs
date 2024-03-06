use num_traits::Inv;
use pest::{iterators::Pairs, pratt_parser::PrattParser};

use super::{Component, Parse, Rule};

#[derive(Debug, PartialEq)]
pub(in crate::parser) enum Term<'i> {
    Multi(MultiTerm<'i>),
    Single(Component<'i>),
}

#[derive(Debug, PartialEq)]
pub(in crate::parser) struct MultiTerm<'i> {
    pub(in crate::parser) lhs: Box<Term<'i>>,
    pub(in crate::parser) op: Op,
    pub(in crate::parser) rhs: Box<Term<'i>>,
}

impl<'i> MultiTerm<'i> {
    fn flatten(self) -> Vec<(Op, Component<'i>)> {
        match (*self.lhs, self.op, *self.rhs) {
            (Term::Multi(lhs), Op::Dot, Term::Multi(rhs)) => {
                let mut lhs_vec: Vec<_> = lhs.flatten();
                let rhs_vec: Vec<_> = rhs.flatten();
                lhs_vec.extend(rhs_vec);

                lhs_vec
            }
            (Term::Multi(lhs), Op::Dot, Term::Single(rhs)) => {
                let mut output: Vec<_> = lhs.flatten();
                output.push((Op::Dot, rhs));

                output
            }
            (Term::Multi(lhs), Op::Slash, Term::Multi(rhs)) => {
                let mut lhs_vec: Vec<_> = lhs.flatten();
                let rhs_vec: Vec<_> = rhs
                    .flatten()
                    .into_iter()
                    .map(|(op, t)| match op {
                        Op::Dot => (Op::Slash, t),
                        Op::Slash => (Op::Dot, t),
                    })
                    .collect();
                lhs_vec.extend(rhs_vec);
                lhs_vec
            }
            (Term::Multi(lhs), Op::Slash, Term::Single(rhs)) => {
                let mut output: Vec<_> = lhs.flatten();
                output.push((Op::Slash, rhs));
                output
            }
            (Term::Single(lhs), Op::Dot, Term::Multi(rhs)) => {
                let mut output: Vec<_> = rhs.flatten();
                output.insert(0, (Op::Dot, lhs));
                output
            }
            (Term::Single(lhs), Op::Dot, Term::Single(rhs)) => {
                vec![(Op::Dot, lhs), (Op::Dot, rhs)]
            }
            (Term::Single(lhs), Op::Slash, Term::Multi(rhs)) => {
                let mut output: Vec<_> = rhs
                    .flatten()
                    .into_iter()
                    .map(|(op, t)| match op {
                        Op::Dot => (Op::Slash, t),
                        Op::Slash => (Op::Dot, t),
                    })
                    .collect();
                output.insert(0, (Op::Dot, lhs));
                output
            }
            (Term::Single(lhs), Op::Slash, Term::Single(rhs)) => {
                vec![(Op::Dot, lhs), (Op::Slash, rhs)]
            }
        }
    }
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

                Term::Multi(MultiTerm {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                })
            })
            .parse(pairs)
    }
}

impl TryFrom<Term<'_>> for crate::Unit {
    type Error = crate::parser::Error;

    fn try_from(term: Term<'_>) -> Result<Self, Self::Error> {
        match term {
            Term::Multi(multi_term) => {
                let terms = multi_term
                    .flatten()
                    .into_iter()
                    .map(|(op, component)| -> Result<crate::Term, Self::Error> {
                        let mut t = crate::Term::try_from(component)?;

                        if op == Op::Slash {
                            t = t.inv();
                        }

                        Ok(t)
                    })
                    .collect::<Result<Vec<crate::Term>, Self::Error>>()?;
                Ok(Self::new(terms))
            }
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
