use crate::measurement::Measurement;
use crate::reducible::Reducible;
use crate::ucum_unit::UcumUnit;

#[cfg_attr(feature = "cffi", ffi_common::derive::expose_impl)]
impl UcumUnit for Measurement {
    /// Checks if the associated Unit is "special". "Special" units are ones
    /// that must be converted using a function in combination with some other
    /// non-special units. For example, Celsius is special since it must be
    /// first converted to Kelvin before converting to the requested unit.
    ///
    fn is_special(&self) -> bool {
        self.unit.is_special()
    }

    fn is_arbitrary(&self) -> bool {
        self.unit.is_arbitrary()
    }

    fn is_metric(&self) -> bool {
        self.unit.is_metric()
    }

    /// This scalar is the Measurement's value combined with any scalars that
    /// are part of the Unit's designation.
    ///
    /// # Examples
    ///
    /// ```
    /// use wise_units::{Measurement, UcumUnit};
    ///
    /// let five_meters = Measurement::try_new(5.0, "m").unwrap();
    /// assert_eq!(five_meters.scalar(), 5.0);
    ///
    /// let five_meters_squared = Measurement::try_new(5.0, "m2").unwrap();
    /// assert_eq!(five_meters_squared.scalar(), 5.0);
    ///
    /// let five_three_meters = Measurement::try_new(5.0, "[pi].m").unwrap();
    /// assert_eq!(five_three_meters.scalar(), 15.707_963_267_948_966);
    ///
    /// let sixty_five_f = Measurement::try_new(65.0, "[degF]").unwrap();
    /// assert!((sixty_five_f.scalar() - 291.483_333).abs() < 0.000_001);
    /// ```
    ///
    #[inline]
    fn scalar(&self) -> f64 {
        self.reduce_value(self.value)
    }

    /// This magnitude is the Measurement's value combined with any magnitude
    /// that is part of the Unit's designation.
    ///
    /// # Examples
    ///
    /// ```
    /// use wise_units::{Measurement, UcumUnit};
    ///
    /// let five_meters = Measurement::try_new(5.0, "m").unwrap();
    /// assert_eq!(five_meters.magnitude(), 5.0);
    ///
    /// let five_meters_squared = Measurement::try_new(5.0, "m2").unwrap();
    /// assert_eq!(five_meters_squared.magnitude(), 5.0);
    ///
    /// let five_three_meters = Measurement::try_new(5.0, "[pi].m").unwrap();
    /// assert_eq!(five_three_meters.magnitude(), 5.0);
    ///
    /// let sixty_five_f = Measurement::try_new(65.0, "[degF]").unwrap();
    /// assert!((sixty_five_f.magnitude() - 65.0).abs() < 0.000_001);
    /// ```
    ///
    #[inline]
    fn magnitude(&self) -> f64 {
        self.calculate_magnitude(self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::measurement::Measurement;
    use crate::ucum_unit::UcumUnit;
    use approx::assert_ulps_eq;

    #[test]
    fn validate_scalar() {
        let m = Measurement::try_new(1.0, "m").unwrap();
        assert_ulps_eq!(m.scalar(), 1.0);

        let m = Measurement::try_new(2.3, "m").unwrap();
        assert_ulps_eq!(m.scalar(), 2.3);

        let m = Measurement::try_new(1.0, "km").unwrap();
        assert_ulps_eq!(m.scalar(), 1000.0);

        let m = Measurement::try_new(2.3, "km").unwrap();
        assert_ulps_eq!(m.scalar(), 2300.0);

        let m = Measurement::try_new(1.0, "g/L").unwrap();
        assert_ulps_eq!(m.scalar(), 1000.0);

        let m = Measurement::try_new(2.3, "g/L").unwrap();
        assert_ulps_eq!(m.scalar(), 2300.0);

        let m = Measurement::try_new(20.0, "Cel").unwrap();
        assert_ulps_eq!(m.scalar(), 293.15);
    }

    #[test]
    fn validate_magnitude() {
        let m = Measurement::try_new(1.0, "m").unwrap();
        assert_ulps_eq!(m.magnitude(), 1.0);

        let m = Measurement::try_new(2.3, "m").unwrap();
        assert_ulps_eq!(m.magnitude(), 2.3);

        let m = Measurement::try_new(1.0, "km").unwrap();
        assert_ulps_eq!(m.magnitude(), 1000.0);

        let m = Measurement::try_new(2.3, "km").unwrap();
        assert_ulps_eq!(m.magnitude(), 2300.0);

        let m = Measurement::try_new(1.0, "g/L").unwrap();
        assert_ulps_eq!(m.magnitude(), 1.0);

        let m = Measurement::try_new(2.3, "g/L").unwrap();
        assert_ulps_eq!(m.magnitude(), 2.3);

        let m = Measurement::try_new(20.0, "g/10L").unwrap();
        assert_ulps_eq!(m.magnitude(), 2.0);

        let m = Measurement::try_new(20.0, "Cel").unwrap();
        assert_ulps_eq!(m.magnitude(), 20.0);
    }
}
