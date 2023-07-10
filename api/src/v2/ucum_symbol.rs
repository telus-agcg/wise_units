//! These traits are a progression from the existing `crate::UcumSymbol` trait, allowing the same
//! functionality, but also allowing downstream crates to implement for wrapper types.
//!
pub trait UcumClassified {
    type Classification;

    fn classification(&self) -> Self::Classification;
}

impl<T> UcumClassified for T
where
    T: crate::UcumSymbol,
{
    type Classification = crate::Classification;

    fn classification(&self) -> Self::Classification {
        crate::UcumSymbol::classification(self)
    }
}

pub trait UcumIdentifiers {
    type String;
    type Names;

    fn primary_code(&self) -> Self::String;
    fn secondary_code(&self) -> Option<Self::String>;
    fn print_symbol(&self) -> Option<Self::String>;
    fn names(&self) -> Self::Names;
}

pub trait UcumDefinitionValue<T> {
    fn definition_value(&self) -> T;
}

pub trait UcumDefinitionUnit {
    type Unit;

    fn definition_unit(&self) -> Self::Unit;
}

/// Some Atoms have two names; some have one.
pub enum Names<T> {
    One(T),
    Two((T, T)),
}
