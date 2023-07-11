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
macro_rules! wrapper_impl_v2_composable {
    ($dest:ty, $composition:ty) => {
        impl $crate::v2::Composable for $dest {
            type Composition = $composition;

            fn composition(&self) -> Self::Composition {
                Self::Composition::from($crate::Composable::composition(&*self.as_wrapped_ref()))
            }
        }
    };
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

#[macro_export]
macro_rules! wrapper_impl_v2_ucum_unit_flags {
    ($dest:ty) => {
        impl $crate::v2::ucum_unit::UcumUnitFlags for $dest {
            fn is_special(&self) -> bool {
                $crate::UcumUnit::is_special(&*self.as_wrapped_ref())
            }

            fn is_metric(&self) -> bool {
                $crate::UcumUnit::is_metric(&*self.as_wrapped_ref())
            }

            fn is_arbitrary(&self) -> bool {
                $crate::UcumUnit::is_arbitrary(&*self.as_wrapped_ref())
            }
        }
    };
}
