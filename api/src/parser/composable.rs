use parser::Composition;

/// The main purpose of `Composable` is to get you to be able to determine
/// compatibility with other unit-like structs/enums. You solely need to define
/// how a thing is composed (using `composition()`, then you can compare
/// `Composable`s with `is_compatible_with()`. Two things are compatible if
/// they `Composition`s are equal.
///
pub trait Composable {
    fn composition(&self) -> Composition;

    fn is_compatible_with<T: Composable>(&self, other_unit: &T) -> bool {
        let me = self.composition();
        let other_comp = other_unit.composition();

        me == other_comp
    }
}
