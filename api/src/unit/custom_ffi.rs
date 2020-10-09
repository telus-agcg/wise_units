//!
//! Custom FFI implementation for `Unit`, since we have no interest in exposing its `terms` (or the
//! fields of `Term`, etc).
//!

use crate::Unit;
use std::{os::raw::c_char, str::FromStr};

#[allow(box_pointers)]
#[no_mangle]
pub extern "C" fn unit_init(expression: *const c_char) -> *const Unit {
    let string = ffi_common::string::string_from_c(expression);
    ffi_common::try_or_set_error!(Unit::from_str(&string).map(|u| Box::into_raw(Box::new(u))))
}

#[no_mangle]
pub extern "C" fn get_unit_expression(ptr: *const Unit) -> *const c_char {
    unsafe { ffi_common::ffi_string!((&*ptr).expression()) }
}
