use std::convert::Infallible;

use crate::{
    v2::behavior_traits::{convert::ToScalar, ops},
    Unit,
};

impl<'a> ops::Comparable<'a, f64> for Unit {
    /// Overriding default to use `ulps_eq` for comparing `f64`
    ///
    fn is_commensurable_with(&'a self, rhs: &'a Self) -> bool {
        if !ops::Comparable::is_compatible_with(self, rhs) {
            return false;
        }

        approx::ulps_eq!(self.to_scalar(), rhs.to_scalar())
    }
}

impl ops::MulRef for Unit {
    fn mul_ref(&self, rhs: &Self) -> Self {
        std::ops::Mul::mul(self, rhs)
    }
}

impl ops::TryMulRef<'_> for Unit {
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

impl ops::TryDivRef<'_> for Unit {
    type Error = Infallible;

    fn try_div_ref(&self, rhs: &Self) -> Result<Self, Self::Error> {
        Ok(ops::DivRef::div_ref(self, rhs))
    }
}
