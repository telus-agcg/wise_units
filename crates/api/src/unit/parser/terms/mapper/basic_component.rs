use pest::iterators::Pair;

use crate::{
    term::{variants::FactorAnnotation, Factor, UNITY},
    unit::parser::{terms::term_parser::Rule, Error, Visit},
    Annotation,
};

use super::{Annotatable, Annotation as MapperAnnotation, AstTerm, Finishable, Term};

pub(super) struct BasicComponent<'a> {
    pub(super) factor: Option<Factor>,
    pub(super) annotatable: Option<Annotatable>,
    pub(super) annotation: Option<&'a str>,
    pub(super) terms: Vec<Term>,
}

impl<'a> Visit<'a, Rule> for BasicComponent<'a> {
    fn visit(pair: Pair<'a, Rule>) -> Result<Self, Error> {
        let mut pairs = pair.into_inner();

        let first_token = match pairs.next() {
            Some(first) => match first.as_rule() {
                Rule::annotatable => FirstToken::Annotatable(Annotatable::visit(first)?),
                Rule::annotation => {
                    return Ok(Self {
                        factor: None,
                        annotatable: None,
                        annotation: Some(MapperAnnotation::visit(first)?),
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
                        annotation: Some(MapperAnnotation::visit(second)?),
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

impl Finishable for BasicComponent<'_> {
    fn finish(self) -> Vec<Term> {
        let mut terms: Vec<Term> = Vec::with_capacity(self.terms.len() + 1);

        let self_term = match (self.factor, self.annotatable, self.annotation) {
            (None, None, None) => UNITY,
            (None, None, Some(annotation)) => Term::Annotation(Annotation::from(annotation)),
            (None, Some(Annotatable(term)), None) => term,
            (None, Some(Annotatable(mut term)), Some(annotation)) => {
                let _ = term.set_annotation(annotation);
                term
            }
            (Some(factor), None, None) => Term::Factor(factor),
            (Some(factor), None, Some(annotation)) => {
                Term::FactorAnnotation(FactorAnnotation::new(factor, Annotation::from(annotation)))
            }
            (Some(factor), Some(Annotatable(mut term)), None) => {
                let _ = term.set_factor(factor);
                term
            }
            (Some(factor), Some(Annotatable(mut term)), Some(annotation)) => {
                let _ = term.set_factor(factor).set_annotation(annotation);
                term
            }
        };

        terms.push(self_term);
        terms.extend_from_slice(&self.terms);

        terms
    }
}
