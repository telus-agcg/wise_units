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

pub use measurable::Measurable;
pub use measurement::Measurement;
pub use unit::Unit;
pub use error::Error;
