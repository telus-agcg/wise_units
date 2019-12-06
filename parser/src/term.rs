use crate::{
    tokens::{Component, Term, TermSeparator},
    ParseResult,
};

pub(crate) fn parse<'input>(input: &'input str) -> ParseResult<'_, Term<'input>> {
    // TODO: see if this has any overhead
    if input.is_empty() {
        return Err(input);
    }

    let (component, component_tail) = crate::component::parse(input).map_err(|_| input)?;

    if component_tail.starts_with('.') {
        parse_dot_term(&component_tail[1..], component)
    } else if component_tail.starts_with('/') {
        parse_slash_term(&component_tail[1..], component)
    } else {
        Ok((Term::Basic(Box::new(component)), component_tail))
    }
}

fn parse_dot_term<'input>(
    input: &'input str,
    component: Component<'input>,
) -> ParseResult<'input, Term<'input>> {
    parse(input).map(|(term, tail)| {
        let t = Term::Combined(Box::new(component), TermSeparator::Dot, Box::new(term));
        (t, tail)
    })
}

fn parse_slash_term<'input>(
    input: &'input str,
    component: Component<'input>,
) -> ParseResult<'input, Term<'input>> {
    parse(input).map(|(term, tail)| {
        let t = Term::Combined(Box::new(component), TermSeparator::Slash, Box::new(term));
        (t, tail)
    })
}

#[cfg(test)]
mod tests {
    use crate::tokens::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_component_only() {
        let component = Component::Annotatable(Annotatable(
            SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m")),
            Some(2),
        ));

        let expected = Term::Basic(Box::new(component));
        assert_eq!(super::parse("2km2"), Ok((expected, "")));
    }

    #[test]
    fn test_parse_dot() {
        let term2 = Term::Basic(Box::new(Component::Annotatable(Annotatable(
            SimpleUnit(Some(3), Some(PrefixSymbol("m")), AtomSymbol("g")),
            Some(3),
        ))));

        let expected = Term::Combined(
            Box::new(Component::Annotatable(Annotatable(
                SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m")),
                Some(2),
            ))),
            TermSeparator::Dot,
            Box::new(term2),
        );

        assert_eq!(super::parse("2km2.3mg3"), Ok((expected.clone(), "")));
        assert_eq!(super::parse("2km2.3mg3!@#$"), Ok((expected, "!@#$")));
    }

    #[test]
    fn test_parse_dot_nested_dot() {
        let term3 = Term::Basic(Box::new(Component::Annotatable(Annotatable(
            SimpleUnit(Some(4), Some(PrefixSymbol("d")), AtomSymbol("[acr_us]")),
            Some(4),
        ))));

        let term2 = Term::Combined(
            Box::new(Component::Annotatable(Annotatable(
                SimpleUnit(Some(3), Some(PrefixSymbol("m")), AtomSymbol("g")),
                Some(3),
            ))),
            TermSeparator::Dot,
            Box::new(term3),
        );

        let expected = Term::Combined(
            Box::new(Component::Annotatable(Annotatable(
                SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m")),
                Some(2),
            ))),
            TermSeparator::Dot,
            Box::new(term2),
        );

        assert_eq!(
            super::parse("2km2.3mg3.4d[acr_us]4"),
            Ok((expected.clone(), ""))
        );
        assert_eq!(
            super::parse("2km2.3mg3.4d[acr_us]4!@#"),
            Ok((expected, "!@#"))
        );
    }

    #[test]
    fn test_parse_dot_nested_slash() {
        let term3 = Term::Basic(Box::new(Component::Annotatable(Annotatable(
            SimpleUnit(Some(4), Some(PrefixSymbol("d")), AtomSymbol("[acr_us]")),
            Some(4),
        ))));

        let term2 = Term::Combined(
            Box::new(Component::Annotatable(Annotatable(
                SimpleUnit(Some(3), Some(PrefixSymbol("m")), AtomSymbol("g")),
                Some(3),
            ))),
            TermSeparator::Slash,
            Box::new(term3),
        );

        let expected = Term::Combined(
            Box::new(Component::Annotatable(Annotatable(
                SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m")),
                Some(2),
            ))),
            TermSeparator::Dot,
            Box::new(term2),
        );

        assert_eq!(
            super::parse("2km2.3mg3/4d[acr_us]4"),
            Ok((expected.clone(), ""))
        );
        assert_eq!(
            super::parse("2km2.3mg3/4d[acr_us]4!@#"),
            Ok((expected, "!@#"))
        );
    }

    #[test]
    fn test_parse_slash() {
        let term2 = Term::Basic(Box::new(Component::Annotatable(Annotatable(
            SimpleUnit(Some(3), Some(PrefixSymbol("m")), AtomSymbol("g")),
            Some(3),
        ))));

        let expected = Term::Combined(
            Box::new(Component::Annotatable(Annotatable(
                SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m")),
                Some(2),
            ))),
            TermSeparator::Slash,
            Box::new(term2),
        );

        assert_eq!(super::parse("2km2/3mg3"), Ok((expected.clone(), "")));
        assert_eq!(super::parse("2km2/3mg3!@#$"), Ok((expected, "!@#$")));
    }

    #[test]
    fn test_parse_slash_nested_slash() {
        let term3 = Term::Basic(Box::new(Component::Annotatable(Annotatable(
            SimpleUnit(Some(4), Some(PrefixSymbol("d")), AtomSymbol("[acr_us]")),
            Some(4),
        ))));

        let term2 = Term::Combined(
            Box::new(Component::Annotatable(Annotatable(
                SimpleUnit(Some(3), Some(PrefixSymbol("m")), AtomSymbol("g")),
                Some(3),
            ))),
            TermSeparator::Slash,
            Box::new(term3),
        );

        let expected = Term::Combined(
            Box::new(Component::Annotatable(Annotatable(
                SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m")),
                Some(2),
            ))),
            TermSeparator::Slash,
            Box::new(term2),
        );

        assert_eq!(
            super::parse("2km2/3mg3/4d[acr_us]4"),
            Ok((expected.clone(), ""))
        );
        assert_eq!(
            super::parse("2km2/3mg3/4d[acr_us]4!@#"),
            Ok((expected, "!@#"))
        );
    }

    #[test]
    fn test_parse_slash_nested_dot() {
        let term3 = Term::Basic(Box::new(Component::Annotatable(Annotatable(
            SimpleUnit(Some(4), Some(PrefixSymbol("d")), AtomSymbol("[acr_us]")),
            Some(4),
        ))));

        let term2 = Term::Combined(
            Box::new(Component::Annotatable(Annotatable(
                SimpleUnit(Some(3), Some(PrefixSymbol("m")), AtomSymbol("g")),
                Some(3),
            ))),
            TermSeparator::Dot,
            Box::new(term3),
        );

        let expected = Term::Combined(
            Box::new(Component::Annotatable(Annotatable(
                SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m")),
                Some(2),
            ))),
            TermSeparator::Slash,
            Box::new(term2),
        );

        assert_eq!(
            super::parse("2km2/3mg3.4d[acr_us]4"),
            Ok((expected.clone(), ""))
        );
        assert_eq!(
            super::parse("2km2/3mg3.4d[acr_us]4!@#"),
            Ok((expected, "!@#"))
        );
    }
}
