use crate::{number::Number, prefixes::PREFIXES, Error};
use std::convert::TryFrom;
use wise_units_parser::tokens as parser_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Prefix {
    pub primary_code: &'static str,
    pub value: Number,
}

impl TryFrom<parser_tokens::PrefixSymbol<'_>> for Prefix {
    type Error = Error;

    fn try_from(parser_prefix: parser_tokens::PrefixSymbol<'_>) -> Result<Prefix, Self::Error> {
        PREFIXES
            .binary_search_by(|prefix| prefix.primary_code.cmp(parser_prefix.0))
            .map(|i| PREFIXES[i])
            .map_err(|_| Error::UnknownPrefixSymbol(parser_prefix.0.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use std::convert::TryFrom;
    use wise_units_parser::tokens as parser_tokens;

    #[test]
    fn test_prefixes_ok() {
        let prefix = Prefix::try_from(parser_tokens::PrefixSymbol("m")).unwrap();

        assert_eq!(
            prefix,
            Prefix {
                primary_code: "m",
                value: Number::Float(1e-3)
            }
        );
    }

    #[test]
    fn test_prefixes_err() {
        let error = Prefix::try_from(parser_tokens::PrefixSymbol("1")).unwrap_err();

        assert_eq!(error, Error::UnknownPrefixSymbol("1".into()));
    }
}
