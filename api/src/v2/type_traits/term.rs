use crate::v2::behavior_traits::{convert, ops};

use super::{Atom, Prefix};

pub trait Term<'a, V>:
    Sized
    + convert::Invert
    + convert::ToInverse
    + convert::ToScalar<'a, V>
    + convert::ToMagnitude<'a, V>
    + ops::Comparable<'a, V>
where
    V: PartialOrd,
{
    type Prefix: Prefix<'a, V>;
    type Atom: Atom<'a, V>;
    type Annotation;

    fn factor(&'a self) -> Option<u32>;
    fn prefix_symbol(&'a self) -> Option<Self::Prefix>;
    fn atom_symbol(&'a self) -> Option<Self::Atom>;
    fn exponent(&'a self) -> Option<i32>;
    fn annotation(&'a self) -> Option<Self::Annotation>;
}
