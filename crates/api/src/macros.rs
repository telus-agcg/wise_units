#[macro_export]
macro_rules! measurement {
    ($value:expr, $unit:expr) => {
        $crate::Measurement::try_new($value, $unit).unwrap()
    };
}

/// The `term!` macro makes building `Term`s slightly more ergonomic and terse.
/// It was initially made for testing purposes (since `Term`s are really mainly
/// used by internal APIs), but since they are used all over the place, it may
/// be useful elsewhere.
///
#[macro_export]
#[allow(clippy::field_reassign_with_default)]
macro_rules! term {
    (@params $term:expr, $attribute_name:ident: $attribute_value:expr) => {
        $term.$attribute_name = Some($attribute_value);
    };

    ($prefix:ident, $atom:ident, $($attribute_name:ident: $attribute_value:expr),+) => {
        {
            let mut term = $crate::Term::new(Some($crate::Prefix::$prefix), Some($crate::Atom::$atom));
            $(
                term!(@params term, $attribute_name: $attribute_value);
            )+
            term
        }
    };

    ($prefix:ident, $atom:ident) => {
        $crate::Term::new(Some($crate::Prefix::$prefix), Some($crate::Atom::$atom))
    };

    ($atom:ident, $($attribute_name:ident: $attribute_value:expr),+) => {
        {
            let mut term = $crate::Term::new(None, Some($crate::Atom::$atom));
            $(
                term!(@params term, $attribute_name: $attribute_value);
            )+
            term
        }
    };

    ($atom:ident) => {
        $crate::Term::new(None, Some($crate::Atom::$atom))
    };

    ($($attribute_name:ident: $attribute_value:expr),+) => {
        {
            let mut term = $crate::Term::default();
            $(
                term!(@params term, $attribute_name: $attribute_value);
            )+
            term
        }
    };

    () => {
        $crate::Term::default()
    };
}

#[macro_export]
macro_rules! terms {
    ($($term:expr),*) => {
        std::borrow::Cow::<[$crate::Term]>::Owned(vec![$($term)*])
    };
}

#[cfg(test)]
mod tests {
    use crate::{Atom, Prefix, Term};

    #[test]
    fn validate_term_macro() {
        let expected = Term::new(None, Some(Atom::Meter));
        assert_eq!(term!(Meter), expected);

        let expected = Term::new(Some(Prefix::Kilo), Some(Atom::Meter));
        assert_eq!(term!(Kilo, Meter), expected);
    }
}

