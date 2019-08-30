use std::{ffi::CStr, ops::Deref, os::raw::c_char, ptr, str::FromStr};
use wise_units::{is_compatible_with::IsCompatibleWith, Error, UcumUnit, Unit as WiseUnit};

/// Wrapper for `wise_units::Unit`. Safe for C interop.
///
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Unit {
    pub inner: WiseUnit,
}

impl FromStr for Unit {
    type Err = Error;

    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        let wise_unit = WiseUnit::from_str(expression)?;
        Ok(Self::from(wise_unit))
    }
}

impl From<WiseUnit> for Unit {
    fn from(wise_unit: WiseUnit) -> Self {
        Self { inner: wise_unit }
    }
}

impl Deref for Unit {
    type Target = WiseUnit;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Create a new `Unit`. Note that you must call `unit_destroy(data: unit)` with
/// this instance when you are done with it so that the the unit can be properly
/// destroyed and its memory freed.
///
#[no_mangle]
pub unsafe extern "C" fn unit_new(expression: *const c_char) -> *const Unit {
    if expression.is_null() {
        return ptr::null();
    };

    match CStr::from_ptr(expression).to_str() {
        Ok(exp_str) => match Unit::from_str(exp_str) {
            Ok(unit) => Box::into_raw(Box::new(unit)),
            Err(_) => ptr::null(),
        },
        Err(_) => ptr::null(),
    }
}

/// Return ownership of `data` to Rust to deallocate safely.
///
#[no_mangle]
pub unsafe extern "C" fn unit_destroy(data: *const Unit) {
    let _ = Box::from_raw(data as *mut Unit);
}

#[no_mangle]
pub unsafe extern "C" fn unit_partial_eq(data: *const Unit, other: *const Unit) -> bool {
    let unit = &*data;
    let other = &*other;
    unit == other
}

#[no_mangle]
pub unsafe extern "C" fn unit_is_special(data: *const Unit) -> bool {
    let unit = &*data;
    unit.is_special()
}

#[no_mangle]
pub unsafe extern "C" fn unit_scalar(data: *const Unit) -> f64 {
    let unit = &*data;
    unit.scalar()
}

#[no_mangle]
pub unsafe extern "C" fn unit_magnitude(data: *const Unit) -> f64 {
    let unit = &*data;
    unit.magnitude()
}

#[no_mangle]
pub unsafe extern "C" fn unit_is_compatible_with(data: *const Unit, other: *const Unit) -> bool {
    let unit1 = &*data;
    let unit2 = &*other;
    unit1.inner.is_compatible_with(&unit2.inner)
}

#[no_mangle]
pub unsafe extern "C" fn unit_is_valid(expression: *const c_char) -> bool {
    if expression.is_null() {
        return false;
    };

    match CStr::from_ptr(expression).to_str() {
        Ok(exp_str) => Unit::from_str(exp_str).is_ok(),
        Err(_) => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn unit_div(data: *const Unit, other: *const Unit) -> *const Unit {
    let unit = &*data;
    let other = &*other;
    let quotient = Unit::from(&unit.inner / &other.inner);
    let quotient_box = Box::new(quotient);
    Box::into_raw(quotient_box)
}

#[no_mangle]
pub unsafe extern "C" fn unit_mul(data: *const Unit, other: *const Unit) -> *const Unit {
    let unit = &*data;
    let other = &*other;
    let product = Unit::from(&unit.inner * &other.inner);
    let product_box = Box::new(product);
    Box::into_raw(product_box)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_relative_eq, assert_ulps_eq};
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

}
