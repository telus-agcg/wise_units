use crate::v2::behavior_traits::{convert, ops, unit_conversion};

use super::Unit;

pub trait Measurement<'a, V>:
    Sized
    + convert::Invert
    + convert::ToInverse
    + convert::ToMagnitude<'a, V>
    + convert::ToScalar<V>
    + ops::DimEq
    + ops::TryAddRef<'a>
    + ops::TryDivRef<'a>
    + ops::TryMulRef<'a>
    + ops::TrySubRef<'a>
    + unit_conversion::TryConvertTo<'a, Self::Unit>
where
    V: PartialOrd,
{
    type Unit: Unit<'a, V>;

    fn value(&'a self) -> &'a V;
    fn unit(&'a self) -> &'a Self::Unit;
}
