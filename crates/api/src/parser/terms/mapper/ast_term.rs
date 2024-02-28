use pest::iterators::{Pair, Pairs};

use crate::parser::terms::term_parser::Rule;

use super::{Component, Error, Finishable, Term, Visit};

pub(super) struct AstTerm {
    pub(super) component: Component,
    pub(super) terms: Vec<Term>,
}

impl AstTerm {
    pub(super) fn visit(pair: Pair<'_, Rule>) -> Result<Self, Error> {
        fn parse_component(pairs: &mut Pairs<'_, Rule>) -> Result<Component, Error> {
            pairs.next().map_or_else(
                || unreachable!(),
                |first| match first.as_rule() {
                    Rule::component => Component::visit(first),
                    _ => unreachable!(),
                },
            )
        }

        fn parse_dot_or_slash(pairs: &mut Pairs<'_, Rule>) -> Option<SecondToken> {
            match pairs.next()?.as_rule() {
                Rule::dot => Some(SecondToken::Dot),
                Rule::slash => Some(SecondToken::Slash),
                _ => unreachable!(),
            }
        }

        fn parse_tail(
            component: Component,
            op: SecondToken,
            pairs: &mut Pairs<'_, Rule>,
        ) -> Result<AstTerm, Error> {
            match pairs.next() {
                Some(third) => match third.as_rule() {
                    Rule::term => {
                        let mut new_terms: Vec<Term> = AstTerm::visit(third)?.finish();

                        match op {
                            SecondToken::Dot => (),
                            SecondToken::Slash => {
                                crate::parser::term::num_traits::inv::inv_terms(&mut new_terms);
                            }
                        }

                        Ok(AstTerm {
                            component,
                            terms: new_terms,
                        })
                    }
                    _ => unreachable!(),
                },
                None => unreachable!(),
            }
        }

        let mut pairs = pair.into_inner();

        let component = parse_component(&mut pairs)?;

        let Some(op) = parse_dot_or_slash(&mut pairs) else {
            return Ok(Self {
                component,
                terms: Vec::with_capacity(0),
            });
        };

        parse_tail(component, op, &mut pairs)
    }
}

#[derive(Clone, Copy)]
enum SecondToken {
    Dot,
    Slash,
}

impl Finishable for AstTerm {
    fn finish(self) -> Vec<Term> {
        let mut component_terms = self.component.finish();
        component_terms.extend(self.terms);

        component_terms
    }
}
