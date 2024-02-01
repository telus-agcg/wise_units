//! This module contains (re)definitions of standard `wise_units` traits, but arguably as they
//! should've been defined. Defining them here gives an opportunity to use them in other crates that
//! build on `wise_units`, and if that goes well, then these traits can then replace the old ones.
#![allow(clippy::module_name_repetitions)]

pub mod definition;
pub mod dimension;
pub mod prefix;
pub mod unit;

pub use self::dimension::Dimension;
pub use self::prefix::StaticPrefix;
pub use self::unit::{BasicStaticUnit, StaticUnit};
