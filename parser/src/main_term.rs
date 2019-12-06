use crate::{tokens::MainTerm, ParseResult};

pub(crate) fn parse<'input>(input: &'input str) -> ParseResult<'_, MainTerm<'input>> {
    let mut leading_slash = false;

    let tail = if input.starts_with('/') {
        leading_slash = true;
        &input[1..]
    } else {
        input
    };

    crate::term::parse(tail)
        .map(|(term, new_tail)| {
            let mt = MainTerm {
                leading_slash,
                term,
            };

            (mt, new_tail)
        })
        .map_err(|_| input)
}

#[cfg(test)]
mod tests {
    use crate::tokens::*;

    #[test]
    fn test_parse_no_slash() {
        let expected = MainTerm {
            term: Term::Basic(Box::new(Component::Factor(2))),
            leading_slash: false,
        };
        assert_eq!(super::parse("2"), Ok((expected.clone(), "")));
        assert_eq!(super::parse("2!@#$"), Ok((expected, "!@#$")));

        let component = Component::Annotatable(Annotatable(
            SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m")),
            Some(2),
        ));

        let term = Term::Basic(Box::new(component));

        let expected = MainTerm {
            term,
            leading_slash: false,
        };

        assert_eq!(super::parse("2km2"), Ok((expected.clone(), "")));
        assert_eq!(super::parse("2km2!@#$"), Ok((expected, "!@#$")));
    }

    #[test]
    fn test_parse_with_slash() {
        let expected = MainTerm {
            term: Term::Basic(Box::new(Component::Factor(0))),
            leading_slash: true,
        };
        assert_eq!(super::parse("/0"), Ok((expected.clone(), "")));
        assert_eq!(super::parse("/0!@#$"), Ok((expected, "!@#$")));
    }
}
