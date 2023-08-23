use crate::v2::traits::{
    convert::{Invert, ToFraction, ToInverse, ToMagnitude, ToReduced, ToScalar},
    dim::Composable,
    ops::{IsCompatibleWith, RefDiv, RefMul},
    ucum::{FromUcumStr, UnitFlags},
};

pub trait Unit<'a>:
    Sized
    + ToFraction<Option<Self>, Option<Self>>
    + Composable
    + FromUcumStr
    + Invert
    + IsCompatibleWith<&'a Self>
    + PartialEq
    + PartialOrd
    + RefDiv
    + RefMul
    + ToInverse
    + ToReduced
    + ToScalar<f64>
    + ToMagnitude<f64>
    + UnitFlags
    + 'a
{
}

// impl<'a> Unit<'a> for crate::Unit {}

impl FromUcumStr for crate::Unit {
    type String = str;
    type Output = Result<Self, crate::Error>;

    fn from_ucum_str(ucum_str: &Self::String) -> Self::Output {
        std::str::FromStr::from_str(ucum_str)
    }
}

impl<'a> IsCompatibleWith<&'a Self> for crate::Unit {
    fn is_compatible_with(&self, rhs: &'a Self) -> bool {
        crate::IsCompatibleWith::is_compatible_with(self, rhs)
    }
}
