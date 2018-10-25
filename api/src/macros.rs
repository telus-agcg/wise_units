/// The `term!` macro makes building `Term`s slightly more ergonomic and terse.
/// It was initially made for testing purposes (since `Term`s are really mainly
/// used by internal APIs), but since they are used all over the place, it may
/// be useful elsewhere.
///
#[macro_export]
macro_rules! term {
    (@params $term:expr, $attribute_name:ident: $attribute_value:expr) => {
        $term.$attribute_name = Some($attribute_value);
    };

    ($prefix:ident, $atom:ident, $($attribute_name:ident: $attribute_value:expr),+) => {
        {
            let mut term = Term::new(Some(Prefix::$prefix), Some(Atom::$atom));
            $(
                term!(@params term, $attribute_name: $attribute_value);
            )+
            term
        }
    };

    ($prefix:ident, $atom:ident) => {
        Term::new(Some(Prefix::$prefix), Some(Atom::$atom))
    };

    ($atom:ident, $($attribute_name:ident: $attribute_value:expr),+) => {
        {
            let mut term = Term::new(None, Some(Atom::$atom));
            $(
                term!(@params term, $attribute_name: $attribute_value);
            )+
            term
        }
    };

    ($atom:ident) => {
        Term::new(None, Some(Atom::$atom))
    };

    ($($attribute_name:ident: $attribute_value:expr),+) => {
        {
            let mut term = Term::default();
            $(
                term!(@params term, $attribute_name: $attribute_value);
            )+
            term
        }
    };

    () => {
        Term::default();
    };
}

#[cfg(test)]
mod tests {
    use crate::parser::{Atom, Prefix, Term};

    #[test]
    fn validate_term_macro() {
        let expected = Term::new(None, Some(Atom::Meter));
        assert_eq!(term!(Meter), expected);

        let expected = Term::new(Some(Prefix::Kilo), Some(Atom::Meter));
        assert_eq!(term!(Kilo, Meter), expected);
    }
}
