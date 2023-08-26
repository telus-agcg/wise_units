use crate::IsCompatibleWith;

use crate::v2::behavior_traits::{
    convert::{Invert, ToInverse},
    ops::{FieldEq, IsCommensurableWith},
    ucum::Dim,
};

pub trait Term:
    Sized + Dim<crate::Composition> + Invert + IsCommensurableWith<Self> + IsCompatibleWith + ToInverse
{
    type Prefix: PartialEq;
    type Atom: PartialEq;
    type Annotation: PartialEq;

    fn factor(&self) -> Option<u32>;
    fn prefix_symbol(&self) -> Option<Self::Prefix>;
    fn atom_symbol(&self) -> Option<Self::Atom>;
    fn exponent(&self) -> Option<i32>;
    fn exponent_mut(&mut self) -> &mut Option<i32>;
    fn annotation(&self) -> &Option<Self::Annotation>;

    fn is_unity(&self) -> bool {
        self.factor() == Some(1_u32)
            && self.exponent().is_none()
            && self.atom_symbol().is_none()
            && self.prefix_symbol().is_none()
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

    //     fn to_scalar<T>(&self) -> T
    //     where
    //         Self: ToScalar<T>,
    //     {
    //         ToScalar::to_scalar(self)
    //     }

    //     fn to_magnitude<T>(&self) -> T
    //     where
    //         Self: ToMagnitude<T>,
    //     {
    //         ToMagnitude::to_magnitude(self)
    //     }
}
