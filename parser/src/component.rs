use crate::{tokens::Component, ParseResult};

pub(crate) fn parse<'input>(input: &'input str) -> ParseResult<'_, Component<'input>> {
    if let Ok((annotatable, ann_tail)) = crate::annotatable::parse(input) {
        if let Ok((annotation, tail)) = crate::annotation::parse(ann_tail) {
            return Ok((
                Component::AnnotatedAnnotatable(annotatable, annotation),
                tail,
            ));
        } else {
            return Ok((Component::Annotatable(annotatable), ann_tail));
        }
    }

    if let Ok((annotation, tail)) = crate::annotation::parse(input) {
        return Ok((Component::Annotation(annotation), tail));
    }

    if let Ok((factor, tail)) = crate::factor::parse(input) {
        return Ok((Component::Factor(factor), tail));
    }

    parse_nested_term(input)
}

fn parse_nested_term<'input>(input: &'input str) -> ParseResult<'_, Component<'input>> {
    if input.starts_with('(') {
        let (term, tail) = crate::term::parse(&input[1..]).map_err(|_| input)?;

        if tail.starts_with(')') {
            Ok((Component::NestedTerm(term), &tail[1..]))
        } else {
            Err(input)
        }
    } else {
        Err(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::{
        Annotatable, Annotation, AtomSymbol, Component, PrefixSymbol, SimpleUnit, Term,
    };

    #[test]
    fn test_parse_annotated_annotation() {
        assert_eq!(
            super::parse("2km2{things}"),
            Ok((
                Component::AnnotatedAnnotatable(
                    Annotatable(
                        SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m")),
                        Some(2)
                    ),
                    Annotation("things")
                ),
                ""
            ))
        );
        assert_eq!(
            super::parse("2km2{things}stuff"),
            Ok((
                Component::AnnotatedAnnotatable(
                    Annotatable(
                        SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m")),
                        Some(2)
                    ),
                    Annotation("things")
                ),
                "stuff"
            ))
        );
        assert_eq!(
            super::parse("2M[acr_us]2{things}"),
            Ok((
                Component::AnnotatedAnnotatable(
                    Annotatable(
                        SimpleUnit(Some(2), Some(PrefixSymbol("M")), AtomSymbol("[acr_us]")),
                        Some(2)
                    ),
                    Annotation("things")
                ),
                ""
            ))
        );
        assert_eq!(
            super::parse("2km2{things}stuff"),
            Ok((
                Component::AnnotatedAnnotatable(
                    Annotatable(
                        SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m")),
                        Some(2)
                    ),
                    Annotation("things")
                ),
                "stuff"
            ))
        );
    }

    #[test]
    fn test_parse_annotatable() {
        assert_eq!(
            super::parse("2km2"),
            Ok((
                Component::Annotatable(Annotatable(
                    SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m")),
                    Some(2)
                )),
                ""
            ))
        );
        assert_eq!(
            super::parse("2km2asdf"),
            Ok((
                Component::Annotatable(Annotatable(
                    SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m")),
                    Some(2)
                )),
                "asdf"
            ))
        );
    }

    #[test]
    fn test_parse_annotation() {
        assert_eq!(
            super::parse("{things}"),
            Ok((Component::Annotation(Annotation("things")), ""))
        );
        assert_eq!(
            super::parse("{things}!@#$"),
            Ok((Component::Annotation(Annotation("things")), "!@#$"))
        );
    }

    #[test]
    fn test_parse_factor() {
        assert_eq!(super::parse("2"), Ok((Component::Factor(2), "")));
        assert_eq!(super::parse("2!@#$"), Ok((Component::Factor(2), "!@#$")));
    }

    #[test]
    fn test_nested_term() {
        assert_eq!(
            super::parse("(2km2)"),
            Ok((
                Component::NestedTerm(Term::Basic(Box::new(Component::Annotatable(Annotatable(
                    SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m"),),
                    Some(2)
                ))))),
                ""
            ))
        )
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(super::parse(""), Err(""));
    }
}
