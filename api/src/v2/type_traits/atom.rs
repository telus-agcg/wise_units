use super::dimension::Dimension;

pub trait Atom {
    type String;
    type Names;
    type Property;
    type Class;
    type Dimension: Dimension;
    type Definition;

    fn primary_code(&self) -> Self::String;
    fn secondary_code(&self) -> Option<Self::String>;
    fn print_symbol(&self) -> Option<Self::String>;
    fn names(&self) -> Self::Names;

    fn is_special(&self) -> bool;
    fn is_metric(&self) -> bool;
    fn is_arbitrary(&self) -> bool;

    fn class(&self) -> Self::Class;
    fn property(&self) -> Self::Property;

    fn dim(&self) -> Self::Dimension;

    fn definition(&self) -> Self::Definition;
}
