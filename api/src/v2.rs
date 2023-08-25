//! This module contains (re)definitions of standard `wise_units` traits and types,
//! but arguably as they should've been defined. Defining them here gives an
//! opportunity to use them in other crates that build on `wise_units`, and if that
//! goes well, then these traits can then replace the old ones.

// pub mod dimension;
// pub mod measurement;
pub mod traits;

// pub use self::measurement::Measurement;
