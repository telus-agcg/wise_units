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
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "with_serde")]
#[cfg(test)]
extern crate serde_json;

#[cfg(feature = "with_stdweb")]
#[macro_use]
extern crate stdweb;

pub mod measurement;
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
