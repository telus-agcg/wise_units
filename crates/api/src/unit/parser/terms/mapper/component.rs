use pest::iterators::Pair;

use crate::{
    term::Factor,
    unit::parser::{terms::term_parser::Rule, Error, Visit},
};

use super::{BasicComponent, Finishable, Term};

pub(super) struct Component {
    pub(super) factor: Option<Factor>,
    pub(super) terms: Vec<Term>,
}

impl Visit<'_, Rule> for Component {
    fn visit(pair: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut pairs = pair.into_inner();

        let factor = match pairs.next() {
            Some(first) => match first.as_rule() {
                Rule::factor => Factor::visit(first)?,
                Rule::basic_component => {
                    return Ok(Self {
                        factor: None,
                        terms: BasicComponent::visit(first)?.finish(),
                    });
                }
                _ => unreachable!(),
            },
            None => unreachable!(),
        };

        match pairs.next() {
            Some(second) => match second.as_rule() {
                Rule::basic_component => Ok(Self {
                    factor: Some(factor),
                    terms: BasicComponent::visit(second)?.finish(),
                }),
                _ => unreachable!(),
            },
            None => unreachable!(),
        }
    }
}

impl Finishable for Component {
    fn finish(mut self) -> Vec<Term> {
        if let Some(factor) = self.factor {
            if factor != 1 {
                if let Some(first_term) = self.terms.first_mut() {
                    let _ = first_term.set_factor(factor);
                }
            }
        }

        self.terms
    }
}
