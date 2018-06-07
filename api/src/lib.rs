// Turn on proc_macro if we're on nightly AND using the with_stdweb feature.
#![cfg_attr(
    all(any(target_arch = "wasm32", target_os = "emscripten"), feature = "with_stdweb"),
    feature(proc_macro)
)]

#[cfg(test)]
#[macro_use]
extern crate approx;

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

#[cfg_attr(test, macro_use)]
extern crate wise_units_parser;

#[cfg(all(any(target_arch = "wasm32", target_os = "emscripten"), feature = "with_stdweb"))]
#[macro_use]
extern crate stdweb;

pub mod measurement;
pub mod unit;

mod decomposable;
mod measurable;
mod reduction_decomposer;
mod simple_decomposer;

pub use measurable::Measurable;
pub use measurement::Measurement;
pub use unit::Unit;
pub use wise_units_parser::{Composable, Error};
