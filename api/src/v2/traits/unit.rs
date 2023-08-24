use crate::v2::traits::{
    convert::{Invert, ToFraction, ToInverse, ToMagnitude, ToScalar},
    ops::{DivRef, IsCompatibleWith, MulRef},
    ucum::{DefinitionFlags, Dim, ParseUcumStr},
    unit_conversion::ToReduced,
};

pub trait Unit<'a>:
    Sized
    + ToFraction<Option<Self>, Option<Self>>
    + Dim
    + ParseUcumStr<'a>
    + Invert
    + IsCompatibleWith<&'a Self>
    + PartialEq
    + PartialOrd
    + DivRef
    + MulRef
    + ToInverse
    + ToReduced
    + ToScalar<f64>
    + ToMagnitude<f64>
    + DefinitionFlags
    + 'a
{
}
