use crate::{
    v2::{
        behavior_traits::{convert, ucum},
        type_traits,
    },
    Composable, UcumSymbol, UcumUnit,
};

use super::Atom;


impl ucum::Dimensionable for Atom {
    type Output = crate::Composition;

    fn dim(&self) -> Self::Output {
        // Just delegate to the original impl for now.
        crate::Composable::composition(self)
    }
}

impl convert::ToScalar<f64> for Atom {
    fn to_scalar(&self) -> f64 {
        <Self as UcumUnit>::scalar(self)
    }
}

impl convert::ToMagnitude<f64> for Atom {
    fn to_magnitude(&self) -> f64 {
        <Self as UcumUnit>::magnitude(self)
    }
}
