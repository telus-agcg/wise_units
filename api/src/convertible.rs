/// Intended for `Measurement`s, it allows for converting using
/// various types to construct the destination `Unit`.
///
pub trait Convertible<RHS> {
    type Output;
    type ConversionError;

    fn convert_to(&self, unit: RHS) -> Result<Self::Output, Self::ConversionError>;
}
