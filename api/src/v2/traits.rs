//! This module contains (re)definitions of standard `wise_units` traits, but arguably as they
//! should've been defined. Defining them here gives an opportunity to use them in other crates that
//! build on `wise_units`, and if that goes well, then these traits can then replace the old ones.

pub mod convert;
pub mod ops;
pub mod term;
pub mod ucum;
pub mod unit;
pub mod unit_conversion;

pub use self::{term::Term, unit::Unit};

// // NOTE: This is only intended to serve as a stepping stone to get from the old
// // `Composable` trait to the new/v2 one.
// #[macro_export]
// macro_rules! bridge_impl_v2_composable {
//     ($dest:ty, $composition:ty) => {
//         impl $crate::v2::traits::dim::Composable for $dest {
//             type Composition = $composition;

//             fn composition(&self) -> Self::Composition {
//                 Self::Composition::from($crate::Composable::composition(&*self.as_wrapped_ref()))
//             }
//         }
//     };
// }

// NOTE: This is only intended to serve as a stepping stone to get from the old
// `UcumUnit` trait to the new/v2 one.
#[macro_export]
macro_rules! bridge_impl_v2_ucum_unit_flags {
    ($dest:ty) => {
        impl $crate::v2::traits::ucum::UcumUnitFlags for $dest {
            fn is_special(&self) -> bool {
                $crate::UcumUnit::is_special(&*self.as_wrapped_ref())
            }

            fn is_metric(&self) -> bool {
                $crate::UcumUnit::is_metric(&*self.as_wrapped_ref())
            }

            fn is_arbitrary(&self) -> bool {
                $crate::UcumUnit::is_arbitrary(&*self.as_wrapped_ref())
            }
        }
    };
}
