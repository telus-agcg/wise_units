// use crate::v2::behavior_traits::{convert, ucum};

use super::WrapperUnit;

// + convert::InvertMut
// + convert::ToInverse
// + convert::TryConvertTo<'a, Self::Unit>
// + ops::CommensurableAdd<'a, V>
// + ops::TrySubRef<'a>
// + ops::TryMulRef<'a>
// + ops::TryDivRef<'a>
// where
//     V: PartialOrd + PartialEq,

// Sized + convert::TryToMagnitude<V> + convert::TryToScalar<V> + ucum::Dimensionable
// Sized + convert::TryToMagnitude<V> + convert::TryToScalar<V> + ucum::Dimensionable
pub trait WrapperMeasurement {
    type Unit: WrapperUnit;
    type Number;

    fn value(&self) -> Self::Number;
    fn unit(&self) -> Self::Unit;
}
