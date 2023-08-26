use super::Atom;

pub trait Property {
    type Atom: Atom;

    fn atoms(&self) -> Vec<Self::Atom>;
}
