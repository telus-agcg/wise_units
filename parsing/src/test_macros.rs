//! These macros are only really intended for testing purposes, although may
//! prove useful elsewhere and could then be exposed.
//!
macro_rules! term {
    (
        @params
        $term:expr,
        $attribute1_name:ident: $attribute1_value:expr,
        $attribute2_name:ident: $attribute2_value:expr
    ) => {
        $term.$attribute1_name = $attribute1_value;
        $term.$attribute2_name = $attribute2_value;
    };

    (@params $term:expr, $attribute_name:ident: $attribute_value:expr) => {
        $term.$attribute_name = $attribute_value;
    };

    ($prefix:ident, $atom:ident, $($params:tt)+) => {
        {
            let mut term = Term::new(Some(Atom::$atom), Some(Prefix::$prefix));
            term!(@params term, $($params)+);
            term
        }
    };

    ($prefix:ident, $atom:ident) => {
        Term::new(Some(Atom::$atom), Some(Prefix::$prefix))
    };

    ($atom:ident, $($params:tt)+) => {
        {
            let mut term = Term::new(Some(Atom::$atom), None);
            term!(@params term, $($params)+);
            term
        }
    };

    ($atom:ident) => {
        Term::new(Some(Atom::$atom), None)
    };
}

#[cfg(test)]
mod tests {
    use atom::Atom;
    use prefix::Prefix;
    use term::Term;

    #[test]
    fn validate_term_macro() {
        let expected = Term::new(Some(Atom::Meter), None);
        assert_eq!(term!(Meter), expected);

        let expected = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        assert_eq!(term!(Kilo, Meter), expected);
    }
}
