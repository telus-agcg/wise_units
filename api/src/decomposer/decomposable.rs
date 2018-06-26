/// Defines an interface for deconstructing a Unit into a String that represents
/// the Terms that define it.
///
pub(crate) trait Decomposable {
    /// The main function clients should care about.
    ///
    fn expression(&self) -> String {
        let numerator = self.numerator();
        let denominator = self.denominator();

        if denominator.is_empty() {
            numerator
        } else {
            format!("{}/{}", numerator, denominator)
        }
    }

    fn numerator(&self) -> String;
    fn denominator(&self) -> String;
}
