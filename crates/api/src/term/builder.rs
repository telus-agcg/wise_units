use crate::{Annotation, Atom, Prefix, Term};

use super::{
    variants::{
        AtomAnnotation, AtomExponent, AtomExponentAnnotation, FactorAnnotation, FactorAtom,
        FactorAtomAnnotation, FactorAtomExponent, FactorAtomExponentAnnotation, FactorExponent,
        FactorExponentAnnotation, FactorPrefixAtom, FactorPrefixAtomAnnotation,
        FactorPrefixAtomExponent, FactorPrefixAtomExponentAnnotation, PrefixAtom,
        PrefixAtomAnnotation, PrefixAtomExponent, PrefixAtomExponentAnnotation,
    },
    Exponent, Factor,
};

#[derive(Default, Debug)]
pub struct Builder {
    factor: Option<Factor>,
    prefix: Option<Prefix>,
    atom: Option<Atom>,
    exponent: Option<Exponent>,
    annotation: Option<Annotation>,
}

impl Builder {
    #[must_use]
    pub fn factor(self, factor: Factor) -> Self {
        Self {
            factor: Some(factor),
            ..self
        }
    }

    #[must_use]
    pub fn prefix(self, prefix: Prefix) -> Self {
        Self {
            prefix: Some(prefix),
            ..self
        }
    }

    #[must_use]
    pub fn atom(self, atom: Atom) -> Self {
        Self {
            atom: Some(atom),
            ..self
        }
    }

    #[must_use]
    pub fn exponent(self, exponent: Exponent) -> Self {
        Self {
            exponent: Some(exponent),
            ..self
        }
    }

    #[must_use]
    pub fn annotation<T>(self, annotation: T) -> Self
    where
        Annotation: From<T>,
    {
        Self {
            annotation: Some(Annotation::from(annotation)),
            ..self
        }
    }

    /// # Panics
    ///
    /// This will panic if it is called in such a way that would result in an invalid `Term`.
    ///
    #[must_use]
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
            (None, None, None, None, Some(annotation)) => annotation.into(),
            (None, None, Some(atom), None, None) => Term::Atom(atom),
            (None, None, Some(atom), None, Some(annotation)) => {
                AtomAnnotation::new(atom, annotation).into()
            }
            (None, None, Some(atom), Some(exponent), None) => {
                AtomExponent::new(atom, exponent).into()
            }
            (None, None, Some(atom), Some(exponent), Some(annotation)) => {
                AtomExponentAnnotation::new(atom, exponent, annotation).into()
            }
            (None, Some(prefix), Some(atom), None, None) => PrefixAtom { prefix, atom }.into(),
            (None, Some(prefix), Some(atom), None, Some(annotation)) => {
                PrefixAtomAnnotation::new(prefix, atom, annotation).into()
            }
            (None, Some(prefix), Some(atom), Some(exponent), None) => {
                PrefixAtomExponent::new(prefix, atom, exponent).into()
            }
            (None, Some(prefix), Some(atom), Some(exponent), Some(annotation)) => {
                PrefixAtomExponentAnnotation::new(prefix, atom, exponent, annotation).into()
            }
            (Some(factor), None, None, None, None) => Term::Factor(factor),
            (Some(factor), None, None, None, Some(annotation)) => {
                FactorAnnotation { factor, annotation }.into()
            }
            (Some(factor), None, None, Some(exponent), None) => {
                FactorExponent { factor, exponent }.into()
            }
            (Some(factor), None, None, Some(exponent), Some(annotation)) => {
                FactorExponentAnnotation::new(factor, exponent, annotation).into()
            }
            (Some(factor), None, Some(atom), None, None) => FactorAtom { factor, atom }.into(),
            (Some(factor), None, Some(atom), None, Some(annotation)) => {
                FactorAtomAnnotation::new(factor, atom, annotation).into()
            }
            (Some(factor), None, Some(atom), Some(exponent), None) => {
                FactorAtomExponent::new(factor, atom, exponent).into()
            }
            (Some(factor), None, Some(atom), Some(exponent), Some(annotation)) => {
                FactorAtomExponentAnnotation::new(factor, atom, exponent, annotation).into()
            }
            (Some(factor), Some(prefix), Some(atom), None, None) => {
                FactorPrefixAtom::new(factor, prefix, atom).into()
            }
            (Some(factor), Some(prefix), Some(atom), None, Some(annotation)) => {
                FactorPrefixAtomAnnotation::new(factor, prefix, atom, annotation).into()
            }
            (Some(factor), Some(prefix), Some(atom), Some(exponent), None) => {
                FactorPrefixAtomExponent::new(factor, prefix, atom, exponent).into()
            }
            (Some(factor), Some(prefix), Some(atom), Some(exponent), Some(annotation)) => {
                FactorPrefixAtomExponentAnnotation::new(factor, prefix, atom, exponent, annotation)
                    .into()
            }
        }
    }
}
