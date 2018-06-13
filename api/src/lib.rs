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

#[macro_use]
extern crate log;

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

#[cfg(test)]
extern crate simple_logger;

#[cfg(all(any(target_arch = "wasm32", target_os = "emscripten"), feature = "with_stdweb"))]
#[macro_use]
extern crate stdweb;

#[macro_use]
mod macros;

pub mod measurement;
pub mod unit;

mod decomposer;
mod measurable;
mod parser;

pub use measurable::Measurable;
pub use measurement::Measurement;
pub use parser::{Atom, Classification, Composable, Composition, Dimension, Error, Prefix,
                 Property, Term};
pub use unit::Unit;
