use crate::v2::behavior_traits::{convert, ops};

use super::Unit;

pub trait Measurement<'a, V>:
    Sized
    + convert::Invert
    + convert::ToInverse
    + convert::TryToMagnitude<V>
    + convert::TryToScalar<V>
    + convert::TryConvertTo<'a, Self::Unit>
    + ops::DimEq
    + ops::TryAddRef<'a>
    + ops::TryDivRef
    + ops::TryMulRef
    + ops::TrySubRef<'a>
where
    V: PartialOrd,
{
    type Unit: Unit<'a, V>;

    fn value(&self) -> &V;
    fn unit(&self) -> &Self::Unit;
}
