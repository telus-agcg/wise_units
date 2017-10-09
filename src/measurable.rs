use unit::Unit;

pub trait Measurable {
    fn get_unit(&self) -> &Unit;
    fn get_value(&self) -> f64;
    fn scalar(&self) -> f64;
    fn magnitude(&self) -> f64;

    /// Checks if the associated Unit is "special". "Special" units are ones
    /// that must be converted using a function in combination with some other
    /// non-special units. For example, Celsius is special since it must be
    /// first converted to Kelvin before converting to the requested unit.
    ///
    fn is_special(&self) -> bool { self.get_unit().is_special() }
}
