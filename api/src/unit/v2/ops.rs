use crate::{v2::traits::ops, Unit};

impl ops::FieldEq<Self> for Unit {
    fn field_eq(&self, rhs: &Self) -> bool {
        // Just delegate to the old trait impl for now.
        crate::FieldEq::field_eq(self, rhs)
    }
}

impl ops::IsCommensurableWith<Self> for Unit {
    fn is_commensurable_with(&self, rhs: &Self) -> bool {
        // TODO: Delegating, but the current implementation is incorrect (it should behave how the
        // current FieldEq implementation behaves.).
        PartialEq::eq(self, rhs)
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
