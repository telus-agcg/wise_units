/// A simple trait for getting a type to define how its `Composition` should be built.  Typically
/// `Composition`s are used to determine the compatibility with another type; so long as each type
/// provides a `Composition`, they can be compared for compatibility.
///
pub trait Composable {
    fn composition(&self) -> crate::Composition;
}
