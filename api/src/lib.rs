#![forbid(unused_imports)]
#![deny(unused_extern_crates)]
#![warn(
    box_pointers,
    future_incompatible,
    missing_copy_implementations,
    // missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_qualifications,
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::nursery,
    clippy::style
)]

#[macro_use]
extern crate failure_derive;

#[macro_use]
mod macros;

pub mod as_fraction;
pub mod convertible;
pub mod error;
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
pub use crate::error::Error;
pub use crate::field_eq::FieldEq;
pub use crate::is_compatible_with::IsCompatibleWith;
pub use crate::measurement::Measurement;
pub use crate::parser::{
    Atom, Classification, Composable, Composition, Dimension, Prefix, Property, Term, UcumSymbol,
};
pub use crate::ucum_unit::UcumUnit;
pub use crate::unit::Unit;
