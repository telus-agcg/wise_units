use crate::v2::behavior_traits::{convert, ops, unit_conversion};

use super::Unit;

pub trait Measurement<'a, V>:
    Sized
    + convert::Invert
    + convert::ToInverse
    + convert::ToMagnitude<V>
    + convert::ToScalar<V>
    + ops::DimEq
    + ops::TryAddRef<'a>
    + ops::TryDivRef
    + ops::TryMulRef
    + ops::TrySubRef<'a>
    + unit_conversion::TryConvertTo<'a, Self::Unit>
where
    V: PartialOrd,
{
    type Unit: Unit<'a, V>;

    fn value(&self) -> &V;
    fn unit(&self) -> &Self::Unit;
}
