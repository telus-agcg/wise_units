#![deny(unused_extern_crates)]
#![warn(
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
    clippy::perf,
    clippy::nursery,
    clippy::style
)]

#[macro_use]
mod macros;

pub mod prefixes;
pub mod runtime_unit;
pub mod unit;
pub mod units;

mod dimension;
mod error;
mod number;
mod prefix;

pub use crate::{
    error::Error,
    number::Number,
    prefix::Prefix,
    runtime_unit::{RuntimeUnit, RuntimeUnitBuilder},
    unit::UnitDefinition,
};
