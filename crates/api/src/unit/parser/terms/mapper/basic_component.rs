use pest::iterators::Pair;

use crate::{
    term::{
        variants::{
            AtomAnnotation, AtomExponent, AtomExponentAnnotation, FactorAnnotation, FactorAtom,
            FactorAtomAnnotation, FactorAtomExponent, FactorAtomExponentAnnotation,
            FactorPrefixAtom, FactorPrefixAtomAnnotation, PrefixAtom, PrefixAtomAnnotation,
            PrefixAtomExponent,
        },
        Factor,
    },
    unit::parser::{terms::term_parser::Rule, Error, Visit},
    Annotation,
};

use super::{Annotatable, Annotation as MapperAnnotation, AstTerm, Finishable, Term};

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

impl Finishable for BasicComponent {
    fn finish(self) -> Vec<Term> {
        let mut terms: Vec<Term> = Vec::with_capacity(self.terms.len() + 1);

        let self_term = if let Some(annotatable) = self.annotatable {
            match annotatable {
                Annotatable::PrefixedWithExponent {
                    prefix,
                    atom,
                    exponent,
                } => Term::PrefixAtomExponent(PrefixAtomExponent {
                    prefix,
                    atom,
                    exponent,
                }),
                Annotatable::Prefixed { prefix, atom } => {
                    match (self.factor, self.annotation.as_ref()) {
                        (None, None) => Term::PrefixAtom(PrefixAtom { prefix, atom }),
                        (None, Some(annotation)) => {
                            Term::PrefixAtomAnnotation(PrefixAtomAnnotation {
                                prefix,
                                atom,
                                annotation: Annotation::from(annotation),
                            })
                        }
                        (Some(factor), None) => Term::FactorPrefixAtom(FactorPrefixAtom {
                            factor,
                            prefix,
                            atom,
                        }),
                        (Some(factor), Some(annotation)) => {
                            Term::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation {
                                factor,
                                prefix,
                                atom,
                                annotation: Annotation::from(annotation),
                            })
                        }
                    }
                }
                Annotatable::BasicWithExponent { atom, exponent } => {
                    match (self.factor, self.annotation.as_ref()) {
                        (None, None) => Term::AtomExponent(AtomExponent { atom, exponent }),
                        (None, Some(annotation)) => {
                            Term::AtomExponentAnnotation(AtomExponentAnnotation {
                                atom,
                                exponent,
                                annotation: Annotation::from(annotation),
                            })
                        }
                        (Some(factor), None) => Term::FactorAtomExponent(FactorAtomExponent {
                            factor,
                            atom,
                            exponent,
                        }),
                        (Some(factor), Some(annotation)) => {
                            Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                                factor,
                                atom,
                                exponent,
                                annotation: Annotation::from(annotation),
                            })
                        }
                    }
                }
                Annotatable::Basic { atom } => match (self.factor, self.annotation.as_ref()) {
                    (None, None) => Term::Atom(atom),
                    (None, Some(annotation)) => Term::AtomAnnotation(AtomAnnotation {
                        atom,
                        annotation: Annotation::from(annotation),
                    }),
                    (Some(factor), None) => Term::FactorAtom(FactorAtom { factor, atom }),
                    (Some(factor), Some(annotation)) => {
                        Term::FactorAtomAnnotation(FactorAtomAnnotation {
                            factor,
                            atom,
                            annotation: Annotation::from(annotation),
                        })
                    }
                },
                Annotatable::Unity => match (self.factor, self.annotation.as_ref()) {
                    (None, None) => Term::Factor(1),
                    (None, Some(annotation)) => Term::Annotation(Annotation::from(annotation)),
                    (Some(factor), None) => {
                        if factor == 1 {
                            Term::Factor(factor)
                        } else {
                            unreachable!("Parser parsed a unity but also found a factor: {factor}")
                        }
                    }
                    (Some(factor), Some(annotation)) => Term::FactorAnnotation(FactorAnnotation {
                        factor,
                        annotation: Annotation::from(annotation),
                    }),
                },
            }
        } else {
            match (self.factor, self.annotation) {
                (None, None) => unreachable!("Parsed empty unit string for term"),
                (None, Some(annotation)) => Term::Annotation(Annotation::new(annotation)),
                (Some(factor), None) => Term::Factor(factor),
                (Some(factor), Some(annotation)) => Term::FactorAnnotation(FactorAnnotation {
                    factor,
                    annotation: Annotation::new(annotation),
                }),
            }
        };

        terms.push(self_term);
        terms.extend_from_slice(&self.terms);

        terms
    }
}
