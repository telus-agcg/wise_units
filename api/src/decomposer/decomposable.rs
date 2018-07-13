/// Defines an interface for deconstructing a Unit into a String that represents
/// the Terms that define it.
///
pub trait Decomposable<'a> {
    type Terms;
    type Collection;

    /// The main function clients should care about.
    ///
    fn decompose(&self, terms: Self::Terms) -> String {
        let collection = self.terms_to_collection(terms);

        let numerator = self.numerator(&collection);
        let denominator = self.denominator(&collection);

        if denominator.is_empty() {
            numerator
        } else {
            format!("{}/{}", numerator, denominator)
        }
    }

    fn numerator(&self, collection: &Self::Collection) -> String;
    fn denominator(&self, collection: &Self::Collection) -> String;
    fn terms_to_collection(&self, terms: Self::Terms) -> Self::Collection;
}
