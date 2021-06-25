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

use ffi_common::try_or_set_error;
use std::{convert::TryInto, os::raw::c_char, ptr};
pub mod measurement;
pub mod unit;

pub use wise_units::{Measurement, Unit};

#[allow(clippy::needless_pass_by_value)]
fn set_error_and_return<T>(message: String) -> *const T {
    ffi_common::error::set_last_err_msg(&message);
    std::ptr::null()
}

#[allow(clippy::needless_pass_by_value)]
fn move_string_to_buffer(string: String, buffer: *mut c_char, length: usize) -> i32 {
    ffi_common::error::clear_last_err_msg();

    if buffer.is_null() {
        ffi_common::error::set_last_err_msg(
            "Null pointer passed into return_string_in_buffer() as the buffer",
        );
        return -1;
    }

    unsafe {
        let buffer_internal = std::slice::from_raw_parts_mut(buffer.cast::<u8>(), length);

        if string.len() >= buffer_internal.len() {
            ffi_common::error::set_last_err_msg(
                format!(
                    "Buffer provided for writing the message is too small.
            Expected at least {} bytes but got {}",
                    string.len() + 1,
                    buffer_internal.len()
                )
                .as_str(),
            );
            buffer_internal[0] = 0;

            return try_or_set_error!((string.len() + 1).try_into(), -1);
        }

        ptr::copy_nonoverlapping(string.as_ptr(), buffer_internal.as_mut_ptr(), string.len());

        // Add a trailing null so people using the string as a `char *` don't
        // accidentally read into garbage.
        buffer_internal[string.len()] = 0;
    }

    try_or_set_error!(string.len().try_into(), -1)
}
