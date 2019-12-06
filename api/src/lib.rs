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
pub mod composable;
pub mod composition;
pub mod convertible;
pub mod error;
pub mod field_eq;
pub mod invert;
pub mod is_compatible_with;
pub mod measurement;
pub mod reduce;
pub mod unit;

mod reducible;
mod ucum_unit;

pub use crate::{
    composable::Composable, composition::Composition, convertible::Convertible, error::Error,
    field_eq::FieldEq, is_compatible_with::IsCompatibleWith, measurement::Measurement,
    ucum_unit::UcumUnit, unit::Unit,
};

pub use wise_units_parser::Dimension;
