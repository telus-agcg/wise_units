use crate::{
    v2::{
        behavior_traits::{convert, ucum},
        type_traits,
    },
    Composable, UcumSymbol, UcumUnit,
};

use super::Atom;

impl type_traits::Atom<f64> for Atom {
    type String = &'static str;
    type Names = Vec<&'static str>;
    type Property = crate::Property;
    type Class = crate::Classification;
    type Dimension = crate::Composition;
    type Definition = crate::parser::definition::Definition;

    fn primary_code(&self) -> Self::String {
        // Just delegate to existing implementation.
        UcumSymbol::primary_code(self)
    }

    fn secondary_code(&self) -> Option<Self::String> {
        // Just delegate to existing implementation.
        UcumSymbol::secondary_code(self)
    }

    fn print_symbol(&self) -> Option<Self::String> {
        // Just delegate to existing implementation.
        UcumSymbol::print_symbol(self)
    }

    fn names(&self) -> Self::Names {
        // Just delegate to existing implementation.
        UcumSymbol::names(self)
    }

    fn is_special(&self) -> bool {
        // Just delegate to existing implementation.
        UcumUnit::is_special(self)
    }

    fn is_metric(&self) -> bool {
        // Just delegate to existing implementation.
        UcumUnit::is_metric(self)
    }

    fn is_arbitrary(&self) -> bool {
        // Just delegate to existing implementation.
        UcumUnit::is_arbitrary(self)
    }

    fn class(&self) -> Self::Class {
        // Just delegate to existing implementation.
        UcumSymbol::classification(self)
    }

    fn property(&self) -> Self::Property {
        // Just delegate to existing implementation.
        Self::property(*self)
    }

    fn dim(&self) -> Self::Dimension {
        // Just delegate to existing implementation.
        Composable::composition(self)
    }

    fn definition(&self) -> Self::Definition {
        Self::definition(*self)
    }
}

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
