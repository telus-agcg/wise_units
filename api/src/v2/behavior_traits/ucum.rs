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

use crate::v2::type_traits::dimension::Dimension;

pub trait Dimensionable {
    type Output: Dimension;

    fn dim(&self) -> Self::Output;
}

// pub trait DefinitionFlags {
//     fn is_special(&self) -> bool;
//     fn is_metric(&self) -> bool;
//     fn is_arbitrary(&self) -> bool;
// }

/// Some Atoms have two names; some have one.
pub enum Names<T> {
    One(T),
    Two((T, T)),
}
