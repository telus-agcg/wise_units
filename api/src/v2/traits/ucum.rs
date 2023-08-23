//! These traits are a progression from the existing `crate::UcumSymbol` trait, allowing the same
//! functionality, but also allowing downstream crates to implement for wrapper types.
//!

/// Defines how to parse a string of unit symbols into a `Unit`.
///
pub trait ParseUcumStr<'a, U = Self> {
    type String: ?Sized;
    type Error;

    /// # Errors
    ///
    /// This should error if the `ucum_str` can't be parsed into a type that represents that
    /// combinations of units.
    ///
    fn parse_ucum_str(ucum_str: Self::String) -> Result<U, Self::Error>;
}

pub trait DefinitionIdentifiers {
    type String;
    type Names;

    fn primary_code(&self) -> Self::String;
    fn secondary_code(&self) -> Option<Self::String>;
    fn print_symbol(&self) -> Option<Self::String>;
    fn names(&self) -> Self::Names;
}

pub trait Dim {
    type Dimension;

    fn dim(&self) -> Self::Dimension;
}

pub trait DefinitionFlags {
    fn is_special(&self) -> bool;
    fn is_metric(&self) -> bool;
    fn is_arbitrary(&self) -> bool;
}

pub trait Property {
    type Property;

    fn property(&self) -> Self::Property;
}

pub trait DefinitionValue<T> {
    fn definition_value(&self) -> T;
}

pub trait DefinitionUnit {
    type Unit;

    fn definition_unit(&self) -> Self::Unit;
}

/// Some Atoms have two names; some have one.
pub enum Names<T> {
    One(T),
    Two((T, T)),
}
