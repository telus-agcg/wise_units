use super::Atom;

pub trait Property<'a, V> {
    type Atom: Atom<'a, V>;

    fn atoms(&self) -> Vec<Self::Atom>;
}
