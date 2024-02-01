// NOTE: The difference with this trait is that it takes a `&mut self` instead of `self`, allowing
// it to be implemented a bit more conventionally on types: ex. `impl Invert on Term` instead of
// `impl Invert on &mut Term`.
//
pub trait InvertMut {
    fn invert_mut(&mut self);
}

/// Similar to `Invert`, but allows for checking for divide-by-0 errors before converting; should
/// return `None` in that case.
///
pub trait CheckedInvert {
    fn checked_invert(&mut self) -> Option<()>;
}

// NOTE: The difference with this trait is that it's generic over `T`, allowing
// for multiple implementations.
//
pub trait ToInverse<T = Self> {
    fn to_inverse(&self) -> T;
}

// NOTE: The difference with this trait is that it's generic over `T`, allowing
// for multiple implementations.
//
pub trait CheckedToInverse<T = Self> {
    fn checked_to_inverse(&self) -> Option<T>;
}
