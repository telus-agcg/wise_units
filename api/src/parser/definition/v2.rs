// use crate::v2::{behavior_traits::convert, type_traits};
//
// use super::Definition;

// impl type_traits::StaticDefinition<f64> for Definition {
//     type Unit = crate::Unit;
//
//     fn value(&self) -> &f64 {
//         &self.value
//     }
//
//     fn unit(&self) -> Option<&Self::Unit> {
//         Some(self.terms())
//     }
//
//     fn function_set(&self) -> Option<&crate::parser::function_set::FunctionSet> {
//         self.function_set.as_ref()
//     }
// }
//
// impl convert::ToScalar<f64> for Definition {
//     fn to_scalar(&self) -> f64 {
//         self.terms.to_scalar()
//     }
// }
//
// impl convert::ToMagnitude<f64> for Definition {
//     fn to_magnitude(&self) -> f64 {
//         self.terms.to_magnitude()
//     }
// }
