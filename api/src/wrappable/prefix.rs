use super::Ucum;

pub trait WrapperPrefix: Ucum {
    type String;
    type Number;
    type Composition;
    type Classification;

    // Base methods, defined by the UCUM spec.
    fn primary_code(&self) -> Self::String;
    fn secondary_code(&self) -> Self::String;
    fn name(&self) -> Self::String;
    fn print_symbol(&self) -> Self::String;
    fn value(&self) -> Self::Number;

    // Methods built into the library to enable functionality or fulfill trait.
    // contracts.
    fn class(&self) -> Self::Classification;
}
