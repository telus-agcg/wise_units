use proc_macro2::TokenStream;

#[derive(Debug)]
pub(crate) struct RustFunctionSet {
    /// The function to use to convert from a unit to this one.
    ///
    pub(crate) convert_from: TokenStream,

    /// The function to use to convert to a unit from this one.
    ///
    pub(crate) convert_to: TokenStream,
}
