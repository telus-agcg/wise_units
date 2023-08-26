//! This module contains (re)definitions of standard `wise_units` traits, but arguably as they
//! should've been defined. Defining them here gives an opportunity to use them in other crates that
//! build on `wise_units`, and if that goes well, then these traits can then replace the old ones.

pub mod atom;
pub mod definition;
pub mod dimension;
pub mod measurement;
pub mod prefix;
pub mod property;
pub mod term_old;
pub mod term_unit;
pub mod unit;
pub mod unit_old;

pub use self::{atom::Atom, prefix::Prefix, term_old::Term, unit::Unit};
