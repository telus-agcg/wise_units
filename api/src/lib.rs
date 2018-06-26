// Turn on proc_macro if we're on nightly AND using the with_stdweb feature.
#![cfg_attr(
    all(any(target_arch = "wasm32", target_os = "emscripten"), feature = "with_stdweb"),
    feature(proc_macro)
)]

#[cfg(test)]
#[macro_use]
extern crate approx;
extern crate failure;

#[macro_use]
extern crate failure_derive;

// Only include macros for testing
#[cfg(test)]
#[macro_use(consumes_to, fails_with, parses_to)]
extern crate pest;

#[cfg(not(test))]
extern crate pest;

#[macro_use]
extern crate pest_derive;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(feature = "with_serde")]
extern crate serde;

#[cfg(feature = "with_serde")]
#[cfg_attr(feature = "with_serde", macro_use)]
extern crate serde_derive;

#[cfg(all(test, feature = "with_serde"))]
extern crate serde_json;

#[cfg(all(any(target_arch = "wasm32", target_os = "emscripten"), feature = "with_stdweb"))]
#[macro_use]
extern crate stdweb;

#[macro_use]
mod macros;

pub mod convertible;
pub mod field_eq;
pub mod measurement;
pub mod unit;

mod decomposer;
mod parser;
mod reducible;
mod ucum_unit;

pub use convertible::Convertible;
pub use field_eq::FieldEq;
pub use measurement::Measurement;
pub use parser::{
    Atom, Classification, Composable, Composition, Dimension, Error, Prefix, Property, Term,
};
pub use ucum_unit::UcumUnit;
pub use unit::Unit;
