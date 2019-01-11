#![forbid(unused_imports)]
#![deny(unused_extern_crates)]
#![warn(
    box_pointers,
    future_incompatible,
    missing_copy_implementations,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts
)]

#[cfg(test)]
#[macro_use]
extern crate approx;

#[macro_use]
extern crate failure_derive;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

// Only include macros for testing
#[cfg(test)]
#[macro_use(consumes_to, fails_with, parses_to)]
extern crate pest;

#[cfg(not(test))]
extern crate pest;

#[macro_use]
extern crate pest_derive;

#[cfg(feature = "with_serde")]
#[cfg_attr(feature = "with_serde", macro_use)]
extern crate serde_derive;

#[cfg(all(test, feature = "with_serde"))]
extern crate serde_json;

#[macro_use]
mod macros;

pub mod as_fraction;
pub mod convertible;
pub mod field_eq;
pub mod invert;
pub mod is_compatible_with;
pub mod measurement;
pub mod parser;
pub mod reduce;
pub mod unit;

mod reducible;
mod ucum_unit;

pub use crate::convertible::Convertible;
pub use crate::field_eq::FieldEq;
pub use crate::is_compatible_with::IsCompatibleWith;
pub use crate::measurement::Measurement;
pub use crate::parser::{
    Atom, Classification, Composable, Composition, Dimension, Error, Prefix, Property, Term,
    UcumSymbol,
};
pub use crate::ucum_unit::UcumUnit;
pub use crate::unit::Unit;
