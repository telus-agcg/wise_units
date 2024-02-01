use super::{Commensurability, Compare, TryToNumber, Ucum, WrapperAtom, WrapperPrefix};

pub trait WrapperTerm: Ucum + TryToNumber + Compare + Commensurability {
    type Prefix: WrapperPrefix;
    type Atom: WrapperAtom;
    type Annotation: ?Sized;

    // Accessors
    fn factor(&self) -> Option<u32>;
    fn prefix(&self) -> Option<&Self::Prefix>;
    fn atom(&self) -> Option<&Self::Atom>;
    fn exponent(&self) -> Option<i32>;
    fn annotation(&self) -> Option<&Self::Annotation>;
}
