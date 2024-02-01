use crate::{
    parser::{function_set::FunctionSet, Definition},
    v2::{
        behavior_traits::{convert, dim},
        type_traits::{self, unit::Names},
    },
    Composable, UcumSymbol, UcumUnit,
};

use super::Atom;

// impl type_traits::StaticUnit<f64> for Atom {
//     type Unit = Definition;
//
//     fn primary_code(&self) -> &'static str {
//         // Just delegate to existing implementation.
//         UcumSymbol::primary_code(self)
//     }
//
//     fn secondary_code(&self) -> Option<&'static str> {
//         // Just delegate to existing implementation.
//         UcumSymbol::secondary_code(self)
//     }
//
//     fn print_symbol(&self) -> Option<&'static str> {
//         // Just delegate to existing implementation.
//         UcumSymbol::print_symbol(self)
//     }
//
//     fn names(&self) -> Names {
//         let v = UcumSymbol::names(self);
//         if v.len() == 1 {
//             Names::One(v[0])
//         } else {
//             Names::Two((v[0], v[1]))
//         }
//     }
//
//     fn is_special(&self) -> bool {
//         // Just delegate to existing implementation.
//         UcumUnit::is_special(self)
//     }
//
//     fn is_metric(&self) -> bool {
//         // Just delegate to existing implementation.
//         UcumUnit::is_metric(self)
//     }
//
//     fn is_arbitrary(&self) -> bool {
//         // Just delegate to existing implementation.
//         UcumUnit::is_arbitrary(self)
//     }
//
//     fn class(&self) -> crate::Classification {
//         // Just delegate to existing implementation.
//         UcumSymbol::classification(self)
//     }
//
//     fn property(&self) -> crate::Property {
//         // Just delegate to existing implementation.
//         Self::property(*self)
//     }
//
//     fn dimensions(&self) -> crate::Composition {
//         // Just delegate to existing implementation.
//         Composable::composition(self)
//     }
//
//     fn value(&self) -> &f64 {
//         // Just delegate to existing implementation.
//         &UcumSymbol::definition_value(self)
//     }
//
//     fn unit(&self) -> Option<&<Self as type_traits::StaticUnit<f64>>::Unit> {
//         todo!()
//     }
//
//     fn function_set(&self) -> Option<&FunctionSet> {
//         todo!()
//     }
// }

impl dim::Dimensionable for Atom {
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
