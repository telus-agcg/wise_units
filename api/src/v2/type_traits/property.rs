use super::Atom;

pub trait Property<V> {
    type Atom: Atom<V>;

    fn atoms(&self) -> Vec<Self::Atom>;
}
