//! These traits are a progression from the existing `crate::UcumSymbol` trait, allowing the same
//! functionality, but also allowing downstream crates to implement for wrapper types.
//!

pub trait FromUcumStr {
    type String: ?Sized;
    type Output;

    fn from_ucum_str(ucum_str: &Self::String) -> Self::Output;
}

// NOTE: This used to revolve around the term "classification", but that's nowhere to be found in
// the UCUM spec; this term was borrowed from the `unitwise` Ruby gem during the initial port to
// `wise_units`.
//
pub trait UcumDim {
    type Dimension;

    fn dim(&self) -> Self::Dimension;
}

// impl<T> UcumDim for T
// where
//     T: crate::UcumSymbol,
// {
//     type Dimension = crate::Classification;

//     fn dim(&self) -> Self::Dimension {
//         crate::UcumSymbol::classification(self)
//     }
// }

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

impl<T> UcumDefinitionValue<f64> for T
where
    T: crate::UcumSymbol,
{
    fn definition_value(&self) -> f64 {
        crate::UcumSymbol::definition_value(self)
    }
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

pub trait UnitFlags {
    fn is_special(&self) -> bool;
    fn is_metric(&self) -> bool;
    fn is_arbitrary(&self) -> bool;
}

// impl<T> UcumUnitFlags for T
// where
//     T: crate::UcumUnit,
// {
//     fn is_special(&self) -> bool {
//         crate::UcumUnit::is_special(self)
//     }

//     fn is_metric(&self) -> bool {
//         crate::UcumUnit::is_metric(self)
//     }

//     fn is_arbitrary(&self) -> bool {
//         crate::UcumUnit::is_arbitrary(self)
//     }
// }
