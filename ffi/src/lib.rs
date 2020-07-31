#![forbid(unused_imports)]
#![deny(unused_extern_crates)]
#![warn(
    future_incompatible,
    missing_copy_implementations,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications,
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::nursery,
    clippy::style
)]
// C bindings don't include the module name, so ffi functions will need it.
#![allow(clippy::module_name_repetitions)]

pub mod measurement;
pub mod unit;

pub use wise_units::{Measurement, Unit};

fn set_error_and_return<T>(message: String) -> *const T {
    ffi_common::error::set_last_err_msg(&message);
    std::ptr::null()
}
