use std::ops::Deref;

pub struct Wrappable<T> {
    wrapped: T,
}

impl<T> Deref for Wrappable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.wrapped
    }
}

pub trait AsWrappedRef<'a> {
    /// This should be the type of reference that's returned. Ideally this would be, say
    /// `&'a Measurement`, but in cases where alternative reference types are used (ex. `RefCell`), this would allow for that (ex. `Ref<'a, Measurement>`).
    ///
    type Reference: std::ops::Deref<Target = Self::RustType>;

    /// This is the type we want to get to for its functionality.
    ///
    type RustType;

    fn as_wrapped_ref(&'a self) -> Self::Reference;
}

#[macro_export]
macro_rules! wrapper_impl_display {
    ($dest:ty) => {
        impl<'a> std::fmt::Display for $dest {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.as_wrapped_ref().fmt(f)
            }
        }
    };
}
