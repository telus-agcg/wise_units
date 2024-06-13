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
#![allow(clippy::redundant_pub_crate, deprecated_in_future)]

#[macro_use]
mod macros;

pub(crate) mod annotation;
pub mod as_fraction;
pub mod atom;
pub mod classification;
pub mod composition;
pub mod convertible;
pub mod error;
pub mod field_eq;
pub mod invert;
pub mod is_compatible_with;
pub mod measurement;
pub mod property;
pub mod reduce;
pub mod term;
pub mod unit;
#[cfg(feature = "v2")]
pub mod v2;

mod composable;
mod dimension;
mod prefix;
mod reducible;
#[cfg(test)]
mod testing;
mod ucum_symbol;
mod ucum_unit;

pub use crate::{
    annotation::Annotation, atom::Atom, classification::Classification, composable::Composable,
    composition::Composition, convertible::Convertible, dimension::Dimension, error::Error,
    field_eq::FieldEq, is_compatible_with::IsCompatibleWith, measurement::Measurement,
    prefix::Prefix, property::Property, term::Term, ucum_symbol::UcumSymbol, ucum_unit::UcumUnit,
    unit::Unit,
};
