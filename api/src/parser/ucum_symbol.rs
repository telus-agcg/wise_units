use crate::{parser::Classification, unit::Unit};

pub trait UcumSymbol: Send + Sync + Copy + Clone {
    fn classification(&self) -> Classification;
    fn primary_code(&self) -> &'static str;
    fn print_symbol(&self) -> Option<&'static str>;
    fn names(&self) -> Vec<&'static str>;
    fn secondary_code(&self) -> Option<&'static str>;

    /// A `UcumSymbol`'s `definition_value` is the scalar value of how many
    /// unit this symbol can be expressed in. For example, an `[in_i]`
    /// (international inch) is defined as 2.54 cm; in that case, the
    /// `definition_value` for `[in_i]` is `2.54`.
    ///
    fn definition_value(&self) -> f64;

    /// A `UcumSymbol`'s `definition_value` is the scalar value of how many
    /// unit this symbol can be expressed in. For example, an `[in_i]`
    /// (international inch) is defined as 2.54 cm; in that case, the
    /// `definition_value` for `[in_i]` is `2.54`.
    ///
    fn definition_unit(&self) -> Unit;
}
