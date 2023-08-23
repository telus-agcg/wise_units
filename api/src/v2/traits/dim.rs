pub trait Composable {
    type Composition;

    fn composition(&self) -> Self::Composition;
}

// impl<T> Composable for T
// where
//     T: crate::Composable,
// {
//     type Composition = crate::Composition;

//     fn composition(&self) -> Self::Composition {
//         crate::Composable::composition(self)
//     }
// }
