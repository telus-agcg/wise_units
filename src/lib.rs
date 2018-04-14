// Turn on proc_macro if we're on nightly AND using the with_stdweb feature.
#![cfg_attr(all(any(target_arch = "wasm32", target_os = "emscripten"), feature = "with_stdweb"),
            feature(proc_macro))]

extern crate failure;
#[macro_use]
extern crate failure_derive;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

// Only include macros for testing
#[cfg(test)]
#[macro_use]
extern crate pest;

#[cfg(not(test))]
extern crate pest;

#[macro_use]
extern crate pest_derive;

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

pub mod measurement;
pub mod ucum_symbol;
pub mod unit;

mod atom;
mod composition;
mod classification;
mod decomposable;
mod definition;
mod dimension;
mod error;
mod interpreter;
mod measurable;
mod prefix;
mod property;
mod reduction_decomposer;
mod simple_decomposer;
mod term;
mod unit_parser;

pub use error::Error;
pub use measurable::Measurable;
pub use measurement::Measurement;
pub use unit::Unit;
