#![deny(unused_extern_crates)]
#![warn(
    box_pointers,
    clippy::all,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    missing_copy_implementations,
    // missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#![allow(clippy::redundant_pub_crate)]

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
#[cfg(feature = "v2")]
pub mod v2;
#[cfg(feature = "wrappable")]
pub mod wrappable;

mod reducible;
mod ucum_unit;

pub use crate::{
    convertible::Convertible,
    error::Error,
    field_eq::FieldEq,
    is_compatible_with::IsCompatibleWith,
    measurement::Measurement,
    parser::{
        composition, Atom, Classification, Composable, Composition, Dimension, Prefix, Property,
        Term, UcumSymbol,
    },
    ucum_unit::UcumUnit,
    unit::Unit,
};
