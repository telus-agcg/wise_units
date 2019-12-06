use crate::{tokens::Annotatable, ParseResult};

pub(crate) fn parse<'input>(input: &'input str) -> ParseResult<'_, Annotatable<'input>> {
    let (simple_unit, su_tail) = crate::simple_unit::parse(input).map_err(|_| input)?;

    let (exponent, tail) = crate::exponent::parse(su_tail)
        .map(|(e, e_tail)| (Some(e), e_tail))
        .unwrap_or_else(|_| (None, su_tail));

    Ok((Annotatable(simple_unit, exponent), tail))
}

#[cfg(test)]
mod tests {
    use crate::tokens::{Annotatable, AtomSymbol, PrefixSymbol, SimpleUnit};

    #[test]
    fn test_parse() {
        assert_eq!(
            super::parse("2km"),
            Ok((
                Annotatable(
                    SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m"),),
                    None
                ),
                ""
            ))
        );

        assert_eq!(
            super::parse("m"),
            Ok((
                Annotatable(SimpleUnit(None, None, AtomSymbol("m")), None),
                ""
            ))
        );

        assert_eq!(
            super::parse("2km2"),
            Ok((
                Annotatable(
                    SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m"),),
                    Some(2)
                ),
                ""
            ))
        );

        assert_eq!(
            super::parse("m2"),
            Ok((
                Annotatable(SimpleUnit(None, None, AtomSymbol("m")), Some(2)),
                ""
            ))
        );

        assert_eq!(
            super::parse("2km!@#"),
            Ok((
                Annotatable(
                    SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m"),),
                    None
                ),
                "!@#"
            ))
        );

        assert_eq!(
            super::parse("m!@#"),
            Ok((
                Annotatable(SimpleUnit(None, None, AtomSymbol("m")), None),
                "!@#"
            ))
        );

        assert_eq!(
            super::parse("2km2!@#"),
            Ok((
                Annotatable(
                    SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m"),),
                    Some(2)
                ),
                "!@#"
            ))
        );

        assert_eq!(
            super::parse("m2!@#"),
            Ok((
                Annotatable(SimpleUnit(None, None, AtomSymbol("m")), Some(2)),
                "!@#"
            ))
        );

        assert_eq!(super::parse("2"), Err("2"));
        assert_eq!(super::parse(""), Err(""));
    }
}
