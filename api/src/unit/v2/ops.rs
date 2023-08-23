use crate::{v2::traits::ops, Term, Unit};

impl<'a> ops::FieldEq<&'a Self> for Unit {
    fn field_eq(&self, rhs: &'a Self) -> bool {
        // Just delegate to the old trait impl for now.
        crate::FieldEq::field_eq(self, rhs)
    }
}

impl<'a> ops::IsCompatibleWith<&'a Self> for Unit {
    fn is_compatible_with(&self, rhs: &'a Self) -> bool {
        crate::IsCompatibleWith::is_compatible_with(self, rhs)
    }
}

impl<'a> ops::IsCompatibleWith<&'a [Term]> for Unit {
    fn is_compatible_with(&self, rhs: &'a [Term]) -> bool {
        crate::IsCompatibleWith::is_compatible_with(self, rhs)
    }
}

impl ops::MulRef for Unit {
    fn mul_ref(&self, rhs: &Self) -> Self {
        std::ops::Mul::mul(self, rhs)
    }
}

impl ops::DivRef for Unit {
    fn div_ref(&self, rhs: &Self) -> Self {
        std::ops::Div::div(self, rhs)
    }
}
