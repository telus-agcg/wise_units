use crate::move_string_to_buffer;
use ffi_common::core::error;
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    ptr,
    str::FromStr,
};
use wise_units::parser::Composable;
use wise_units::{is_compatible_with::IsCompatibleWith, reduce::ToReduced, UcumUnit, Unit};

/// Create a new `Unit`. Note that you must call `unit_destroy(data: unit)` with
/// this instance when you are done with it so that the the unit can be properly
/// destroyed and its memory freed.
///
/// # Safety
///
/// `expression` is checked that it's null, but not checked for its type.
///
/// # Errors
///
/// * If the `expression` can't be converted to a `&str`.
/// * If `Unit` can't be created using the `expression` string.
///
#[no_mangle]
pub unsafe extern "C" fn unit_new(expression: *const c_char) -> *const Unit {
    error::clear_last_err_msg();

    if expression.is_null() {
        return ptr::null();
    };

    match CStr::from_ptr(expression).to_str() {
        Ok(exp_str) => match Unit::from_str(exp_str) {
            Ok(unit) => Box::into_raw(Box::new(unit)),
            Err(why) => crate::set_error_and_return(why.to_string()),
        },
        Err(why) => crate::set_error_and_return(why.to_string()),
    }
}

/// Return ownership of `data` to Rust to deallocate safely.
///
/// # Safety
///
/// `data` is unchecked, so validate it before passing it in.
///
#[no_mangle]
pub unsafe extern "C" fn unit_destroy(data: *const Unit) {
    drop(Box::from_raw(data as *mut Unit));
}

/// Essentially checks if the two `Unit`'s scalar values are equal.
///
/// # Safety
///
/// `data` and `other` are unchecked, so validate them before passing them in.
///
#[no_mangle]
pub unsafe extern "C" fn unit_partial_eq(data: *const Unit, other: *const Unit) -> bool {
    let unit = &*data;
    let other = &*other;
    unit == other
}

/// Checks to see if the `Unit` is "special" (one that uses a function for converting instead of
/// a scalar value).
///
/// # Safety
///
/// `data` is unchecked, so validate it before passing it in.
///
#[no_mangle]
pub unsafe extern "C" fn unit_is_special(data: *const Unit) -> bool {
    let unit = &*data;
    unit.is_special()
}

/// Gets the `Unit`'s scalar value (expressed in terms of the `Unit`'s base unit).
///
/// # Safety
///
/// `data` is unchecked, so validate it before passing it in.
///
#[no_mangle]
pub unsafe extern "C" fn unit_scalar(data: *const Unit) -> f64 {
    let unit = &*data;
    unit.scalar()
}

/// Gets the `Unit`'s magnitude.
///
/// # Safety
///
/// `data` is unchecked, so validate it before passing it in.
///
#[no_mangle]
pub unsafe extern "C" fn unit_magnitude(data: *const Unit) -> f64 {
    let unit = &*data;
    unit.magnitude()
}

/// Checks if `data` and `other` are compatible/comparable `Units`.
///
/// # Safety
///
/// Both `data` and `other` are unchecked, so validate your pointers before passing them in.
///
#[no_mangle]
pub unsafe extern "C" fn unit_is_compatible_with(data: *const Unit, other: *const Unit) -> bool {
    let unit1 = &*data;
    let unit2 = &*other;
    unit1.is_compatible_with(unit2)
}

/// Wrapper function for checking if the `Unit` is valid.
///
/// # Safety
///
/// `expression` is checked that it's null, but not checked for its type. If it's null, this returns
/// `false`.
///
/// # Errors
///
/// * If the `expression` can't be converted to a `&str`.
/// * If `Unit` can't be created using the `expression` string.
///
#[no_mangle]
pub unsafe extern "C" fn unit_is_valid(expression: *const c_char) -> bool {
    error::clear_last_err_msg();

    if expression.is_null() {
        return false;
    };

    match CStr::from_ptr(expression).to_str() {
        Ok(exp_str) => Unit::from_str(exp_str).is_ok(),
        Err(why) => {
            error::set_last_err_msg(&why.to_string());
            false
        }
    }
}

/// After copying the string data referenced by the returned pointer, you must
/// call `destroy_string` to free the pointer from Rust.
///
/// # Safety
///
/// `data` is unchecked, so validate it before passing it in.
///
#[no_mangle]
pub unsafe extern "C" fn unit_expression(data: *const Unit) -> *const c_char {
    error::clear_last_err_msg();

    let unit = &*data;
    match CString::new(unit.expression()) {
        Ok(expression) => expression.into_raw(),
        Err(why) => crate::set_error_and_return(why.to_string()),
    }
}

/// Wrapper function for extracting `Unit`s expression.
/// If buffer is not large enough the first char will be set to 0 and the required length as the return value
/// # Safety
///
/// `data` is unchecked, validate your pointers before passing them in.
///
#[no_mangle]
pub unsafe extern "C" fn unit_expression_buf(
    data: *const Unit,
    buffer: *mut c_char,
    length: usize,
) -> i32 {
    error::clear_last_err_msg();

    let unit = &*data;

    move_string_to_buffer(unit.expression(), buffer, length)
}

/// Wrapper function for reducing a `Unit`.
///
/// # Safety
///
/// `data` is unchecked, so validate your pointer before passing it in.
///
#[no_mangle]
pub unsafe extern "C" fn unit_reduced(data: *const Unit) -> *const Unit {
    let original = &*data;
    let reduced = original.to_reduced();
    Box::into_raw(Box::new(reduced))
}

/// Wrapper function for dividing `Unit`s.
///
/// # Safety
///
/// Both `data` and `other` are unchecked, so validate your pointers before passing them in.
///
#[no_mangle]
pub unsafe extern "C" fn unit_div(data: *const Unit, other: *const Unit) -> *const Unit {
    let unit = &*data;
    let other = &*other;
    let quotient = unit / other;
    let quotient_box = Box::new(quotient);
    Box::into_raw(quotient_box)
}

/// Wrapper function for multiplying `Unit`s.
///
/// # Safety
///
/// Both `data` and `other` are unchecked, so validate your pointers before passing them in.
///
#[no_mangle]
pub unsafe extern "C" fn unit_mul(data: *const Unit, other: *const Unit) -> *const Unit {
    let unit = &*data;
    let other = &*other;
    let product = unit * other;
    let product_box = Box::new(product);
    Box::into_raw(product_box)
}

/// Wrapper function for extracting `Unit`s composition.
/// After copying the string data referenced by the returned pointer, you must
/// call `destroy_string` to free the pointer from Rust.
///
///
/// # Safety
///
/// `data` is unchecked, validate your pointers before passing them in.
///
#[no_mangle]
pub unsafe extern "C" fn unit_composition(data: *const Unit) -> *const c_char {
    error::clear_last_err_msg();
    let unit = &*data;
    match CString::new(unit.composition().to_string()) {
        Ok(composition) => composition.into_raw(),
        Err(why) => crate::set_error_and_return(why.to_string()),
    }
}

/// Wrapper function for extracting `Unit`s composition.
/// If buffer is not large enough the first char will be set to 0 and the required length as the return value
/// # Safety
///
/// `data` is unchecked, validate your pointers before passing them in.
///
#[no_mangle]
pub unsafe extern "C" fn unit_composition_buf(
    data: *const Unit,
    buffer: *mut c_char,
    length: usize,
) -> i32 {
    error::clear_last_err_msg();

    let unit = &*data;

    move_string_to_buffer(unit.composition().to_string(), buffer, length)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_relative_eq, assert_ulps_eq};
    use ffi_common::core;
    use std::ffi::CString;

    #[test]
    fn can_create_unit() {
        let expression = "[bu_us]/[acr_us]";
        let expression_c = CString::new(expression).expect("CString::new failed");
        unsafe {
            let u = unit_new(expression_c.as_ptr());
            let boxed_u = Box::from_raw(u as *mut Unit);
            assert_eq!(expression, boxed_u.expression());
        }
    }

    #[test]
    fn creating_bad_unit_sets_error() {
        let unit = "meow";
        let expected_error = format!("Unknown unit string fragment: {} (0)", &unit);
        let expression = CString::new(unit).expect("CString::new failed");
        unsafe {
            let u = unit_new(expression.as_ptr());
            assert_eq!(u, ptr::null());
            let error = CStr::from_ptr(core::error::get_last_err_msg())
                .to_str()
                .expect("Failed to get str from CStr.");
            assert_eq!(error, expected_error);
        }
    }

    #[test]
    fn can_destroy_unit() {
        let expression = "[bu_us]/[acr_us]";
        let expression_c = CString::new(expression).expect("CString::new failed");
        unsafe { unit_destroy(unit_new(expression_c.as_ptr())) };
    }

    #[test]
    fn only_equal_units_are_equal() {
        let square_meters = CString::new("m2").expect("CString::new failed");
        let meters = CString::new("m").expect("CString::new failed");
        unsafe {
            let u1 = unit_new(square_meters.as_ptr());
            let u2 = unit_new(square_meters.as_ptr());
            let u3 = unit_new(meters.as_ptr());
            assert!(unit_partial_eq(u1, u2));
            assert!(!unit_partial_eq(u1, u3));
        }
    }

    #[test]
    fn only_special_units_are_special() {
        let celsius = CString::new("Cel").expect("CString::new failed");
        let gram = CString::new("g").expect("CString::new failed");
        unsafe {
            let special_unit = unit_new(celsius.as_ptr());
            assert!(unit_is_special(special_unit));
            let normal_unit = unit_new(gram.as_ptr());
            assert!(!unit_is_special(normal_unit));
        }
    }

    #[test]
    fn can_get_scalar() {
        let expression = CString::new("km").expect("CString::new failed");
        let scalar = 1000.0;
        unsafe {
            let u = unit_new(expression.as_ptr());
            assert_relative_eq!(scalar, unit_scalar(u));
            assert_ulps_eq!(scalar, unit_scalar(u));
        }
    }

    #[test]
    fn can_get_magnitude() {
        let expression = CString::new("10m/5s2").expect("CString::new failed");
        let magnitude = 0.4;
        unsafe {
            let u = unit_new(expression.as_ptr());
            assert_relative_eq!(magnitude, unit_magnitude(u));
            assert_ulps_eq!(magnitude, unit_magnitude(u));
        }
    }

    #[test]
    fn only_compatible_units_are_compatible() {
        let meter_per_second = CString::new("m/s").expect("CString::new failed");
        let kilometers_per_hour = CString::new("km/h").expect("CString::new failed");
        let pounds_per_acre = CString::new("[lb_av]/[acr_us]").expect("CString::new failed");
        unsafe {
            let u1 = unit_new(meter_per_second.as_ptr());
            let u2 = unit_new(kilometers_per_hour.as_ptr());
            let u3 = unit_new(pounds_per_acre.as_ptr());
            assert!(unit_is_compatible_with(u1, u2));
            assert!(!unit_is_compatible_with(u1, u3));
        }
    }

    #[test]
    fn only_valid_units_are_valid() {
        let ten_meters_per_hour_per_second = CString::new("10m2/s/s").expect("CString::new failed");
        let trees = CString::new("trees").expect("CString::new failed");
        unsafe {
            assert!(unit_is_valid(ten_meters_per_hour_per_second.as_ptr()));
            assert!(!unit_is_valid(trees.as_ptr()));
        }
    }

    #[test]
    fn can_get_expression() {
        let expression = CString::new("[lb_av]/[acr_us]").expect("CString::new failed");
        unsafe {
            let u = unit_new(expression.as_ptr());
            let result = CStr::from_ptr(unit_expression(u));
            assert_eq!(expression.to_str(), result.to_str());
        }
    }

    #[test]
    fn can_get_expression_buf() {
        let expression = CString::new("[lb_av]/[acr_us]").expect("CString::new failed");

        unsafe {
            let mut buffer: [c_char; 256] = [0; 256];
            let u = unit_new(expression.as_ptr());
            let result = unit_expression_buf(u, buffer.as_mut_ptr(), 256);
            assert_eq!(result, 16);
            assert_eq!(
                CStr::from_ptr(buffer.as_ptr()).to_str().unwrap(),
                expression.to_str().unwrap()
            );
        }
    }

    #[test]
    fn can_get_reduced_unit() {
        let expression = CString::new("km-1/m2.cm").expect("CString::new failed");
        unsafe {
            let unit = unit_new(expression.as_ptr());
            let reduced = unit_reduced(unit);
            assert_eq!((*reduced).expression().as_str(), "/m2.cm.km");
        }
    }

    #[test]
    fn can_divide() {
        let base_expression = CString::new("10m2").expect("CString::new failed");
        let divisor_expression = CString::new("s").expect("CString::new failed");
        let expected = "10m2/s";
        unsafe {
            let u = unit_new(base_expression.as_ptr());
            let d = unit_new(divisor_expression.as_ptr());
            let result = unit_div(u, d);
            assert_eq!(expected, Box::from_raw(result as *mut Unit).expression());
        }
    }

    #[test]
    fn can_multiply() {
        let base_expression = CString::new("10m2/s").expect("CString::new failed");
        let multiplier_expression = CString::new("s").expect("CString::new failed");
        let expected = "10m2";
        unsafe {
            let u = unit_new(base_expression.as_ptr());
            let m = unit_new(multiplier_expression.as_ptr());
            let result = Box::from_raw(unit_mul(u, m) as *mut Unit);
            assert_eq!(expected, result.expression());
        }
    }
    #[test]
    fn can_get_composition() {
        let expression = CString::new("[acr_us]").expect("CString::new failed");
        unsafe {
            let u = unit_new(expression.as_ptr());
            let result = CStr::from_ptr(unit_composition(u));
            assert_eq!(result.to_str().unwrap(), "L2");
        }
    }
    #[test]
    fn can_get_composition_buf() {
        let expression = CString::new("[acr_us]").expect("CString::new failed");
        unsafe {
            let mut buffer: [c_char; 256] = [0; 256];
            let u = unit_new(expression.as_ptr());
            let result = unit_composition_buf(u, buffer.as_mut_ptr(), 256);
            assert_eq!(result, 2);
            assert_eq!(CStr::from_ptr(buffer.as_ptr()).to_str().unwrap(), "L2");
        }
    }
}
