///
/// The `term!` macro makes building `Term`s slightly more ergonomic and terse. It was initially
/// made for testing purposes (since `Term`s are really mainly used by internal APIs), but since
/// they are used all over the place, it may be useful elsewhere.
///
macro_rules! term {
    (@params $term:expr, $attribute_name:ident: $attribute_value:expr) => {
        $term.$attribute_name = $attribute_value;
    };

    ($prefix:ident, $atom:ident, $($attribute_name:ident: $attribute_value:expr),+) => {
        {
            let mut term = Term::new(Some(Atom::$atom), Some(Prefix::$prefix));
            $(
                term!(@params term, $attribute_name: $attribute_value);
            )+
            term
        }
    };

    ($prefix:ident, $atom:ident) => {
        Term::new(Some(Atom::$atom), Some(Prefix::$prefix))
    };

    ($atom:ident, $($attribute_name:ident: $attribute_value:expr),+) => {
        {
            let mut term = Term::new(Some(Atom::$atom), None);
            $(
                term!(@params term, $attribute_name: $attribute_value);
            )+
            term
        }
    };

    ($atom:ident) => {
        Term::new(Some(Atom::$atom), None)
    };

    ($($attribute_name:ident: $attribute_value:expr),+) => {
        {
            let mut term = Term::new(None, None);
            $(
                term!(@params term, $attribute_name: $attribute_value);
            )+
            term
        }
    };

    () => {
        Term::new(None, None);
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
