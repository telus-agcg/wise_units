/// Intended for `Measurement`s, it allows for converting using various types to construct the
/// destination `Unit`.
///
pub trait Convertible<RHS> {
    type Output;
    type ConversionError;

    /// _The_ method for doing the conversion.
    ///
    /// # Errors
    ///
    /// If `self` can't be converted, it should return a `Self::ConversionError`.
    ///
    fn convert_to(&self, unit: RHS) -> Result<Self::Output, Self::ConversionError>;
}
