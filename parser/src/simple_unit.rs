use crate::{tokens::SimpleUnit, ParseResult};

pub(crate) fn parse<'input>(input: &'input str) -> ParseResult<'_, SimpleUnit<'input>> {
    let (factor, factor_tail) = crate::factor::parse(input)
        .map(|(f, t)| (Some(f), t))
        .unwrap_or_else(|_| (None, input));

    let (mut prefix, prefix_tail) = crate::prefix_symbol::parse(factor_tail)
        .map(|(ps, t)| (Some(ps), t))
        .unwrap_or_else(|_| (None, factor_tail));

    let (atom, tail) = crate::atom_symbol::parse(prefix_tail).or_else(|_| {
        if prefix.is_some() {
            prefix = None;

            crate::atom_symbol::parse(factor_tail).map_err(|_| input)
        } else {
            Err(input)
        }
    })?;

    Ok((SimpleUnit(factor, prefix, atom), tail))
}

#[cfg(test)]
mod tests {
    use crate::tokens::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            super::parse("2km!@#$"),
            Ok((
                SimpleUnit(Some(2), Some(PrefixSymbol("k")), AtomSymbol("m")),
                "!@#$"
            ))
        );

        assert_eq!(
            super::parse("2m!@#$"),
            Ok((SimpleUnit(Some(2), None, AtomSymbol("m")), "!@#$"))
        );

        assert_eq!(
            super::parse("m!#@$"),
            Ok((SimpleUnit(None, None, AtomSymbol("m")), "!#@$"))
        );

        assert_eq!(
            super::parse("m123"),
            Ok((SimpleUnit(None, None, AtomSymbol("m")), "123"))
        );

        assert_eq!(
            super::parse("2M[acr_us]!@#$"),
            Ok((
                SimpleUnit(Some(2), Some(PrefixSymbol("M")), AtomSymbol("[acr_us]")),
                "!@#$"
            ))
        );

        assert_eq!(
            super::parse("2[acr_us]!@#$"),
            Ok((SimpleUnit(Some(2), None, AtomSymbol("[acr_us]")), "!@#$"))
        );

        assert_eq!(
            super::parse("[acr_us]!#@$"),
            Ok((SimpleUnit(None, None, AtomSymbol("[acr_us]")), "!#@$"))
        );

        assert_eq!(
            super::parse("[acr_us]123"),
            Ok((SimpleUnit(None, None, AtomSymbol("[acr_us]")), "123"))
        );

        assert_eq!(super::parse("0"), Err("0"));
        assert_eq!(super::parse(""), Err(""));
    }
}
