use measurement::Measurement;
use parser::Error;

/// Intended strictly for `Measurement`s, it allows for converting either from a `str` or a `Unit`.
///
pub trait Convertible<RHS> {
    fn convert_to(&self, unit: RHS) -> Result<Measurement, Error>;
}
