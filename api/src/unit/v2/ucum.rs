// use crate::{v2::behavior_traits::ucum, Unit};

// impl<'a> ucum::ParseUcumStr<'a, &'a str> for Unit {
//     type Error = crate::Error;

//     fn parse_ucum_str(ucum_str: &'a str) -> Result<Self, Self::Error> {
//         std::str::FromStr::from_str(ucum_str)
//     }
// }

// impl ucum::Dim<crate::Composition> for Unit {
//     fn dim(&self) -> crate::Composition {
//         // Just delegate to the old trait impl for now.
//         crate::Composable::composition(self)
//     }
// }

// impl ucum::DefinitionFlags for Unit {
//     fn is_special(&self) -> bool {
//         // Just delegate to the old trait impl for now.
//         crate::UcumUnit::is_special(self)
//     }

//     fn is_metric(&self) -> bool {
//         // Just delegate to the old trait impl for now.
//         crate::UcumUnit::is_metric(self)
//     }

//     fn is_arbitrary(&self) -> bool {
//         // Just delegate to the old trait impl for now.
//         crate::UcumUnit::is_arbitrary(self)
//     }
// }
