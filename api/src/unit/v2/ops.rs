use std::convert::Infallible;

use crate::{v2::behavior_traits::ops, Unit};

impl ops::MulRef for Unit {
    fn mul_ref(&self, rhs: &Self) -> Self {
        std::ops::Mul::mul(self, rhs)
    }
}

impl ops::TryMulRef for Unit {
    type Error = Infallible;

    fn try_mul_ref(&self, rhs: &Self) -> Result<Self, Self::Error> {
        Ok(ops::MulRef::mul_ref(self, rhs))
    }
}

impl ops::DivRef for Unit {
    fn div_ref(&self, rhs: &Self) -> Self {
        std::ops::Div::div(self, rhs)
    }
}

impl ops::TryDivRef for Unit {
    type Error = Infallible;

    fn try_div_ref(&self, rhs: &Self) -> Result<Self, Self::Error> {
        Ok(ops::DivRef::div_ref(self, rhs))
    }
}
