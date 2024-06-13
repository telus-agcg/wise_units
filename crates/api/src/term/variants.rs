use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::{Atom, Prefix};

use super::{Annotation, Exponent, Factor, Term};

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
pub(super) trait AssignFactorMut {
    type Output;

    fn assign_factor_mut(&mut self, factor: Factor) -> Self::Output;
}

impl AssignFactorMut for Atom {
    type Output = FactorAtom;

    fn assign_factor_mut(&mut self, factor: Factor) -> Self::Output {
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
pub(super) trait AssignExponent {
    fn assign_exponent(self, exponent: Exponent) -> Term;
}

macro_rules! impl_assign_exponent {
    ($t:ident) => {
        impl AssignExponent for $t {
            fn assign_exponent(self, exponent: Exponent) -> Term {
                Term::$t(Self { exponent, ..self })
            }
        }
    };
}

impl AssignExponent for Atom {
    fn assign_exponent(self, exponent: Exponent) -> Term {
        Term::AtomExponent(AtomExponent {
            atom: self,
            exponent,
        })
    }
}

impl AssignExponent for Factor {
    fn assign_exponent(self, exponent: Exponent) -> Term {
        Term::FactorExponent(FactorExponent {
            factor: self,
            exponent,
        })
    }
}

/// Trait for doing the opposite of `AssignExponent`.
///
pub(crate) trait UnassignExponent {
    fn unassign_exponent(self) -> Term;
}

// ╭────────────╮
// │ Annotation │
// ╰────────────╯

impl AssignFactorMut for Annotation {
    type Output = FactorAnnotation;

    fn assign_factor_mut(&mut self, factor: Factor) -> Self::Output {
        FactorAnnotation {
            factor,
            annotation: mem::take(self),
        }
    }
}

impl AssignExponent for Annotation {
    fn assign_exponent(self, exponent: Exponent) -> Term {
        Term::FactorExponentAnnotation(FactorExponentAnnotation {
            factor: 1,
            exponent,
            annotation: self,
        })
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

impl Pow<Exponent> for Annotation {
    type Output = PowOutput<Self, Self, FactorExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(self),
            1 => PowOutput::One(self),
            exponent => PowOutput::Rest(FactorExponentAnnotation {
                factor: 1,
                exponent,
                annotation: self,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a Annotation {
    type Output = PowOutput<Annotation, Annotation, FactorExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(self.clone()),
            1 => PowOutput::One(self.clone()),
            exponent => PowOutput::Rest(FactorExponentAnnotation {
                factor: 1,
                exponent,
                annotation: self.clone(),
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut Annotation {
    type Output = PowOutput<(), (), FactorExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(()),
            1 => PowOutput::One(()),
            exponent => PowOutput::Rest(FactorExponentAnnotation {
                factor: 1,
                exponent,
                annotation: mem::take(self),
            }),
        }
    }
}

// ╭────────────────╮
// │ AtomAnnotation │
// ╰────────────────╯
/// Ex. "g{sucrose}"
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtomAnnotation {
    pub(crate) atom: Atom,
    pub(crate) annotation: Annotation,
}

impl AtomAnnotation {
    #[must_use]
    pub const fn new(atom: Atom, annotation: Annotation) -> Self {
        Self { atom, annotation }
    }
}

impl AssignFactorMut for AtomAnnotation {
    type Output = FactorAtomAnnotation;

    fn assign_factor_mut(&mut self, factor: Factor) -> Self::Output {
        FactorAtomAnnotation {
            factor,
            atom: self.atom,
            annotation: mem::take(&mut self.annotation),
        }
    }
}

impl AssignExponent for AtomAnnotation {
    fn assign_exponent(self, exponent: Exponent) -> Term {
        Term::AtomExponentAnnotation(AtomExponentAnnotation {
            atom: self.atom,
            exponent,
            annotation: self.annotation,
        })
    }
}

impl From<AtomAnnotation> for Term {
    fn from(value: AtomAnnotation) -> Self {
        Self::AtomAnnotation(value)
    }
}

impl fmt::Display for AtomAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{atom}{annotation}",
            atom = self.atom,
            annotation = self.annotation
        )
    }
}

impl Inv for AtomAnnotation {
    type Output = AtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a AtomAnnotation {
    type Output = AtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut AtomAnnotation {
    type Output = AtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl Pow<Exponent> for AtomAnnotation {
    type Output = PowOutput<Annotation, Self, AtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(self.annotation),
            1 => PowOutput::One(self),
            exponent => PowOutput::Rest(AtomExponentAnnotation {
                atom: self.atom,
                exponent,
                annotation: self.annotation,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a AtomAnnotation {
    type Output = PowOutput<Annotation, AtomAnnotation, AtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(self.annotation.clone()),
            1 => PowOutput::One(self.clone()),
            exponent => PowOutput::Rest(AtomExponentAnnotation {
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut AtomAnnotation {
    type Output = PowOutput<&'a mut Annotation, (), AtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(&mut self.annotation),
            1 => PowOutput::One(()),
            exponent => PowOutput::Rest(AtomExponentAnnotation {
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}

// ╭──────────────╮
// │ AtomExponent │
// ╰──────────────╯
/// Ex. "m2"
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtomExponent {
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
}

impl AtomExponent {
    #[must_use]
    pub const fn new(atom: Atom, exponent: Exponent) -> Self {
        Self { atom, exponent }
    }
}

impl AssignFactorMut for AtomExponent {
    type Output = FactorAtomExponent;

    fn assign_factor_mut(&mut self, factor: Factor) -> Self::Output {
        FactorAtomExponent {
            factor,
            atom: self.atom,
            exponent: self.exponent,
        }
    }
}

impl_assign_exponent!(AtomExponent);

impl UnassignExponent for AtomExponent {
    fn unassign_exponent(self) -> Term {
        Term::Atom(self.atom)
    }
}

impl From<AtomExponent> for Term {
    fn from(value: AtomExponent) -> Self {
        Self::AtomExponent(value)
    }
}

impl fmt::Display for AtomExponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent == 1 {
            write!(f, "{}", self.atom)
        } else {
            write!(f, "{}{}", self.atom, self.exponent)
        }
    }
}

impl Inv for AtomExponent {
    type Output = InvOutput<Atom, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(atom) => InvOutput::One(atom),
            PowOutput::Rest(atom_exponent) => InvOutput::Rest(atom_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut AtomExponent {
    type Output = InvOutput<Atom, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(atom) => InvOutput::One(atom),
            PowOutput::Rest(atom_exponent) => InvOutput::Rest(atom_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl Pow<Exponent> for AtomExponent {
    type Output = PowOutput<Factor, Atom, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(self.atom),
            exponent => PowOutput::Rest(Self {
                atom: self.atom,
                exponent,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut AtomExponent {
    type Output = PowOutput<Factor, Atom, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(self.atom),
            exponent => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

// ╭────────────────────────╮
// │ AtomExponentAnnotation │
// ╰────────────────────────╯
/// Ex. "m2{peaches}"
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtomExponentAnnotation {
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
    pub(crate) annotation: Annotation,
}

impl AtomExponentAnnotation {
    #[must_use]
    pub const fn new(atom: Atom, exponent: Exponent, annotation: Annotation) -> Self {
        Self {
            atom,
            exponent,
            annotation,
        }
    }
}

impl AssignFactorMut for AtomExponentAnnotation {
    type Output = FactorAtomExponentAnnotation;

    fn assign_factor_mut(&mut self, factor: Factor) -> Self::Output {
        FactorAtomExponentAnnotation {
            factor,
            atom: self.atom,
            exponent: self.exponent,
            annotation: mem::take(&mut self.annotation),
        }
    }
}

impl_assign_exponent!(AtomExponentAnnotation);

impl UnassignExponent for AtomExponentAnnotation {
    fn unassign_exponent(self) -> Term {
        Term::AtomAnnotation(AtomAnnotation {
            atom: self.atom,
            annotation: self.annotation,
        })
    }
}

impl From<AtomExponentAnnotation> for Term {
    fn from(value: AtomExponentAnnotation) -> Self {
        Self::AtomExponentAnnotation(value)
    }
}

impl fmt::Display for AtomExponentAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent == 1 {
            write!(f, "{}{}", self.atom, self.annotation)
        } else {
            write!(f, "{}{}{}", self.atom, self.exponent, self.annotation)
        }
    }
}

impl Inv for AtomExponentAnnotation {
    type Output = InvOutput<AtomAnnotation, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(atom_annotation) => InvOutput::One(atom_annotation),
            PowOutput::Rest(atom_exponent_annotation) => InvOutput::Rest(atom_exponent_annotation),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a AtomExponentAnnotation {
    type Output = InvOutput<AtomAnnotation, AtomExponentAnnotation>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(atom_annotation) => InvOutput::One(atom_annotation),
            PowOutput::Rest(atom_exponent_annotation) => InvOutput::Rest(atom_exponent_annotation),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut AtomExponentAnnotation {
    type Output = InvOutput<AtomAnnotation, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(atom_annotation) => InvOutput::One(atom_annotation),
            PowOutput::Rest(atom_exponent_annotation) => InvOutput::Rest(atom_exponent_annotation),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl Pow<Exponent> for AtomExponentAnnotation {
    type Output = PowOutput<Annotation, AtomAnnotation, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(self.annotation),
            1 => PowOutput::One(AtomAnnotation {
                atom: self.atom,
                annotation: self.annotation,
            }),
            exponent => PowOutput::Rest(Self {
                atom: self.atom,
                exponent,
                annotation: self.annotation,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a AtomExponentAnnotation {
    type Output = PowOutput<Annotation, AtomAnnotation, AtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(self.annotation.clone()),
            1 => PowOutput::One(AtomAnnotation {
                atom: self.atom,
                annotation: self.annotation.clone(),
            }),
            exponent => PowOutput::Rest(AtomExponentAnnotation {
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut AtomExponentAnnotation {
    type Output = PowOutput<&'a mut Annotation, AtomAnnotation, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(&mut self.annotation),
            1 => PowOutput::One(AtomAnnotation {
                atom: self.atom,
                annotation: mem::take(&mut self.annotation),
            }),
            exponent => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

// ╭────────────╮
// │ PrefixAtom │
// ╰────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrefixAtom {
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
}

impl PrefixAtom {
    #[must_use]
    pub const fn new(prefix: Prefix, atom: Atom) -> Self {
        Self { prefix, atom }
    }
}

impl AssignFactorMut for PrefixAtom {
    type Output = FactorPrefixAtom;

    fn assign_factor_mut(&mut self, factor: Factor) -> Self::Output {
        FactorPrefixAtom {
            factor,
            prefix: self.prefix,
            atom: self.atom,
        }
    }
}

impl AssignExponent for PrefixAtom {
    fn assign_exponent(self, exponent: Exponent) -> Term {
        Term::PrefixAtomExponent(PrefixAtomExponent {
            prefix: self.prefix,
            atom: self.atom,
            exponent,
        })
    }
}

impl From<PrefixAtom> for Term {
    fn from(value: PrefixAtom) -> Self {
        Self::PrefixAtom(value)
    }
}

impl fmt::Display for PrefixAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.prefix, self.atom,)
    }
}

impl Inv for PrefixAtom {
    type Output = PrefixAtomExponent;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut PrefixAtom {
    type Output = PrefixAtomExponent;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl Pow<Exponent> for PrefixAtom {
    type Output = PowOutput<Factor, Self, PrefixAtomExponent>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(self),
            exponent => PowOutput::Rest(PrefixAtomExponent {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut PrefixAtom {
    type Output = PowOutput<Factor, (), PrefixAtomExponent>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(()),
            exponent => PowOutput::Rest(PrefixAtomExponent {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
            }),
        }
    }
}

// ╭──────────────────────╮
// │ PrefixAtomAnnotation │
// ╰──────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrefixAtomAnnotation {
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
    pub(crate) annotation: Annotation,
}

impl PrefixAtomAnnotation {
    #[must_use]
    pub const fn new(prefix: Prefix, atom: Atom, annotation: Annotation) -> Self {
        Self {
            prefix,
            atom,
            annotation,
        }
    }
}

impl AssignFactorMut for PrefixAtomAnnotation {
    type Output = FactorPrefixAtomAnnotation;

    fn assign_factor_mut(&mut self, factor: Factor) -> Self::Output {
        FactorPrefixAtomAnnotation {
            factor,
            prefix: self.prefix,
            atom: self.atom,
            annotation: mem::take(&mut self.annotation),
        }
    }
}

impl AssignExponent for PrefixAtomAnnotation {
    fn assign_exponent(self, exponent: Exponent) -> Term {
        Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
            prefix: self.prefix,
            atom: self.atom,
            exponent,
            annotation: self.annotation,
        })
    }
}

impl From<PrefixAtomAnnotation> for Term {
    fn from(value: PrefixAtomAnnotation) -> Self {
        Self::PrefixAtomAnnotation(value)
    }
}

impl fmt::Display for PrefixAtomAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.prefix, self.atom, self.annotation)
    }
}

impl Inv for PrefixAtomAnnotation {
    type Output = PrefixAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a PrefixAtomAnnotation {
    type Output = PrefixAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut PrefixAtomAnnotation {
    type Output = PrefixAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl Pow<Exponent> for PrefixAtomAnnotation {
    type Output = PowOutput<Annotation, Self, PrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(self.annotation),
            1 => PowOutput::One(self),
            exponent => PowOutput::Rest(PrefixAtomExponentAnnotation {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: self.annotation,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a PrefixAtomAnnotation {
    type Output = PowOutput<Annotation, PrefixAtomAnnotation, PrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(self.annotation.clone()),
            1 => PowOutput::One(self.clone()),
            exponent => PowOutput::Rest(PrefixAtomExponentAnnotation {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut PrefixAtomAnnotation {
    type Output = PowOutput<&'a mut Annotation, (), PrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(&mut self.annotation),
            1 => PowOutput::One(()),
            exponent => PowOutput::Rest(PrefixAtomExponentAnnotation {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: mem::take(&mut self.annotation),
            }),
        }
    }
}

// ╭────────────────────╮
// │ PrefixAtomExponent │
// ╰────────────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrefixAtomExponent {
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
}

impl PrefixAtomExponent {
    #[must_use]
    pub const fn new(prefix: Prefix, atom: Atom, exponent: Exponent) -> Self {
        Self {
            prefix,
            atom,
            exponent,
        }
    }
}

impl AssignFactorMut for PrefixAtomExponent {
    type Output = FactorPrefixAtomExponent;

    fn assign_factor_mut(&mut self, factor: Factor) -> Self::Output {
        FactorPrefixAtomExponent {
            factor,
            prefix: self.prefix,
            atom: self.atom,
            exponent: self.exponent,
        }
    }
}

impl_assign_exponent!(PrefixAtomExponent);

impl UnassignExponent for PrefixAtomExponent {
    fn unassign_exponent(self) -> Term {
        Term::PrefixAtom(PrefixAtom {
            prefix: self.prefix,
            atom: self.atom,
        })
    }
}

impl From<PrefixAtomExponent> for Term {
    fn from(value: PrefixAtomExponent) -> Self {
        Self::PrefixAtomExponent(value)
    }
}

impl fmt::Display for PrefixAtomExponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent == 1 {
            write!(f, "{}{}", self.prefix, self.atom)
        } else {
            write!(f, "{}{}{}", self.prefix, self.atom, self.exponent)
        }
    }
}

impl Inv for PrefixAtomExponent {
    type Output = InvOutput<PrefixAtom, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(prefix_atom) => InvOutput::One(prefix_atom),
            PowOutput::Rest(prefix_atom_exponent) => InvOutput::Rest(prefix_atom_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut PrefixAtomExponent {
    type Output = InvOutput<PrefixAtom, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(prefix_atom) => InvOutput::One(prefix_atom),
            PowOutput::Rest(prefix_atom_exponent) => InvOutput::Rest(prefix_atom_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl Pow<Exponent> for PrefixAtomExponent {
    type Output = PowOutput<Factor, PrefixAtom, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(PrefixAtom {
                prefix: self.prefix,
                atom: self.atom,
            }),
            exponent => PowOutput::Rest(Self {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut PrefixAtomExponent {
    type Output = PowOutput<Factor, PrefixAtom, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(PrefixAtom {
                prefix: self.prefix,
                atom: self.atom,
            }),
            exponent => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

// ╭──────────────────────────────╮
// │ PrefixAtomExponentAnnotation │
// ╰──────────────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrefixAtomExponentAnnotation {
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
    pub(crate) annotation: Annotation,
}

impl PrefixAtomExponentAnnotation {
    #[must_use]
    pub const fn new(
        prefix: Prefix,
        atom: Atom,
        exponent: Exponent,
        annotation: Annotation,
    ) -> Self {
        Self {
            prefix,
            atom,
            exponent,
            annotation,
        }
    }
}

impl AssignFactorMut for PrefixAtomExponentAnnotation {
    type Output = FactorPrefixAtomExponentAnnotation;

    fn assign_factor_mut(&mut self, factor: Factor) -> Self::Output {
        FactorPrefixAtomExponentAnnotation {
            factor,
            prefix: self.prefix,
            atom: self.atom,
            exponent: self.exponent,
            annotation: mem::take(&mut self.annotation),
        }
    }
}

impl_assign_exponent!(PrefixAtomExponentAnnotation);

impl UnassignExponent for PrefixAtomExponentAnnotation {
    fn unassign_exponent(self) -> Term {
        Term::PrefixAtomAnnotation(PrefixAtomAnnotation {
            prefix: self.prefix,
            atom: self.atom,
            annotation: self.annotation,
        })
    }
}

impl From<PrefixAtomExponentAnnotation> for Term {
    fn from(value: PrefixAtomExponentAnnotation) -> Self {
        Self::PrefixAtomExponentAnnotation(value)
    }
}

impl fmt::Display for PrefixAtomExponentAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent == 1 {
            write!(
                f,
                "{prefix}{atom}{annotation}",
                prefix = self.prefix,
                atom = self.atom,
                annotation = self.annotation
            )
        } else {
            write!(
                f,
                "{prefix}{atom}{exponent}{annotation}",
                prefix = self.prefix,
                atom = self.atom,
                exponent = self.exponent,
                annotation = self.annotation
            )
        }
    }
}

impl Inv for PrefixAtomExponentAnnotation {
    type Output = InvOutput<PrefixAtomAnnotation, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(prefix_atom_annotation) => InvOutput::One(prefix_atom_annotation),
            PowOutput::Rest(prefix_atom_exponent_annotation) => {
                InvOutput::Rest(prefix_atom_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a PrefixAtomExponentAnnotation {
    type Output = InvOutput<PrefixAtomAnnotation, PrefixAtomExponentAnnotation>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(prefix_atom_annotation) => InvOutput::One(prefix_atom_annotation),
            PowOutput::Rest(prefix_atom_exponent_annotation) => {
                InvOutput::Rest(prefix_atom_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut PrefixAtomExponentAnnotation {
    type Output = InvOutput<PrefixAtomAnnotation, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(prefix_atom_annotation) => InvOutput::One(prefix_atom_annotation),
            PowOutput::Rest(prefix_atom_exponent_annotation) => {
                InvOutput::Rest(prefix_atom_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl Pow<Exponent> for PrefixAtomExponentAnnotation {
    type Output = PowOutput<Annotation, PrefixAtomAnnotation, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(self.annotation),
            1 => PowOutput::One(PrefixAtomAnnotation {
                prefix: self.prefix,
                atom: self.atom,
                annotation: self.annotation,
            }),
            exponent => PowOutput::Rest(Self {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: self.annotation,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a PrefixAtomExponentAnnotation {
    type Output = PowOutput<Annotation, PrefixAtomAnnotation, PrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(self.annotation.clone()),
            1 => PowOutput::One(PrefixAtomAnnotation {
                prefix: self.prefix,
                atom: self.atom,
                annotation: self.annotation.clone(),
            }),
            exponent => PowOutput::Rest(PrefixAtomExponentAnnotation {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut PrefixAtomExponentAnnotation {
    type Output = PowOutput<&'a mut Annotation, PrefixAtomAnnotation, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(&mut self.annotation),
            1 => PowOutput::One(PrefixAtomAnnotation {
                prefix: self.prefix,
                atom: self.atom,
                annotation: mem::take(&mut self.annotation),
            }),
            exponent => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

// ╭──────────────────╮
// │ FactorAnnotation │
// ╰──────────────────╯
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorAnnotation {
    pub(crate) factor: Factor,
    pub(crate) annotation: Annotation,
}

impl FactorAnnotation {
    #[must_use]
    pub const fn new(factor: Factor, annotation: Annotation) -> Self {
        Self { factor, annotation }
    }
}

impl AssignExponent for FactorAnnotation {
    fn assign_exponent(self, exponent: Exponent) -> Term {
        Term::FactorExponentAnnotation(FactorExponentAnnotation {
            factor: self.factor,
            exponent,
            annotation: self.annotation,
        })
    }
}

impl From<FactorAnnotation> for Term {
    fn from(value: FactorAnnotation) -> Self {
        Self::FactorAnnotation(value)
    }
}

impl fmt::Display for FactorAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.factor == 1 {
            write!(f, "{}", self.annotation)
        } else {
            write!(f, "{}{}", self.factor, self.annotation)
        }
    }
}

impl Inv for FactorAnnotation {
    type Output = FactorExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a FactorAnnotation {
    type Output = FactorExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut FactorAnnotation {
    type Output = FactorExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl Pow<Exponent> for FactorAnnotation {
    type Output = PowOutput<Annotation, Self, FactorExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(self.annotation),
            1 => PowOutput::One(self),
            exponent => PowOutput::Rest(FactorExponentAnnotation {
                factor: self.factor,
                exponent,
                annotation: self.annotation,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a FactorAnnotation {
    type Output = PowOutput<Annotation, FactorAnnotation, FactorExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(self.annotation.clone()),
            1 => PowOutput::One(self.clone()),
            exponent => PowOutput::Rest(FactorExponentAnnotation {
                factor: self.factor,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorAnnotation {
    type Output = PowOutput<&'a mut Annotation, (), FactorExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(&mut self.annotation),
            1 => PowOutput::One(()),
            exponent => PowOutput::Rest(FactorExponentAnnotation {
                factor: self.factor,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}

// ╭────────────────╮
// │ FactorExponent │
// ╰────────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FactorExponent {
    pub(crate) factor: Factor,
    pub(crate) exponent: Exponent,
}

impl FactorExponent {
    #[must_use]
    pub const fn new(factor: Factor, exponent: Exponent) -> Self {
        Self { factor, exponent }
    }
}

impl_assign_exponent!(FactorExponent);

impl UnassignExponent for FactorExponent {
    fn unassign_exponent(self) -> Term {
        Term::Factor(self.factor)
    }
}

impl From<FactorExponent> for Term {
    fn from(value: FactorExponent) -> Self {
        Self::FactorExponent(value)
    }
}

impl fmt::Display for FactorExponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent.is_negative() {
            write!(
                f,
                "{factor}-{exponent}",
                factor = self.factor,
                exponent = self.exponent
            )
        } else {
            write!(
                f,
                "{factor}+{exponent}",
                factor = self.factor,
                exponent = self.exponent
            )
        }
    }
}

impl Inv for FactorExponent {
    type Output = InvOutput<Factor, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor) => InvOutput::One(factor),
            PowOutput::Rest(factor_exponent) => InvOutput::Rest(factor_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut FactorExponent {
    type Output = InvOutput<Factor, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor) => InvOutput::One(factor),
            PowOutput::Rest(factor_exponent) => InvOutput::Rest(factor_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl Pow<Exponent> for FactorExponent {
    type Output = PowOutput<Factor, Factor, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(self.factor),
            exponent => PowOutput::Rest(Self {
                factor: self.factor,
                exponent,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorExponent {
    type Output = PowOutput<Factor, Factor, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(self.factor),
            exponent => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

// ╭──────────────────────────╮
// │ FactorExponentAnnotation │
// ╰──────────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorExponentAnnotation {
    pub(crate) factor: Factor,
    pub(crate) exponent: Exponent,
    pub(crate) annotation: Annotation,
}

impl FactorExponentAnnotation {
    #[must_use]
    pub const fn new(factor: Factor, exponent: Exponent, annotation: Annotation) -> Self {
        Self {
            factor,
            exponent,
            annotation,
        }
    }
}

impl_assign_exponent!(FactorExponentAnnotation);

impl UnassignExponent for FactorExponentAnnotation {
    fn unassign_exponent(self) -> Term {
        Term::FactorAnnotation(FactorAnnotation {
            factor: self.factor,
            annotation: self.annotation,
        })
    }
}

impl From<FactorExponentAnnotation> for Term {
    fn from(value: FactorExponentAnnotation) -> Self {
        Self::FactorExponentAnnotation(value)
    }
}

impl fmt::Display for FactorExponentAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.factor, self.exponent) {
            (0, 0) | (1, 1) => self.annotation.fmt(f),
            (factor, exponent) if exponent.is_negative() => {
                write!(
                    f,
                    "{factor}-{exponent}{annotation}",
                    exponent = self.exponent,
                    annotation = self.annotation
                )
            }
            (factor, exponent) => {
                write!(
                    f,
                    "{factor}+{exponent}{annotation}",
                    annotation = self.annotation
                )
            }
        }
    }
}

impl Inv for FactorExponentAnnotation {
    type Output = InvOutput<FactorAnnotation, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_annotation) => InvOutput::One(factor_annotation),
            PowOutput::Rest(factor_exponent_annotation) => {
                InvOutput::Rest(factor_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a FactorExponentAnnotation {
    type Output = InvOutput<FactorAnnotation, FactorExponentAnnotation>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_annotation) => InvOutput::One(factor_annotation),
            PowOutput::Rest(factor_exponent_annotation) => {
                InvOutput::Rest(factor_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut FactorExponentAnnotation {
    type Output = InvOutput<FactorAnnotation, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_annotation) => InvOutput::One(factor_annotation),
            PowOutput::Rest(factor_exponent_annotation) => {
                InvOutput::Rest(factor_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl Pow<Exponent> for FactorExponentAnnotation {
    type Output = PowOutput<Annotation, FactorAnnotation, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(self.annotation),
            1 => PowOutput::One(FactorAnnotation {
                factor: self.factor,
                annotation: self.annotation,
            }),
            exponent => PowOutput::Rest(Self {
                factor: self.factor,
                exponent,
                annotation: self.annotation,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a FactorExponentAnnotation {
    type Output = PowOutput<Annotation, FactorAnnotation, FactorExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(self.annotation.clone()),
            1 => PowOutput::One(FactorAnnotation {
                factor: self.factor,
                annotation: self.annotation.clone(),
            }),
            exponent => PowOutput::Rest(FactorExponentAnnotation {
                factor: self.factor,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorExponentAnnotation {
    type Output = PowOutput<&'a mut Annotation, FactorAnnotation, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(&mut self.annotation),
            1 => PowOutput::One(FactorAnnotation {
                factor: self.factor,
                annotation: mem::take(&mut self.annotation),
            }),
            exponent => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

// ╭────────────╮
// │ FactorAtom │
// ╰────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FactorAtom {
    pub(crate) factor: Factor,
    pub(crate) atom: Atom,
}

impl FactorAtom {
    #[must_use]
    pub const fn new(factor: Factor, atom: Atom) -> Self {
        Self { factor, atom }
    }
}

impl AssignExponent for FactorAtom {
    fn assign_exponent(self, exponent: Exponent) -> Term {
        Term::FactorAtomExponent(FactorAtomExponent {
            factor: self.factor,
            atom: self.atom,
            exponent,
        })
    }
}

impl From<FactorAtom> for Term {
    fn from(value: FactorAtom) -> Self {
        Self::FactorAtom(value)
    }
}

impl fmt::Display for FactorAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.factor == 1 {
            write!(f, "{}", self.atom)
        } else {
            write!(f, "{}{}", self.factor, self.atom)
        }
    }
}

impl Inv for FactorAtom {
    type Output = FactorAtomExponent;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut FactorAtom {
    type Output = FactorAtomExponent;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl Pow<Exponent> for FactorAtom {
    type Output = PowOutput<Factor, Self, FactorAtomExponent>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(self),
            exponent => PowOutput::Rest(FactorAtomExponent {
                factor: self.factor,
                atom: self.atom,
                exponent,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorAtom {
    type Output = PowOutput<Factor, (), FactorAtomExponent>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(()),
            exponent => PowOutput::Rest(FactorAtomExponent {
                factor: self.factor,
                atom: self.atom,
                exponent,
            }),
        }
    }
}

// ╭──────────────────────╮
// │ FactorAtomAnnotation │
// ╰──────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorAtomAnnotation {
    pub(crate) factor: Factor,
    pub(crate) atom: Atom,
    pub(crate) annotation: Annotation,
}

impl FactorAtomAnnotation {
    #[must_use]
    pub const fn new(factor: Factor, atom: Atom, annotation: Annotation) -> Self {
        Self {
            factor,
            atom,
            annotation,
        }
    }
}

impl AssignExponent for FactorAtomAnnotation {
    fn assign_exponent(self, exponent: Exponent) -> Term {
        Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
            factor: self.factor,
            atom: self.atom,
            exponent,
            annotation: self.annotation,
        })
    }
}

impl From<FactorAtomAnnotation> for Term {
    fn from(value: FactorAtomAnnotation) -> Self {
        Self::FactorAtomAnnotation(value)
    }
}

impl fmt::Display for FactorAtomAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.factor == 1 {
            write!(f, "{}{}", self.atom, self.annotation)
        } else {
            write!(f, "{}{}{}", self.factor, self.atom, self.annotation)
        }
    }
}

impl Inv for FactorAtomAnnotation {
    type Output = FactorAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a FactorAtomAnnotation {
    type Output = FactorAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut FactorAtomAnnotation {
    type Output = FactorAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl Pow<Exponent> for FactorAtomAnnotation {
    type Output = PowOutput<Annotation, Self, FactorAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(self.annotation),
            1 => PowOutput::One(self),
            exponent => PowOutput::Rest(FactorAtomExponentAnnotation {
                factor: self.factor,
                atom: self.atom,
                exponent,
                annotation: self.annotation,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a FactorAtomAnnotation {
    type Output = PowOutput<Annotation, FactorAtomAnnotation, FactorAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(self.annotation.clone()),
            1 => PowOutput::One(self.clone()),
            exponent => PowOutput::Rest(FactorAtomExponentAnnotation {
                factor: self.factor,
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorAtomAnnotation {
    type Output = PowOutput<&'a mut Annotation, (), FactorAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(&mut self.annotation),
            1 => PowOutput::One(()),
            exponent => PowOutput::Rest(FactorAtomExponentAnnotation {
                factor: self.factor,
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}

// ╭────────────────────╮
// │ FactorAtomExponent │
// ╰────────────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FactorAtomExponent {
    pub(crate) factor: Factor,
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
}

impl FactorAtomExponent {
    #[must_use]
    pub const fn new(factor: Factor, atom: Atom, exponent: Exponent) -> Self {
        Self {
            factor,
            atom,
            exponent,
        }
    }
}

impl_assign_exponent!(FactorAtomExponent);

impl UnassignExponent for FactorAtomExponent {
    fn unassign_exponent(self) -> Term {
        Term::FactorAtom(FactorAtom {
            factor: self.factor,
            atom: self.atom,
        })
    }
}

impl From<FactorAtomExponent> for Term {
    fn from(value: FactorAtomExponent) -> Self {
        Self::FactorAtomExponent(value)
    }
}

impl fmt::Display for FactorAtomExponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.factor, self.exponent) {
            (1, 1) => {
                write!(f, "{}", self.atom)
            }
            (1, exponent) => {
                write!(f, "{atom}{exponent}", atom = self.atom)
            }
            (factor, 1) => {
                write!(f, "{factor}{atom}", atom = self.atom)
            }
            (factor, exponent) => {
                write!(f, "{factor}{atom}{exponent}", atom = self.atom)
            }
        }
    }
}

impl Inv for FactorAtomExponent {
    type Output = InvOutput<FactorAtom, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_atom) => InvOutput::One(factor_atom),
            PowOutput::Rest(factor_atom_exponent) => InvOutput::Rest(factor_atom_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut FactorAtomExponent {
    type Output = InvOutput<FactorAtom, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_atom) => InvOutput::One(factor_atom),
            PowOutput::Rest(factor_atom_exponent) => InvOutput::Rest(factor_atom_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl Pow<Exponent> for FactorAtomExponent {
    type Output = PowOutput<Factor, FactorAtom, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(FactorAtom {
                factor: self.factor,
                atom: self.atom,
            }),
            exponent => PowOutput::Rest(Self {
                factor: self.factor,
                atom: self.atom,
                exponent,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorAtomExponent {
    type Output = PowOutput<Factor, FactorAtom, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(FactorAtom {
                factor: self.factor,
                atom: self.atom,
            }),
            exponent => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

// ╭──────────────────────────────╮
// │ FactorAtomExponentAnnotation │
// ╰──────────────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorAtomExponentAnnotation {
    pub(crate) factor: Factor,
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
    pub(crate) annotation: Annotation,
}

impl FactorAtomExponentAnnotation {
    #[must_use]
    pub const fn new(
        factor: Factor,
        atom: Atom,
        exponent: Exponent,
        annotation: Annotation,
    ) -> Self {
        Self {
            factor,
            atom,
            exponent,
            annotation,
        }
    }
}

impl_assign_exponent!(FactorAtomExponentAnnotation);

impl UnassignExponent for FactorAtomExponentAnnotation {
    fn unassign_exponent(self) -> Term {
        Term::FactorAtomAnnotation(FactorAtomAnnotation {
            factor: self.factor,
            atom: self.atom,
            annotation: self.annotation,
        })
    }
}

impl From<FactorAtomExponentAnnotation> for Term {
    fn from(value: FactorAtomExponentAnnotation) -> Self {
        Self::FactorAtomExponentAnnotation(value)
    }
}

impl fmt::Display for FactorAtomExponentAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.factor, self.exponent) {
            (1, 1) => {
                write!(f, "{}{}", self.atom, self.annotation)
            }
            (1, exponent) => {
                write!(
                    f,
                    "{atom}{exponent}{annotation}",
                    atom = self.atom,
                    annotation = self.annotation
                )
            }
            (factor, 1) => {
                write!(
                    f,
                    "{factor}{atom}{annotation}",
                    atom = self.atom,
                    annotation = self.annotation
                )
            }
            (factor, exponent) => {
                write!(
                    f,
                    "{factor}{atom}{exponent}{annotation}",
                    atom = self.atom,
                    annotation = self.annotation
                )
            }
        }
    }
}

impl Inv for FactorAtomExponentAnnotation {
    type Output = InvOutput<FactorAtomAnnotation, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_atom_annotation) => InvOutput::One(factor_atom_annotation),
            PowOutput::Rest(factor_atom_exponent_annotation) => {
                InvOutput::Rest(factor_atom_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a FactorAtomExponentAnnotation {
    type Output = InvOutput<FactorAtomAnnotation, FactorAtomExponentAnnotation>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_atom_annotation) => InvOutput::One(factor_atom_annotation),
            PowOutput::Rest(factor_atom_exponent_annotation) => {
                InvOutput::Rest(factor_atom_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut FactorAtomExponentAnnotation {
    type Output = InvOutput<FactorAtomAnnotation, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_atom_annotation) => InvOutput::One(factor_atom_annotation),
            PowOutput::Rest(factor_atom_exponent_annotation) => {
                InvOutput::Rest(factor_atom_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl Pow<Exponent> for FactorAtomExponentAnnotation {
    type Output = PowOutput<Annotation, FactorAtomAnnotation, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(self.annotation),
            1 => PowOutput::One(FactorAtomAnnotation {
                factor: self.factor,
                atom: self.atom,
                annotation: self.annotation,
            }),
            exponent => PowOutput::Rest(Self {
                factor: self.factor,
                atom: self.atom,
                exponent,
                annotation: self.annotation,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a FactorAtomExponentAnnotation {
    type Output = PowOutput<Annotation, FactorAtomAnnotation, FactorAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(self.annotation.clone()),
            1 => PowOutput::One(FactorAtomAnnotation {
                factor: self.factor,
                atom: self.atom,
                annotation: self.annotation.clone(),
            }),
            exponent => PowOutput::Rest(FactorAtomExponentAnnotation {
                factor: self.factor,
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorAtomExponentAnnotation {
    type Output = PowOutput<&'a mut Annotation, FactorAtomAnnotation, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(&mut self.annotation),
            1 => PowOutput::One(FactorAtomAnnotation {
                factor: self.factor,
                atom: self.atom,
                annotation: mem::take(&mut self.annotation),
            }),
            exponent => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

// ╭──────────────────╮
// │ FactorPrefixAtom │
// ╰──────────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FactorPrefixAtom {
    pub(crate) factor: Factor,
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
}

impl FactorPrefixAtom {
    #[must_use]
    pub const fn new(factor: Factor, prefix: Prefix, atom: Atom) -> Self {
        Self {
            factor,
            prefix,
            atom,
        }
    }
}

impl AssignExponent for FactorPrefixAtom {
    fn assign_exponent(self, exponent: Exponent) -> Term {
        Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent {
            factor: self.factor,
            prefix: self.prefix,
            atom: self.atom,
            exponent,
        })
    }
}

impl From<FactorPrefixAtom> for Term {
    fn from(value: FactorPrefixAtom) -> Self {
        Self::FactorPrefixAtom(value)
    }
}

impl fmt::Display for FactorPrefixAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.factor == 1 {
            write!(f, "{}{}", self.prefix, self.atom,)
        } else {
            write!(
                f,
                "{factor}{prefix}{atom}",
                factor = self.factor,
                prefix = self.prefix,
                atom = self.atom,
            )
        }
    }
}

impl Inv for FactorPrefixAtom {
    type Output = FactorPrefixAtomExponent;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut FactorPrefixAtom {
    type Output = FactorPrefixAtomExponent;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl Pow<Exponent> for FactorPrefixAtom {
    type Output = PowOutput<Factor, Self, FactorPrefixAtomExponent>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(self),
            exponent => PowOutput::Rest(FactorPrefixAtomExponent {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                exponent,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorPrefixAtom {
    type Output = PowOutput<Factor, (), FactorPrefixAtomExponent>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(()),
            exponent => PowOutput::Rest(FactorPrefixAtomExponent {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                exponent,
            }),
        }
    }
}

// ╭────────────────────────────╮
// │ FactorPrefixAtomAnnotation │
// ╰────────────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorPrefixAtomAnnotation {
    pub(crate) factor: Factor,
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
    pub(crate) annotation: Annotation,
}

impl FactorPrefixAtomAnnotation {
    #[must_use]
    pub const fn new(factor: Factor, prefix: Prefix, atom: Atom, annotation: Annotation) -> Self {
        Self {
            factor,
            prefix,
            atom,
            annotation,
        }
    }
}

impl AssignExponent for FactorPrefixAtomAnnotation {
    fn assign_exponent(self, exponent: Exponent) -> Term {
        Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
            factor: self.factor,
            prefix: self.prefix,
            atom: self.atom,
            exponent,
            annotation: self.annotation,
        })
    }
}

impl From<FactorPrefixAtomAnnotation> for Term {
    fn from(value: FactorPrefixAtomAnnotation) -> Self {
        Self::FactorPrefixAtomAnnotation(value)
    }
}

impl fmt::Display for FactorPrefixAtomAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.factor == 1 {
            write!(
                f,
                "{prefix}{atom}{annotation}",
                prefix = self.prefix,
                atom = self.atom,
                annotation = self.annotation
            )
        } else {
            write!(
                f,
                "{factor}{prefix}{atom}{annotation}",
                factor = self.factor,
                prefix = self.prefix,
                atom = self.atom,
                annotation = self.annotation
            )
        }
    }
}

impl Inv for FactorPrefixAtomAnnotation {
    type Output = FactorPrefixAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a FactorPrefixAtomAnnotation {
    type Output = FactorPrefixAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut FactorPrefixAtomAnnotation {
    type Output = FactorPrefixAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl Pow<Exponent> for FactorPrefixAtomAnnotation {
    type Output = PowOutput<Annotation, Self, FactorPrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(self.annotation),
            1 => PowOutput::One(self),
            exponent => PowOutput::Rest(FactorPrefixAtomExponentAnnotation {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: self.annotation,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a FactorPrefixAtomAnnotation {
    type Output =
        PowOutput<Annotation, FactorPrefixAtomAnnotation, FactorPrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(self.annotation.clone()),
            1 => PowOutput::One(self.clone()),
            exponent => PowOutput::Rest(FactorPrefixAtomExponentAnnotation {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorPrefixAtomAnnotation {
    type Output = PowOutput<&'a mut Annotation, (), FactorPrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match rhs {
            0 => PowOutput::Zero(&mut self.annotation),
            1 => PowOutput::One(()),
            exponent => PowOutput::Rest(FactorPrefixAtomExponentAnnotation {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: mem::take(&mut self.annotation),
            }),
        }
    }
}

// ╭──────────────────────────╮
// │ FactorPrefixAtomExponent │
// ╰──────────────────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FactorPrefixAtomExponent {
    pub(crate) factor: Factor,
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
}

impl FactorPrefixAtomExponent {
    #[must_use]
    pub const fn new(factor: Factor, prefix: Prefix, atom: Atom, exponent: Exponent) -> Self {
        Self {
            factor,
            prefix,
            atom,
            exponent,
        }
    }
}

impl_assign_exponent!(FactorPrefixAtomExponent);

impl UnassignExponent for FactorPrefixAtomExponent {
    fn unassign_exponent(self) -> Term {
        Term::FactorPrefixAtom(FactorPrefixAtom {
            factor: self.factor,
            prefix: self.prefix,
            atom: self.atom,
        })
    }
}

impl From<FactorPrefixAtomExponent> for Term {
    fn from(value: FactorPrefixAtomExponent) -> Self {
        Self::FactorPrefixAtomExponent(value)
    }
}

impl fmt::Display for FactorPrefixAtomExponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.factor, self.exponent) {
            (1, 1) => {
                write!(f, "{prefix}{atom}", prefix = self.prefix, atom = self.atom)
            }
            (1, exponent) => {
                write!(
                    f,
                    "{prefix}{atom}{exponent}",
                    prefix = self.prefix,
                    atom = self.atom,
                )
            }
            (factor, 1) => {
                write!(
                    f,
                    "{factor}{prefix}{atom}",
                    prefix = self.prefix,
                    atom = self.atom,
                )
            }
            (factor, exponent) => {
                write!(
                    f,
                    "{factor}{prefix}{atom}{exponent}",
                    prefix = self.prefix,
                    atom = self.atom,
                )
            }
        }
    }
}

impl Inv for FactorPrefixAtomExponent {
    type Output = InvOutput<FactorPrefixAtom, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(fpa) => InvOutput::One(fpa),
            PowOutput::Rest(fpae) => InvOutput::Rest(fpae),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut FactorPrefixAtomExponent {
    type Output = InvOutput<FactorPrefixAtom, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(fpa) => InvOutput::One(fpa),
            PowOutput::Rest(fpae) => InvOutput::Rest(fpae),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl Pow<Exponent> for FactorPrefixAtomExponent {
    type Output = PowOutput<Factor, FactorPrefixAtom, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(FactorPrefixAtom {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
            }),
            exponent => PowOutput::Rest(Self {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                exponent,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorPrefixAtomExponent {
    type Output = PowOutput<Factor, FactorPrefixAtom, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(FactorPrefixAtom {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
            }),
            exponent => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

// ╭────────────────────────────────────╮
// │ FactorPrefixAtomExponentAnnotation │
// ╰────────────────────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorPrefixAtomExponentAnnotation {
    pub(crate) factor: Factor,
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
    pub(crate) annotation: Annotation,
}

impl FactorPrefixAtomExponentAnnotation {
    #[must_use]
    pub const fn new(
        factor: Factor,
        prefix: Prefix,
        atom: Atom,
        exponent: Exponent,
        annotation: Annotation,
    ) -> Self {
        Self {
            factor,
            prefix,
            atom,
            exponent,
            annotation,
        }
    }
}

impl_assign_exponent!(FactorPrefixAtomExponentAnnotation);

impl UnassignExponent for FactorPrefixAtomExponentAnnotation {
    fn unassign_exponent(self) -> Term {
        Term::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation {
            factor: self.factor,
            prefix: self.prefix,
            atom: self.atom,
            annotation: self.annotation,
        })
    }
}

impl From<FactorPrefixAtomExponentAnnotation> for Term {
    fn from(value: FactorPrefixAtomExponentAnnotation) -> Self {
        Self::FactorPrefixAtomExponentAnnotation(value)
    }
}

impl fmt::Display for FactorPrefixAtomExponentAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.factor, self.exponent) {
            (1, 1) => {
                write!(
                    f,
                    "{prefix}{atom}{annotation}",
                    prefix = self.prefix,
                    atom = self.atom,
                    annotation = self.annotation
                )
            }
            (1, exponent) => {
                write!(
                    f,
                    "{prefix}{atom}{exponent}{annotation}",
                    prefix = self.prefix,
                    atom = self.atom,
                    annotation = self.annotation
                )
            }
            (factor, 1) => {
                write!(
                    f,
                    "{factor}{prefix}{atom}{annotation}",
                    prefix = self.prefix,
                    atom = self.atom,
                    annotation = self.annotation
                )
            }
            (factor, exponent) => {
                write!(
                    f,
                    "{factor}{prefix}{atom}{exponent}{annotation}",
                    prefix = self.prefix,
                    atom = self.atom,
                    annotation = self.annotation
                )
            }
        }
    }
}

impl Inv for FactorPrefixAtomExponentAnnotation {
    type Output = InvOutput<FactorPrefixAtomAnnotation, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(fpaa) => InvOutput::One(fpaa),
            PowOutput::Rest(fpaea) => InvOutput::Rest(fpaea),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a FactorPrefixAtomExponentAnnotation {
    type Output = InvOutput<FactorPrefixAtomAnnotation, FactorPrefixAtomExponentAnnotation>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(fpaa) => InvOutput::One(fpaa),
            PowOutput::Rest(fpaea) => InvOutput::Rest(fpaea),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut FactorPrefixAtomExponentAnnotation {
    type Output = InvOutput<FactorPrefixAtomAnnotation, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(fpaa) => InvOutput::One(fpaa),
            PowOutput::Rest(fpaea) => InvOutput::Rest(fpaea),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl Pow<Exponent> for FactorPrefixAtomExponentAnnotation {
    type Output = PowOutput<Annotation, FactorPrefixAtomAnnotation, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(self.annotation),
            1 => PowOutput::One(FactorPrefixAtomAnnotation {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                annotation: self.annotation,
            }),
            exponent => PowOutput::Rest(Self {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: self.annotation,
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a FactorPrefixAtomExponentAnnotation {
    type Output =
        PowOutput<Annotation, FactorPrefixAtomAnnotation, FactorPrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(self.annotation.clone()),
            1 => PowOutput::One(FactorPrefixAtomAnnotation {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                annotation: self.annotation.clone(),
            }),
            exponent => PowOutput::Rest(FactorPrefixAtomExponentAnnotation {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorPrefixAtomExponentAnnotation {
    type Output = PowOutput<&'a mut Annotation, FactorPrefixAtomAnnotation, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self.exponent * rhs {
            0 => PowOutput::Zero(&mut self.annotation),
            1 => PowOutput::One(FactorPrefixAtomAnnotation {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                annotation: mem::take(&mut self.annotation),
            }),
            exponent => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}
