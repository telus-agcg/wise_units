use pest::iterators::Pair;

use crate::{
    term::{
        variants::{
            AtomAnnotation, AtomExponent, AtomExponentAnnotation, FactorAnnotation, FactorAtom,
            FactorAtomAnnotation, FactorAtomExponent, FactorAtomExponentAnnotation,
            FactorPrefixAtom, FactorPrefixAtomAnnotation, FactorPrefixAtomExponent,
            FactorPrefixAtomExponentAnnotation, PrefixAtom, PrefixAtomAnnotation,
            PrefixAtomExponent, PrefixAtomExponentAnnotation,
        },
        Factor,
    },
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

        let self_term = if let Some(annotatable) = self.annotatable {
            match annotatable {
                Annotatable::PrefixedWithExponent {
                    prefix,
                    atom,
                    exponent,
                } => match (self.factor, self.annotation) {
                    (None, None) => {
                        Term::PrefixAtomExponent(PrefixAtomExponent::new(prefix, atom, exponent))
                    }
                    (None, Some(annotation)) => {
                        Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation::new(
                            prefix,
                            atom,
                            exponent,
                            Annotation::from(annotation),
                        ))
                    }
                    (Some(factor), None) => Term::FactorPrefixAtomExponent(
                        FactorPrefixAtomExponent::new(factor, prefix, atom, exponent),
                    ),
                    (Some(factor), Some(annotation)) => Term::FactorPrefixAtomExponentAnnotation(
                        FactorPrefixAtomExponentAnnotation::new(
                            factor,
                            prefix,
                            atom,
                            exponent,
                            Annotation::from(annotation),
                        ),
                    ),
                },
                Annotatable::Prefixed { prefix, atom } => match (self.factor, self.annotation) {
                    (None, None) => Term::PrefixAtom(PrefixAtom { prefix, atom }),
                    (None, Some(annotation)) => Term::PrefixAtomAnnotation(
                        PrefixAtomAnnotation::new(prefix, atom, Annotation::from(annotation)),
                    ),
                    (Some(factor), None) => {
                        Term::FactorPrefixAtom(FactorPrefixAtom::new(factor, prefix, atom))
                    }
                    (Some(factor), Some(annotation)) => {
                        Term::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation::new(
                            factor,
                            prefix,
                            atom,
                            Annotation::from(annotation),
                        ))
                    }
                },
                Annotatable::BasicWithExponent { atom, exponent } => {
                    match (self.factor, self.annotation) {
                        (None, None) => Term::AtomExponent(AtomExponent { atom, exponent }),
                        (None, Some(annotation)) => {
                            Term::AtomExponentAnnotation(AtomExponentAnnotation::new(
                                atom,
                                exponent,
                                Annotation::from(annotation),
                            ))
                        }
                        (Some(factor), None) => Term::FactorAtomExponent(FactorAtomExponent::new(
                            factor, atom, exponent,
                        )),
                        (Some(factor), Some(annotation)) => {
                            Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation::new(
                                factor,
                                atom,
                                exponent,
                                Annotation::from(annotation),
                            ))
                        }
                    }
                }
                Annotatable::Basic { atom } => match (self.factor, self.annotation) {
                    (None, None) => Term::Atom(atom),
                    (None, Some(annotation)) => Term::AtomAnnotation(AtomAnnotation::new(
                        atom,
                        Annotation::from(annotation),
                    )),
                    (Some(factor), None) => Term::FactorAtom(FactorAtom { factor, atom }),
                    (Some(factor), Some(annotation)) => Term::FactorAtomAnnotation(
                        FactorAtomAnnotation::new(factor, atom, Annotation::from(annotation)),
                    ),
                },
                Annotatable::Unity => match (self.factor, self.annotation) {
                    (None, None) => Term::Factor(1),
                    (None, Some(annotation)) => Term::Annotation(Annotation::from(annotation)),
                    (Some(factor), None) => {
                        if factor == 1 {
                            Term::Factor(factor)
                        } else {
                            unreachable!("Parser parsed a unity but also found a factor: {factor}")
                        }
                    }
                    (Some(factor), Some(annotation)) => Term::FactorAnnotation(
                        FactorAnnotation::new(factor, Annotation::from(annotation)),
                    ),
                },
            }
        } else {
            match (self.factor, self.annotation) {
                (None, None) => unreachable!("Parsed empty unit string for term"),
                (None, Some(annotation)) => Term::Annotation(Annotation::from(annotation)),
                (Some(factor), None) => Term::Factor(factor),
                (Some(factor), Some(annotation)) => Term::FactorAnnotation(FactorAnnotation::new(
                    factor,
                    Annotation::from(annotation),
                )),
            }
        };

        terms.push(self_term);
        terms.extend_from_slice(&self.terms);

        terms
    }
}
