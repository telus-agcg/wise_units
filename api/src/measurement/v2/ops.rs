use crate::{v2::traits::ops, Measurement, Unit};

impl<'a> ops::IsCompatibleWith<&'a Self> for Measurement {
    fn is_compatible_with(&self, rhs: &'a Self) -> bool {
        crate::IsCompatibleWith::is_compatible_with(self, rhs)
    }
}

impl<'a> ops::IsCompatibleWith<&'a Unit> for Measurement {
    fn is_compatible_with(&self, rhs: &'a Unit) -> bool {
        crate::IsCompatibleWith::is_compatible_with(self, rhs)
    }
}

impl<'a> ops::FieldEq<&'a Self> for Measurement {
    fn field_eq(&self, rhs: &'a Self) -> bool {
        // Just delegate to the old trait impl for now.
        crate::FieldEq::field_eq(self, rhs)
    }
}

impl<'a> ops::TryRefAdd<'a> for Measurement {
    type Error = crate::Error;

    fn try_ref_add(&'a self, rhs: &'a Self) -> Result<Self, Self::Error> {
        // Just delegate to the old trait impl for now.
        std::ops::Add::add(self, rhs)
    }
}

impl<'a> ops::TryRefSub<'a> for Measurement {
    type Error = crate::Error;

    fn try_ref_sub(&'a self, rhs: &'a Self) -> Result<Self, Self::Error> {
        // Just delegate to the old trait impl for now.
        std::ops::Sub::sub(self, rhs)
    }
}

impl ops::RefMul for Measurement {
    fn ref_mul(&self, rhs: &Self) -> Self {
        // Just delegate to the old trait impl for now.
        std::ops::Mul::mul(self, rhs)
    }
}

impl ops::RefDiv for Measurement {
    fn ref_div(&self, rhs: &Self) -> Self {
        // Just delegate to the old trait impl for now.
        std::ops::Div::div(self, rhs)
    }
}
