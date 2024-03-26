use pest::iterators::Pair;

use crate::{
    parser::{terms::term_parser::Rule, Error, Visit},
    term::Factor,
};

use super::{Annotatable, Annotation, AstTerm, Finishable, Term};

pub(super) struct BasicComponent {
    pub(super) factor: Option<Factor>,
    pub(super) annotatable: Option<Annotatable>,
    pub(super) annotation: Option<String>,
    pub(super) terms: Vec<Term>,
}

impl Visit<Rule> for BasicComponent {
    fn visit(pair: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut pairs = pair.into_inner();

        let first_token = match pairs.next() {
            Some(first) => match first.as_rule() {
                Rule::annotatable => FirstToken::Annotatable(Annotatable::visit(first)?),
                Rule::annotation => {
                    return Ok(Self {
                        factor: None,
                        annotatable: None,
                        annotation: Some(Annotation::visit(first)?),
                        terms: Vec::with_capacity(0),
                    })
                }
                Rule::factor => FirstToken::Factor(Factor::visit(first)?),
                Rule::term => {
                    return Ok(Self {
                        factor: None,
                        annotatable: None,
                        annotation: None,
                        terms: AstTerm::visit(first)?.finish(),
                    })
                }
                _ => unreachable!(),
            },
            None => unreachable!(),
        };

        match first_token {
            FirstToken::Annotatable(annotatable) => match pairs.next() {
                Some(second) => match second.as_rule() {
                    Rule::annotation => Ok(Self {
                        factor: None,
                        annotatable: Some(annotatable),
                        annotation: Some(Annotation::visit(second)?),
                        terms: Vec::with_capacity(0),
                    }),
                    _ => unreachable!(),
                },
                None => Ok(Self {
                    factor: None,
                    annotatable: Some(annotatable),
                    annotation: None,
                    terms: Vec::with_capacity(0),
                }),
            },
            FirstToken::Factor(factor) => Ok(Self {
                factor: Some(factor),
                annotatable: None,
                annotation: None,
                terms: Vec::with_capacity(0),
            }),
        }
    }
}

enum FirstToken {
    Annotatable(Annotatable),
    Factor(Factor),
}

impl Finishable for BasicComponent {
    fn finish(self) -> Vec<Term> {
        let mut terms: Vec<Term> = Vec::with_capacity(self.terms.len() + 1);

        let self_term = if let Some(annotatable) = self.annotatable {
            match annotatable {
                Annotatable::PrefixedWithExponent {
                    prefix,
                    atom,
                    exponent,
                } => Term {
                    factor: self.factor,
                    prefix: Some(prefix),
                    atom: Some(atom),
                    exponent: Some(exponent),
                    annotation: self.annotation,
                },
                Annotatable::Prefixed { prefix, atom } => Term {
                    factor: self.factor,
                    prefix: Some(prefix),
                    atom: Some(atom),
                    exponent: None,
                    annotation: self.annotation,
                },
                Annotatable::BasicWithExponent { atom, exponent } => Term {
                    factor: self.factor,
                    prefix: None,
                    atom: Some(atom),
                    exponent: Some(exponent),
                    annotation: self.annotation,
                },
                Annotatable::Basic { atom } => Term {
                    factor: self.factor,
                    prefix: None,
                    atom: Some(atom),
                    exponent: None,
                    annotation: self.annotation,
                },
                Annotatable::Unity => Term {
                    factor: self.factor,
                    prefix: None,
                    atom: None,
                    exponent: None,
                    annotation: self.annotation,
                },
            }
        } else {
            Term {
                factor: self.factor,
                prefix: None,
                atom: None,
                exponent: None,
                annotation: self.annotation,
            }
        };

        terms.push(self_term);
        terms.extend_from_slice(&self.terms);

        terms
    }
}
