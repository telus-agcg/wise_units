use super::{Commensurability, Compare, TryToNumber, Ucum};

pub trait WrapperAtom: Ucum + TryToNumber + Compare + Commensurability {
    type String;
    type Names;

    type Classification;
    type Definition;
    type Property;

    fn primary_code(&self) -> Self::String;
    fn secondary_code(&self) -> Option<Self::String>;
    fn print_symbol(&self) -> Option<Self::String>;
    fn names(&self) -> Self::Names;

    fn property(&self) -> Self::Property;
    fn class(&self) -> Self::Classification;
    fn definition(&self) -> Self::Definition;
}
