//! These traits are a progression from the existing `crate::UcumSymbol` trait, allowing the same
//! functionality, but also allowing downstream crates to implement for wrapper types.
//!

// pub trait DefinitionIdentifiers {
//     type String;
//     type Names;

//     fn primary_code(&self) -> Self::String;
//     fn secondary_code(&self) -> Option<Self::String>;
//     fn print_symbol(&self) -> Option<Self::String>;
//     fn names(&self) -> Self::Names;
// }

// where
//     D: Default + Copy + PartialEq + Mul<i32, Output = D>,
pub trait Dim<D> {
    fn dim(&self) -> D;
}

// pub trait DefinitionFlags {
//     fn is_special(&self) -> bool;
//     fn is_metric(&self) -> bool;
//     fn is_arbitrary(&self) -> bool;
// }

// pub trait DefinitionValue<T> {
//     fn definition_value(&self) -> T;
// }

// pub trait DefinitionUnit {
//     type Unit;

//     fn definition_unit(&self) -> Self::Unit;
// }

/// Some Atoms have two names; some have one.
pub enum Names<T> {
    One(T),
    Two((T, T)),
}
