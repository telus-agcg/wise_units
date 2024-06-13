use crate::{Annotation, Atom, Prefix, Term};

use super::{variants::*, Exponent, Factor};

#[derive(Default, Debug)]
pub struct Builder {
    factor: Option<Factor>,
    prefix: Option<Prefix>,
    atom: Option<Atom>,
    exponent: Option<Exponent>,
    annotation: Option<String>,
}

impl Builder {
    pub fn factor(self, factor: Factor) -> Self {
        Self {
            factor: Some(factor),
            ..self
        }
    }
    pub fn prefix(self, prefix: Prefix) -> Self {
        Self {
            prefix: Some(prefix),
            ..self
        }
    }
    pub fn atom(self, atom: Atom) -> Self {
        Self {
            atom: Some(atom),
            ..self
        }
    }
    pub fn exponent(self, exponent: Exponent) -> Self {
        Self {
            exponent: Some(exponent),
            ..self
        }
    }
    pub fn annotation<T>(self, annotation: T) -> Self
    where
        T: ToString,
    {
        Self {
            annotation: Some(annotation.to_string()),
            ..self
        }
    }

    pub fn build(self) -> Term {
        match (
            self.factor,
            self.prefix,
            self.atom,
            self.exponent,
            self.annotation,
        ) {
            (None, None | Some(_), None, None | Some(_), None)
            | (None, None | Some(_), None, Some(_), Some(_))
            | (None | Some(_), Some(_), None, None, Some(_))
            | (Some(_), Some(_), None, None | Some(_), None)
            | (Some(_), Some(_), None, Some(_), Some(_)) => panic!("Invalid term fields"),
            (None, None, None, None, Some(annotation)) => Annotation::new(annotation).into(),
            (None, None, Some(atom), None, None) => Term::Atom(atom),
            (None, None, Some(atom), None, Some(annotation)) => AtomAnnotation {
                atom,
                annotation: Annotation::new(annotation),
            }
            .into(),
            (None, None, Some(atom), Some(exponent), None) => {
                AtomExponent { atom, exponent }.into()
            }
            (None, None, Some(atom), Some(exponent), Some(annotation)) => AtomExponentAnnotation {
                atom,
                exponent,
                annotation: Annotation::new(annotation),
            }
            .into(),
            (None, Some(prefix), Some(atom), None, None) => PrefixAtom { prefix, atom }.into(),
            (None, Some(prefix), Some(atom), None, Some(annotation)) => PrefixAtomAnnotation {
                prefix,
                atom,
                annotation: Annotation::new(annotation),
            }
            .into(),
            (None, Some(prefix), Some(atom), Some(exponent), None) => PrefixAtomExponent {
                prefix,
                atom,
                exponent,
            }
            .into(),
            (None, Some(prefix), Some(atom), Some(exponent), Some(annotation)) => {
                PrefixAtomExponentAnnotation {
                    prefix,
                    atom,
                    exponent,
                    annotation: Annotation::new(annotation),
                }
                .into()
            }
            (Some(factor), None, None, None, None) => Term::Factor(factor),
            (Some(factor), None, None, None, Some(annotation)) => FactorAnnotation {
                factor,
                annotation: Annotation::new(annotation),
            }
            .into(),
            (Some(factor), None, None, Some(exponent), None) => {
                FactorExponent { factor, exponent }.into()
            }
            (Some(factor), None, None, Some(exponent), Some(annotation)) => {
                FactorExponentAnnotation {
                    factor,
                    exponent,
                    annotation: Annotation::new(annotation),
                }
                .into()
            }
            (Some(factor), None, Some(atom), None, None) => FactorAtom { factor, atom }.into(),
            (Some(factor), None, Some(atom), None, Some(annotation)) => FactorAtomAnnotation {
                factor,
                atom,
                annotation: Annotation::new(annotation),
            }
            .into(),
            (Some(factor), None, Some(atom), Some(exponent), None) => FactorAtomExponent {
                factor,
                atom,
                exponent,
            }
            .into(),
            (Some(factor), None, Some(atom), Some(exponent), Some(annotation)) => {
                FactorAtomExponentAnnotation {
                    factor,
                    atom,
                    exponent,
                    annotation: Annotation::new(annotation),
                }
                .into()
            }
            (Some(factor), Some(prefix), Some(atom), None, None) => FactorPrefixAtom {
                factor,
                prefix,
                atom,
            }
            .into(),
            (Some(factor), Some(prefix), Some(atom), None, Some(annotation)) => {
                FactorPrefixAtomAnnotation {
                    factor,
                    prefix,
                    atom,
                    annotation: Annotation::new(annotation),
                }
                .into()
            }
            (Some(factor), Some(prefix), Some(atom), Some(exponent), None) => {
                FactorPrefixAtomExponent {
                    factor,
                    prefix,
                    atom,
                    exponent,
                }
                .into()
            }
            (Some(factor), Some(prefix), Some(atom), Some(exponent), Some(annotation)) => {
                FactorPrefixAtomExponentAnnotation {
                    factor,
                    prefix,
                    atom,
                    exponent,
                    annotation: Annotation::new(annotation),
                }
                .into()
            }
        }
    }
}
