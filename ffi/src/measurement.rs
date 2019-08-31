use std::{ffi::CStr, ops::Deref, os::raw::c_char, ptr, str::FromStr};
use wise_units::{Convertible, Error, Measurement as WiseMeasurement, UcumUnit, Unit as WiseUnit};

use crate::unit::Unit;

/// Wrapper for `wise_units::Measurement`. Safe for C interop.
///
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Measurement {
    inner: WiseMeasurement,
}

impl Measurement {
    pub fn new(value: f64, expression: &str) -> Result<Self, Error> {
        let unit = WiseUnit::from_str(expression)?;
        let wm = WiseMeasurement { value, unit };

        Ok(Self::from(wm))
    }
}

impl From<WiseMeasurement> for Measurement {
    fn from(wise_measurement: WiseMeasurement) -> Self {
        Self {
            inner: wise_measurement,
        }
    }
}

impl Deref for Measurement {
    type Target = WiseMeasurement;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Create a new `Measurement`. Note that you must call
/// `measurement_destroy(data: measurement)` with this instance when you are done with
/// it so that the the measurement can be properly destroyed and its memory freed.
///
#[no_mangle]
pub unsafe extern "C" fn measurement_new(
    value: f64,
    expression: *const c_char,
) -> *const Measurement {
    if expression.is_null() {
        return ptr::null();
    };

    match CStr::from_ptr(expression).to_str() {
        Ok(exp_str) => match Measurement::new(value, exp_str) {
            Ok(measurement) => Box::into_raw(Box::new(measurement)),
            Err(_) => ptr::null(),
        },
        Err(_) => ptr::null(),
    }
}

/// Return ownership of `data` to Rust to deallocate safely.
///
#[no_mangle]
pub unsafe extern "C" fn measurement_destroy(data: *const Measurement) {
    let _ = Box::from_raw(data as *mut Measurement);
}

#[no_mangle]
pub unsafe extern "C" fn measurement_partial_eq(
    data: *const Measurement,
    other: *const Measurement,
) -> bool {
    let m1 = &*data;
    let m2 = &*other;
    m1 == m2
}

#[no_mangle]
pub unsafe extern "C" fn measurement_get_unit(data: *const Measurement) -> *const Unit {
    let measurement = &*data;
    // Passing back a clone in this case since the Unit has a lifetime equal to the Measurement.
    let unit = measurement.unit.clone();
    Box::into_raw(Box::new(Unit::from(unit)))
}

// cbindgen blows up when parsing a const fn, so suppress the warning.
#[allow(clippy::missing_const_for_fn)]
#[no_mangle]
pub unsafe extern "C" fn measurement_get_value(data: *const Measurement) -> f64 {
    let measurement = &*data;
    measurement.value
}

#[no_mangle]
pub unsafe extern "C" fn measurement_scalar(data: *const Measurement) -> f64 {
    let measurement = &*data;
    measurement.scalar()
}

#[no_mangle]
pub unsafe extern "C" fn measurement_magnitude(data: *const Measurement) -> f64 {
    let measurement = &*data;
    measurement.magnitude()
}

#[no_mangle]
pub unsafe extern "C" fn measurement_convert_to(
    data: *const Measurement,
    expression: *const c_char,
) -> *const Measurement {
    let measurement = &*data;

    let converted_measurement = match CStr::from_ptr(expression).to_str() {
        Ok(exp_str) => match measurement.convert_to(exp_str) {
            Ok(m) => m,
            Err(_) => return ptr::null_mut(),
        },
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(Measurement::from(converted_measurement)))
}

#[no_mangle]
pub unsafe extern "C" fn measurement_add(
    data: *const Measurement,
    other: *const Measurement,
) -> *const Measurement {
    let m1 = &*data;
    let m2 = &*other;
    let result = match &m1.inner + &m2.inner {
        Ok(m) => Measurement::from(m),
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub unsafe extern "C" fn measurement_sub(
    data: *const Measurement,
    other: *const Measurement,
) -> *const Measurement {
    let m1 = &*data;
    let m2 = &*other;
    let result = match &m1.inner - &m2.inner {
        Ok(m) => Measurement::from(m),
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub unsafe extern "C" fn measurement_mul(
    data: *const Measurement,
    other: *const Measurement,
) -> *const Measurement {
    let m1 = &*data;
    let m2 = &*other;
    let result = &m1.inner * &m2.inner;

    Box::into_raw(Box::new(Measurement::from(result)))
}

#[no_mangle]
pub unsafe extern "C" fn measurement_mul_scalar(
    data: *const Measurement,
    scalar: f64,
) -> *const Measurement {
    let measurement = &*data;
    let result = &measurement.inner * scalar;

    Box::into_raw(Box::new(Measurement::from(result)))
}

#[no_mangle]
pub unsafe extern "C" fn measurement_div(
    data: *const Measurement,
    other: *const Measurement,
) -> *const Measurement {
    let m1 = &*data;
    let m2 = &*other;
    let result = &m1.inner / &m2.inner;

    Box::into_raw(Box::new(Measurement::from(result)))
}

#[no_mangle]
pub unsafe extern "C" fn measurement_div_scalar(
    data: *const Measurement,
    scalar: f64,
) -> *const Measurement {
    let measurement = &*data;
    let result = &measurement.inner / scalar;

    Box::into_raw(Box::new(Measurement::from(result)))
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
}
