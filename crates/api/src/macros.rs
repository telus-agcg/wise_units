#[macro_export]
macro_rules! measurement {
    ($value:expr, $unit:expr) => {
        $crate::Measurement::try_new($value, $unit).unwrap()
    };
}

#[macro_export]
macro_rules! unit {
    ($($term:expr),+) => {
        $crate::Unit::new(vec![$($term)+])
    };
}

#[macro_export]
macro_rules! parse_unit {
    ($unit_str:expr) => {
        $crate::Unit::from_str($unit_str).unwrap()
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
    (@params $builder:expr, $attribute_name:ident: $attribute_value:expr) => {
        $builder = $builder.$attribute_name($attribute_value);
    };

    ($prefix:ident, $atom:ident, $($attribute_name:ident: $attribute_value:expr),+) => {
        {
            let mut builder = $crate::term::Builder::default()
                .prefix($crate::Prefix::$prefix)
                .atom($crate::Atom::$atom);

            $(
                term!(@params builder, $attribute_name: $attribute_value);
            )+
            builder.build()
        }
    };

    ($prefix:ident, $atom:ident) => {
        $crate::Term::new(Some($crate::Prefix::$prefix), Some($crate::Atom::$atom))
    };

    ($atom:ident, $($attribute_name:ident: $attribute_value:expr),+) => {
        {
            let mut builder = $crate::term::Builder::default()
                .atom($crate::Atom::$atom);
            $(
                term!(@params builder, $attribute_name: $attribute_value);
            )+
            builder.build()
        }
    };

    ($atom:ident) => {
        $crate::Term::Atom($crate::Atom::$atom)
    };

    ($($attribute_name:ident: $attribute_value:expr),+) => {
        {
            let mut builder = $crate::term::Builder::default();
            $(
                term!(@params builder, $attribute_name: $attribute_value);
            )+
            builder.build()
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

#[macro_export]
macro_rules! assert_field_eq {
    ($lhs:expr, $rhs:expr $(,)?) => {
        assert!($crate::FieldEq::field_eq(&$lhs, $rhs));
    };

    ($lhs:expr, $rhs:expr, $($arg:tt)+) => {
        assert!($crate::FieldEq::field_eq(&$lhs, $rhs), $($arg)+);
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
