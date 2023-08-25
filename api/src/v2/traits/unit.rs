use std::{borrow::Cow, cmp::Ordering, ops::Mul};

use num_traits::One;

use crate::IsCompatibleWith;

use super::{
    convert::{self, Invert, ToInverse, ToMagnitude, ToScalar},
    ops::FieldEq,
    ucum::Dim,
    Term,
};

// pub trait Unit<'a>:
//     Sized
//     + DefinitionFlags
//     + DivRef
//     + Invert
//     + IsCompatibleWith<&'a Self>
//     + MulRef
//     + ParseUcumStr<'a>
//     + PartialEq
//     + PartialOrd
//     + ToFraction<Option<Self>, Option<Self>>
//     + ToInverse
//     + ToMagnitude<f64>
//     + ToReduced
//     + ToScalar<f64>
//     + 'a
// {
// }

pub trait Unit:
    Sized
    + FromIterator<Self::Term>
    // + Invert
    // + IsCommensurableWith<Self>
    + IsCompatibleWith
    + PartialEq
    // + ToInverse
{
    type Term: Term + Clone;
    type UcumString: ?Sized;
    type ParseError;

    /// Defines how to parse a string of unit symbols into a `Unit`.
    ///
    /// # Errors
    ///
    /// This should error if the `ucum_str` can't be parsed into a type that represents that
    /// combinations of units.
    ///
    fn parse_ucum_str(ucum_str: &Self::UcumString) -> Result<Self, Self::ParseError>;
    fn expression(&self) -> Cow<'_, str>;
    fn terms(&self) -> &[Self::Term];
    fn terms_mut(&mut self) -> &mut [Self::Term];

    fn is_unity(&self) -> bool {
        self.terms().len() == 1 && self.terms()[0].is_unity()
    }

    fn invert(&mut self) {
        for term in self.terms_mut() {
            term.invert();
        }
    }

    fn to_inverse<U>(&self) -> U
    where
        U: FromIterator<Self::Term>,
    {
        self.terms().iter().map(ToInverse::to_inverse).collect()
    }

    fn to_scalar<T>(&self) -> T
    where
        T: One,
        Self::Term: ToScalar<T>
    {
        self.terms()
            .iter()
            .fold(One::one(), |acc, term| acc * super::Term::to_scalar(term))
    }

    fn to_magnitude<T>(&self) -> T
    where
        T: One,
        Self::Term: ToMagnitude<T>
    {
        self.terms()
            .iter()
            .fold(One::one(), |acc, term| acc * super::Term::to_magnitude(term))
    }

    fn is_commensurable_with<T, U, D>(&self, rhs: &T) -> bool
    where
        T: ToScalar<U> + Dim<D>,
        U: One + PartialEq,
        D: Default + Copy + PartialEq + Mul<i32, Output = D> + Mul<D, Output = D>,
        Self: ToScalar<U> + IsCompatibleWith<T>,
        <Self as Unit>::Term: ToScalar<U> + Dim<D>
    {
        if !<Self as Unit>::is_compatible_with::<T, D>(self, rhs) {
            return false;
        }

        // ulps_eq!(<Self as Unit>::to_scalar(self), rhs.to_scalar())
        <Self as Unit>::to_scalar(self) == rhs.to_scalar()
    }

    fn is_compatible_with<T, D>(&self, rhs: &T) -> bool
    where
        T: Dim<D>,
        D: Default + Copy + PartialEq + Mul<i32, Output = D> + Mul<D, Output = D>,
        Self::Term: Dim<D>
    {
        // TODO:
        // let lhs_annotation_composition = self.annotation_composition();
        // let rhs_annotation_composition = rhs.annotation_composition();

        <Self as Unit>::dim::<D>(self) == rhs.dim()
            // && rhs_annotation_composition == lhs_annotation_composition
    }

    /// Here, "1 kilometer != 1000 meters"; "1 kilometer" only equals "1 kilometer".
    ///
    fn equals<T>(&self, rhs: &T) -> bool
    where
        Self: PartialEq<T> + FieldEq<T>,
    {
        // TODO: Eventually, this should be this:
        // PartialEq::<T>::eq(self, &rhs)
        FieldEq::field_eq(self, rhs)
    }

    fn dim<D>(&self) -> D
    where
        Self::Term: Dim<D>,
        D: Default + Copy + PartialEq + Mul<i32, Output = D> + Mul<D, Output = D>,
    {
        self.terms()
            .iter()
            .fold(D::default(), |acc, term| acc * term.dim())
    }

    /// `PartialOrd` depends on being in concert with `PartialEq`, which depends on being in concert
    /// it `Hash`, but only units that are strictly equal can fulfill this; units that are
    /// commensurable but not equal will not. This allows for the later case.
    ///
    fn commensurable_ord<T, U, D>(&self, rhs: &T) -> Option<Ordering>
    where
        T: ToScalar<U> + Dim<D>,
        U: One + PartialEq + PartialOrd,
        D: Default + Copy + PartialEq + Mul<i32, Output = D> + Mul<D, Output = D>,
        Self: ToScalar<U> + IsCompatibleWith<T>,
        <Self as Unit>::Term: ToScalar<U> + Dim<D>
    {
        if !<Self as Unit>::is_compatible_with::<T, D>(self, rhs) {
            return None;
        }

        <Self as Unit>::to_scalar(self).partial_cmp(&rhs.to_scalar())
    }
}

impl<T> convert::ToFraction for T
where
    T: Unit,
{
    fn to_fraction(&self) -> (Option<Self>, Option<Self>) {
        (self.numerator(), self.denominator())
    }

    fn numerator(&self) -> Option<Self> {
        let mut positive_terms_iter = self
            .terms()
            .iter()
            .filter(|term| term.exponent().unwrap_or(1).is_positive())
            .peekable();

        if positive_terms_iter.peek().is_some() {
            Some(positive_terms_iter.cloned().collect())
        } else {
            None
        }
    }

    fn denominator(&self) -> Option<Self> {
        let mut negative_terms_iter = self
            .terms()
            .iter()
            .filter_map(|term| match term.exponent() {
                Some(e) if e.is_negative() => Some(term.to_inverse()),
                _ => None,
            })
            .peekable();

        if negative_terms_iter.peek().is_some() {
            Some(negative_terms_iter.collect())
        } else {
            None
        }
    }
}
