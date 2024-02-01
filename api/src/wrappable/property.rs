use super::WrapperAtom;

pub trait WrapperProperty {
    type Atom: WrapperAtom;

    fn atoms(&self) -> Vec<Self::Atom>;
}
