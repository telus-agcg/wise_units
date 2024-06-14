mod atom_annotation;
mod atom_exponent;
mod atom_exponent_annotation;
mod factor_annotation;
mod factor_atom;
mod factor_atom_annotation;
mod factor_atom_exponent;
mod factor_atom_exponent_annotation;
mod factor_exponent;
mod factor_exponent_annotation;
mod factor_prefix_atom;
mod factor_prefix_atom_annotation;
mod factor_prefix_atom_exponent;
mod factor_prefix_atom_exponent_annotation;
mod prefix_atom;
mod prefix_atom_annotation;
mod prefix_atom_exponent;
mod prefix_atom_exponent_annotation;

pub use self::{
    atom_annotation::AtomAnnotation, atom_exponent::AtomExponent,
    atom_exponent_annotation::AtomExponentAnnotation, factor_annotation::FactorAnnotation,
    factor_atom::FactorAtom, factor_atom_annotation::FactorAtomAnnotation,
    factor_atom_exponent::FactorAtomExponent,
    factor_atom_exponent_annotation::FactorAtomExponentAnnotation, factor_exponent::FactorExponent,
    factor_exponent_annotation::FactorExponentAnnotation, factor_prefix_atom::FactorPrefixAtom,
    factor_prefix_atom_annotation::FactorPrefixAtomAnnotation,
    factor_prefix_atom_exponent::FactorPrefixAtomExponent,
    factor_prefix_atom_exponent_annotation::FactorPrefixAtomExponentAnnotation,
    prefix_atom::PrefixAtom, prefix_atom_annotation::PrefixAtomAnnotation,
    prefix_atom_exponent::PrefixAtomExponent,
    prefix_atom_exponent_annotation::PrefixAtomExponentAnnotation,
};

use std::mem;

use num_traits::{Inv, Pow};

use crate::Atom;

use super::{Annotation, Exponent, Factor, Term};

// ╭────────╮
// │ Traits │
// ╰────────╯
pub enum PowOutput<Z, O, R> {
    Zero(Z),
    One(O),
    Rest(R),
}

impl<Z, O, R> PowOutput<Z, O, R> {
    pub fn unwrap_rest(self) -> R {
        if let Self::Rest(inner) = self {
            inner
        } else {
            unreachable!()
        }
    }
}

impl<Z, O, R> From<PowOutput<Z, O, R>> for Term
where
    Self: From<Z> + From<O> + From<R>,
{
    fn from(value: PowOutput<Z, O, R>) -> Self {
        match value {
            PowOutput::Zero(inner) => Self::from(inner),
            PowOutput::One(inner) => Self::from(inner),
            PowOutput::Rest(inner) => Self::from(inner),
        }
    }
}

pub enum InvOutput<O, R> {
    One(O),
    Rest(R),
}

impl<O, R> From<InvOutput<O, R>> for Term
where
    Self: From<O> + From<R>,
{
    fn from(value: InvOutput<O, R>) -> Self {
        match value {
            InvOutput::One(inner) => Self::from(inner),
            InvOutput::Rest(inner) => Self::from(inner),
        }
    }
}

/// Trait for updating `self` with a new `factor`. If `self` already has a factor, the parameter
/// should replace the existing value; if `self` does not have a factor, the output should be
/// a new type that is `Self` + `Factor`.
///
pub(super) trait AssignFactor {
    type Output;

    fn assign_factor(&mut self, factor: Factor) -> Self::Output;
}

impl AssignFactor for Atom {
    type Output = FactorAtom;

    fn assign_factor(&mut self, factor: Factor) -> Self::Output {
        FactorAtom {
            factor,
            atom: *self,
        }
    }
}

/// Trait for updating `self` with a new `exponent`. If `self` already has an exponent, the
/// parameter should replace the existing value; if `self` does not have an exponent, the output
/// should be a new type that is `Self` + `Exponent`.
///
/// Also note that this doesn't check for exponent values--the caller should handle `0` and `1`
/// values accordingly, before calling this.
///
pub(super) trait SetExponent {
    type Output;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output;
}

/// Trait for doing the opposite of `AssignExponent`.
///
pub(crate) trait UnassignExponent {
    fn unassign_exponent(self) -> Term;
}

// ╭────────────╮
// │ Annotation │
// ╰────────────╯

impl AssignFactor for Annotation {
    type Output = FactorAnnotation;

    fn assign_factor(&mut self, factor: Factor) -> Self::Output {
        FactorAnnotation {
            factor,
            annotation: mem::take(self),
        }
    }
}

impl SetExponent for Annotation {
    type Output = PowOutput<(), (), FactorExponentAnnotation>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(()),
            1 => PowOutput::One(()),
            _ => PowOutput::Rest(FactorExponentAnnotation {
                factor: 1,
                exponent,
                annotation: mem::take(self),
            }),
        }
    }
}

impl Pow<Exponent> for Annotation {
    type Output = PowOutput<Self, Self, FactorExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(rhs) {
            PowOutput::Zero(()) => PowOutput::Zero(s),
            PowOutput::One(()) => PowOutput::One(s),
            PowOutput::Rest(fea) => PowOutput::Rest(fea),
        }
    }
}

impl<'a> Pow<Exponent> for &'a Annotation {
    type Output = PowOutput<Annotation, Annotation, FactorExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut new = self.clone();

        match new.set_exponent(rhs) {
            PowOutput::Zero(()) => PowOutput::Zero(new),
            PowOutput::One(()) => PowOutput::One(new),
            PowOutput::Rest(fea) => PowOutput::Rest(fea),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut Annotation {
    type Output = PowOutput<(), (), FactorExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(rhs)
    }
}

impl Inv for Annotation {
    type Output = FactorExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a Annotation {
    type Output = FactorExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut Annotation {
    type Output = FactorExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

// ╭──────╮
// │ Atom │
// ╰──────╯

impl SetExponent for Atom {
    type Output = PowOutput<Factor, (), AtomExponent>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(()),
            _ => PowOutput::Rest(AtomExponent {
                atom: *self,
                exponent,
            }),
        }
    }
}

// ╭────────╮
// │ Factor │
// ╰────────╯
impl SetExponent for Factor {
    type Output = PowOutput<(), (), FactorExponent>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => {
                *self = 1;
                PowOutput::Zero(())
            }
            1 => PowOutput::One(()),
            _ => PowOutput::Rest(FactorExponent {
                factor: *self,
                exponent,
            }),
        }
    }
}
