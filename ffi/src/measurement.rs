use ffi_common::error;
use std::{ffi::CStr, os::raw::c_char, ptr};
use wise_units::{reduce::ToReduced, Convertible, Measurement, UcumUnit, Unit};

/// Create a new `Measurement`. Note that you must call
/// `measurement_destroy(data: measurement)` with this instance when you are done with
/// it so that the measurement can be properly destroyed and its memory freed.
///
/// # Safety
///
/// `expression` is checked that it's null, but not checked for its type.
///
/// # Errors
///
/// * If the `expression` can't be converted to a `&str`.
/// * If the `Measurement`'s `Unit` can't be created using the `expression` string.
///
#[no_mangle]
pub unsafe extern "C" fn measurement_new(
    value: f64,
    expression: *const c_char,
) -> *const Measurement {
    error::clear_last_err_msg();

    if expression.is_null() {
        return ptr::null();
    };

    match CStr::from_ptr(expression).to_str() {
        Ok(exp_str) => match Measurement::new(value, exp_str) {
            Ok(measurement) => Box::into_raw(Box::new(measurement)),
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
pub unsafe extern "C" fn measurement_destroy(data: *const Measurement) {
    let _ = Box::from_raw(data as *mut Measurement);
}

/// Essentially checks if the two `Measurement`'s scalar values are equal.
///
/// # Safety
///
/// `data` and `other` are unchecked, so validate them before passing them in.
///
#[no_mangle]
pub unsafe extern "C" fn measurement_partial_eq(
    data: *const Measurement,
    other: *const Measurement,
) -> bool {
    let m1 = &*data;
    let m2 = &*other;
    m1 == m2
}

/// Gets the `Measurement`'s `Unit` (the non-`value` part of the `Measurement`). 
/// Note that you must call `unit_destroy(data: unit)` with
/// this return value when you are done with it so that the the unit can be properly
/// destroyed and its memory freed.
///
/// # Safety
///
/// `data` is unchecked, so validate it before passing it in.
///
#[no_mangle]
pub unsafe extern "C" fn measurement_get_unit(data: *const Measurement) -> *const Unit {
    let measurement = &*data;
    // Passing back a clone in this case since the Unit has a lifetime equal to the Measurement.
    let unit = measurement.unit.clone();
    Box::into_raw(Box::new(unit))
}

/// Gets the `Measurement`'s `value` (the non-`Unit` part of the `Measurement`).
///
/// # Safety
///
/// `data` is unchecked, so validate it before passing it in.
///
// cbindgen blows up when parsing a const fn, so suppress the warning.
#[allow(clippy::missing_const_for_fn)]
#[no_mangle]
pub unsafe extern "C" fn measurement_get_value(data: *const Measurement) -> f64 {
    let measurement = &*data;
    measurement.value
}

/// Gets the `Measurement`'s scalar value (expressed in terms of its `Unit`'s base unit).
///
/// # Safety
///
/// `data` is unchecked, so validate it before passing it in.
///
#[no_mangle]
pub unsafe extern "C" fn measurement_scalar(data: *const Measurement) -> f64 {
    let measurement = &*data;
    measurement.scalar()
}

/// Gets the `Measurement`'s magnitude.
///
/// # Safety
///
/// `data` is unchecked, so validate it before passing it in.
///
#[no_mangle]
pub unsafe extern "C" fn measurement_magnitude(data: *const Measurement) -> f64 {
    let measurement = &*data;
    measurement.magnitude()
}

/// Tries to convert a `Measurement` to the `expression` that represents a `Unit`.
///
/// # Safety
///
/// `data` and `expression` are unchecked, so validate them before passing them in.
///
/// # Errors
///
/// If the `Measurement` can't be converted to the `Unit` represented by `expression`
/// (uncommensurable `Unit`s), the error is set and a `null` pointer is returned.
///
/// Also, if `expression` can't be converted to a `&str`, an error is set and a `null` pointer is
/// returned
///
#[no_mangle]
pub unsafe extern "C" fn measurement_convert_to(
    data: *const Measurement,
    expression: *const c_char,
) -> *const Measurement {
    error::clear_last_err_msg();

    let measurement = &*data;

    let converted_measurement = match CStr::from_ptr(expression).to_str() {
        Ok(exp_str) => match measurement.convert_to(exp_str) {
            Ok(m) => m,
            Err(why) => return crate::set_error_and_return(why.to_string()),
        },
        Err(why) => return crate::set_error_and_return(why.to_string()),
    };

    Box::into_raw(Box::new(converted_measurement))
}

/// Tries to reduce the `Measurement`'s `Unit` terms.
///
/// # Safety
///
/// `data` is unchecked, so validate is before passing is in.
///
/// # Errors
///
/// If the reduction process fails, the error is set and a `null` pointer returned.
///
#[no_mangle]
pub unsafe extern "C" fn measurement_reduced(data: *const Measurement) -> *const Measurement {
    error::clear_last_err_msg();

    let original = &*data;
    let reduced = match original.to_reduced() {
        Ok(r) => r,
        Err(why) => return crate::set_error_and_return(why.to_string()),
    };
    Box::into_raw(Box::new(reduced))
}

/// Adds a `Measurement` to another `Measurement`.
///
/// # Safety
///
/// `data` and `other` are unchecked, so validate them before passing them in.
///
/// # Errors
///
/// If the two `Measurement`s can't be added (uncommensurable `Unit`s), the error is set and a
/// `null` pointer is returned.
///
#[no_mangle]
pub unsafe extern "C" fn measurement_add(
    data: *const Measurement,
    other: *const Measurement,
) -> *const Measurement {
    error::clear_last_err_msg();

    let m1 = &*data;
    let m2 = &*other;
    let result = match m1 + m2 {
        Ok(m) => m,
        Err(why) => return crate::set_error_and_return(why.to_string()),
    };

    Box::into_raw(Box::new(result))
}

/// Subtracts a `Measurement` from another `Measurement`.
///
/// # Safety
///
/// `data` and `other` are unchecked, so validate them before passing them in.
///
/// # Errors
///
/// If the two `Measurement`s can't be subtracted (uncommensurable `Unit`s), the error is set and
/// a `null` pointer is returned.
///
#[no_mangle]
pub unsafe extern "C" fn measurement_sub(
    data: *const Measurement,
    other: *const Measurement,
) -> *const Measurement {
    error::clear_last_err_msg();

    let m1 = &*data;
    let m2 = &*other;
    let result = match m1 - m2 {
        Ok(m) => m,
        Err(why) => return crate::set_error_and_return(why.to_string()),
    };

    Box::into_raw(Box::new(result))
}

/// Multiplies a `Measurement` by another `Measurement`.
///
/// # Safety
///
/// `data` and `other` are unchecked, so validate them before passing them in.
///
#[no_mangle]
pub unsafe extern "C" fn measurement_mul(
    data: *const Measurement,
    other: *const Measurement,
) -> *const Measurement {
    let m1 = &*data;
    let m2 = &*other;
    let result = m1 * m2;

    Box::into_raw(Box::new(result))
}

/// Multiplies a `Measurement` by a scalar value.
///
/// # Safety
///
/// `data` is unchecked, so validate it before passing it in.
///
#[no_mangle]
pub unsafe extern "C" fn measurement_mul_scalar(
    data: *const Measurement,
    scalar: f64,
) -> *const Measurement {
    let measurement = &*data;
    let result = measurement * scalar;

    Box::into_raw(Box::new(result))
}

/// Divides a `Measurement` by another `Measurement`.
///
/// # Safety
///
/// `data` and `other` are unchecked, so validate them before passing them in.
///
#[no_mangle]
pub unsafe extern "C" fn measurement_div(
    data: *const Measurement,
    other: *const Measurement,
) -> *const Measurement {
    let m1 = &*data;
    let m2 = &*other;
    let result = m1 / m2;

    Box::into_raw(Box::new(result))
}

/// Divides a `Measurement` by a scalar value.
///
/// # Safety
///
/// `data` is unchecked, so validate it before passing it in.
///
#[no_mangle]
pub unsafe extern "C" fn measurement_div_scalar(
    data: *const Measurement,
    scalar: f64,
) -> *const Measurement {
    let measurement = &*data;
    let result = measurement / scalar;

    Box::into_raw(Box::new(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_relative_eq, assert_ulps_eq};
    use std::ffi::CString;

    #[test]
    fn can_create_measurement() {
        let value = 14.0;
        let expression = CString::new("m").expect("CString::new failed");
        unsafe {
            let m = measurement_new(value, expression.as_ptr());
            let boxed_m = Box::from_raw(m as *mut Measurement);
            assert_relative_eq!(value, boxed_m.value);
            assert_ulps_eq!(value, boxed_m.value);
        }
    }

    #[test]
    fn can_destroy_measurement() {
        let value = 14.0;
        let expression = CString::new("[bu_us]/[acr_us]").expect("CString::new failed");
        unsafe { measurement_destroy(measurement_new(value, expression.as_ptr())) };
    }

    #[test]
    fn only_equal_measurements_are_equal() {
        let meter = CString::new("m").expect("CString::new failed");
        let kilometer = CString::new("km").expect("CString::new failed");
        unsafe {
            let m1 = measurement_new(1000.0, meter.as_ptr());
            let m2 = measurement_new(1.0, kilometer.as_ptr());
            let m3 = measurement_new(1.1, kilometer.as_ptr());
            assert!(measurement_partial_eq(m1, m2));
            assert!(!measurement_partial_eq(m1, m3));
        }
    }

    #[test]
    fn can_get_unit() {
        let expression = "m";
        let expression_c = CString::new(expression).expect("CString::new failed");
        unsafe {
            let m = measurement_new(1.0, expression_c.as_ptr());
            let unit = measurement_get_unit(m);
            let boxed_unit = Box::from_raw(unit as *mut Unit);
            assert_eq!(expression, boxed_unit.expression());
        }
    }

    #[test]
    fn can_get_value() {
        let expression = CString::new("m").expect("CString::new failed");
        let value = 42.42152;
        unsafe {
            let m = measurement_new(value, expression.as_ptr());
            assert_relative_eq!(value, measurement_get_value(m));
            assert_ulps_eq!(value, measurement_get_value(m));
        }
    }

    #[test]
    fn can_get_scalar() {
        let expression = CString::new("m").expect("CString::new failed");
        let value = 42.42152;
        unsafe {
            let m = measurement_new(value, expression.as_ptr());
            assert_relative_eq!(value, measurement_scalar(m));
            assert_ulps_eq!(value, measurement_scalar(m));
        }
    }

    #[test]
    fn can_get_magnitude() {
        let expression = CString::new("m").expect("CString::new failed");
        let value = 42.42152;
        unsafe {
            let m = measurement_new(value, expression.as_ptr());
            assert_relative_eq!(value, measurement_magnitude(m));
            assert_ulps_eq!(value, measurement_magnitude(m));
        }
    }

    #[test]
    fn can_convert() {
        let expression1 = CString::new("[lb_av]/[acr_us]").expect("CString::new failed");
        let expression2 = CString::new("kg/har").expect("CString::new failed");
        let value = 12.0;
        let expected = 13.450_160_073_531_777;
        unsafe {
            let m = measurement_new(value, expression1.as_ptr());
            let converted =
                Box::from_raw(measurement_convert_to(m, expression2.as_ptr()) as *mut Measurement);
            assert_relative_eq!(converted.value, expected);
            assert_ulps_eq!(converted.value, expected);
        }
    }

    #[test]
    fn can_reduce() {
        let expression = CString::new("[acr_us]/m2/har").expect("CString::new failed");
        let value = 1.0;
        let expected_expression = "[acr_us]";
        let expected_value = 10_000.0;
        unsafe {
            let m = measurement_new(value, expression.as_ptr());
            let r = measurement_reduced(m);
            assert_eq!((*r).unit.expression(), expected_expression);
            assert_relative_eq!((*r).value, expected_value);
            assert_ulps_eq!((*r).value, expected_value);
        }
    }

    #[test]
    fn can_add() {
        let expression1 = CString::new("km").expect("CString::new failed");
        let expression2 = CString::new("m").expect("CString::new failed");
        let value1 = 42.42152;
        let value2 = 4.825;
        let expected = 42.426_345;
        unsafe {
            let m1 = measurement_new(value1, expression1.as_ptr());
            let m2 = measurement_new(value2, expression2.as_ptr());
            let result = Box::from_raw(measurement_add(m1, m2) as *mut Measurement);
            assert_relative_eq!(result.value, expected);
            assert_ulps_eq!(result.value, expected);
        }
    }

    #[test]
    fn can_subtract() {
        let expression1 = CString::new("km").expect("CString::new failed");
        let expression2 = CString::new("m").expect("CString::new failed");
        let value1 = 42.42152;
        let value2 = 4.825;
        let expected = 42.416_695;
        unsafe {
            let m1 = measurement_new(value1, expression1.as_ptr());
            let m2 = measurement_new(value2, expression2.as_ptr());
            let result = Box::from_raw(measurement_sub(m1, m2) as *mut Measurement);
            assert_relative_eq!(result.value, expected);
            assert_ulps_eq!(result.value, expected);
        }
    }

    #[test]
    fn can_multiply() {
        let expression1 = CString::new("m").expect("CString::new failed");
        let expression2 = CString::new("km").expect("CString::new failed");
        let value1 = 42.42152;
        let value2 = 4.825;
        let expected = 204_683.834;
        unsafe {
            let m1 = measurement_new(value1, expression1.as_ptr());
            let m2 = measurement_new(value2, expression2.as_ptr());
            let result = Box::from_raw(measurement_mul(m1, m2) as *mut Measurement);
            assert_relative_eq!(result.value, expected);
            assert_ulps_eq!(result.value, expected);
        }
    }

    #[test]
    fn can_multiply_scalar() {
        let expression = CString::new("kg").expect("CString::new failed");
        let value1 = 42.42152;
        let scalar = 4.825;
        let expected = 204.683_834;
        unsafe {
            let m1 = measurement_new(value1, expression.as_ptr());
            let result = Box::from_raw(measurement_mul_scalar(m1, scalar) as *mut Measurement);
            assert_relative_eq!(result.value, expected);
            assert_ulps_eq!(result.value, expected);
        }
    }

    #[test]
    fn can_divide() {
        let expression1 = CString::new("m").expect("CString::new failed");
        let expression2 = CString::new("km").expect("CString::new failed");
        let value1 = 42.42152;
        let value2 = 4.825;
        let expected = 0.008_792_024_870_466_321;
        unsafe {
            let m1 = measurement_new(value1, expression1.as_ptr());
            let m2 = measurement_new(value2, expression2.as_ptr());
            let result = Box::from_raw(measurement_div(m1, m2) as *mut Measurement);
            assert_relative_eq!(result.value, expected);
            assert_ulps_eq!(result.value, expected);
        }
    }

    #[test]
    fn can_divide_scalar() {
        let expression = CString::new("kg").expect("CString::new failed");
        let value1 = 42.42352;
        let scalar = 4.825;
        let expected = 8.792_439_378_238_342;
        unsafe {
            let m1 = measurement_new(value1, expression.as_ptr());
            let result = Box::from_raw(measurement_div_scalar(m1, scalar) as *mut Measurement);
            assert_relative_eq!(result.value, expected);
            assert_ulps_eq!(result.value, expected);
        }
    }

    #[test]
    fn invalid_conversion_produces_error() {
        let expression1 = CString::new("[lb_av]/[acr_us]").expect("CString::new failed");
        let expression2 = CString::new("m").expect("CString::new failed");
        let value = 12.0;
        let expected_error = r#"Units are not compatible: "[lb_av]/[acr_us]", "m""#;
        unsafe {
            let m = measurement_new(value, expression1.as_ptr());
            let converted = measurement_convert_to(m, expression2.as_ptr());
            assert_eq!(converted, ptr::null());
            let error = CStr::from_ptr(ffi_common::ffi::get_last_err_msg());
            let error_str = error.to_str().expect("Failed to get str from CStr");
            assert_eq!(error_str, expected_error);
        }
    }

    #[test]
    fn error_is_cleared_on_subsequent_ops() {
        let expression1 = CString::new("[lb_av]/[acr_us]").expect("CString::new failed");
        let expression2 = CString::new("m").expect("CString::new failed");
        let expression3 = CString::new("[lb_av]/m2").expect("CString::new failed");
        let value = 12.0;
        unsafe {
            let m = measurement_new(value, expression1.as_ptr());
            // result is null, error is not null
            let conversion_result_1 = measurement_convert_to(m, expression2.as_ptr());
            assert_eq!(conversion_result_1, ptr::null());
            assert_ne!(ffi_common::ffi::get_last_err_msg(), ptr::null());
            // result is not null, error is null
            let conversion_result_2 = measurement_convert_to(m, expression3.as_ptr());
            assert_ne!(conversion_result_2, ptr::null());
            assert_eq!(ffi_common::ffi::get_last_err_msg(), ptr::null());
        }
    }
}
