//!
//! Custom FFI implementation for `Unit`, since we have no interest in exposing its `terms` (or the
//! fields of `Term`, etc).
//!

use crate::Unit;
use ffi_common::ffi_core;
use std::{os::raw::c_char, str::FromStr};

/// Initialize a `wise_units::Unit` with `expression`.
///
/// Sets an error for the caller to retrieve if `expression` is not a valid UCUM unit.
///
#[allow(box_pointers)]
#[no_mangle]
pub extern "C" fn unit_init(expression: *const c_char) -> *const Unit {
    let string = ffi_core::string::string_from_c(expression);
    ffi_core::try_or_set_error!(Unit::from_str(&string).map(|u| Box::into_raw(Box::new(u))))
}

/// Clones the `Unit` behind `ptr` and returns it behind a new raw pointer.
///
/// # Safety
///
/// `ptr` is dereferenced, so make sure it's not null!
///
#[allow(box_pointers)]
#[no_mangle]
pub unsafe extern "C" fn clone_unit(ptr: *const Unit) -> *const Unit {
    Box::into_raw(Box::new((&*ptr).clone()))
}

/// Returns the `expression` for the `Unit` behind `ptr` as a C string.
///
/// # Safety
///
/// `ptr` is dereferenced, so make sure it's not null!
///
#[no_mangle]
pub unsafe extern "C" fn get_unit_expression(ptr: *const Unit) -> *const c_char {
    ffi_core::ffi_string!((&*ptr).expression())
}
