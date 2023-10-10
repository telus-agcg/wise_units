use crate::v2::behavior_traits::{convert, ops};

use super::{Atom, Prefix};

pub trait Term<'a, V>:
    Sized
    + convert::Invert
    + convert::ToInverse
    + convert::ToScalar<V>
    + convert::ToMagnitude<V>
    + ops::DimEq
where
    V: PartialOrd,
{
    type Prefix: Prefix<V>;
    type Atom: Atom<V>;
    type Annotation;

    fn factor(&self) -> Option<u32>;
    fn prefix_symbol(&self) -> Option<Self::Prefix>;
    fn atom_symbol(&self) -> Option<Self::Atom>;
    fn exponent(&self) -> Option<i32>;
    fn annotation(&'a self) -> Option<Self::Annotation>;
}
