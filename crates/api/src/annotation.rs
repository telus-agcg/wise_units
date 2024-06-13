use std::fmt;

use crate::{
    composable::ComposablyEq,
    term::{
        variants::{FactorAnnotation, FactorExponentAnnotation},
        Exponent,
    },
    Composable, IsCompatibleWith, Term,
};

#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Annotation(String);

impl Annotation {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Annotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{annotation}}}", annotation = self.0)
    }
}

impl From<Annotation> for Term {
    fn from(value: Annotation) -> Self {
        Self::Annotation(value)
    }
}

impl<'a, T> From<&'a T> for Annotation
where
    T: ToString + ?Sized,
{
    fn from(value: &'a T) -> Self {
        Self(value.to_string())
    }
}

impl ComposablyEq<Term> for Annotation {
    fn composably_eq(&self, rhs: &Term) -> Option<Exponent> {
        match rhs {
            Term::Annotation(inner) => self.composably_eq(inner),
            Term::FactorAnnotation(inner) => self.composably_eq(inner),
            Term::FactorExponentAnnotation(inner) => self.composably_eq(inner),
            _ => None,
        }
    }
}

impl ComposablyEq<Self> for Annotation {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self == rhs {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorAnnotation> for Annotation {
    fn composably_eq(&self, rhs: &FactorAnnotation) -> Option<Exponent> {
        if rhs.factor == 1 && self == &rhs.annotation {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorExponentAnnotation> for Annotation {
    fn composably_eq(&self, rhs: &FactorExponentAnnotation) -> Option<Exponent> {
        if rhs.factor == 1 && self == &rhs.annotation {
            Some(1 + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for Annotation {
    fn composition(&self) -> crate::Composition {
        crate::Composition::new_dimless()
    }
}

impl IsCompatibleWith<Term> for Annotation {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition() && Some(self.as_str()) == rhs.annotation()
    }
}

impl crate::term::term_reduce::TermReduce for Annotation {
    fn build(&self, exponent: Exponent) -> Term {
        if exponent == 1 {
            Term::Annotation(self.clone())
        } else {
            Term::FactorExponentAnnotation(FactorExponentAnnotation {
                factor: 1,
                exponent,
                annotation: self.clone(),
            })
        }
    }
}
