use crate::{
    parse::{ParseResult, ParseSymbol},
    symbol_collection::SymbolCollection,
    tokens::PrefixSymbol,
    PrefixCollection,
};

pub(super) struct Parser;

impl<'input> ParseSymbol<'input, PrefixCollection, PrefixSymbol> for Parser {
    fn parse_symbol(
        input: &'input str,
        prefixes: &'static PrefixCollection,
    ) -> ParseResult<'input, &'static PrefixSymbol> {
        prefixes
            .find_match(input)
            .map(|(prefix_symbol, prefix_str_len)| (prefix_symbol, &input[prefix_str_len..]))
            .ok_or(input)
    }
}

#[cfg(test)]
mod tests {
    use super::{ParseSymbol, Parser};
    use crate::prefixes::{self, PREFIXES};

    #[test]
    fn test_parse() {
        assert_eq!(
            Parser::parse_symbol("k", &PREFIXES),
            Ok((&prefixes::KILO, ""))
        );
        assert_eq!(Parser::parse_symbol("0", &PREFIXES), Err("0"));
        assert_eq!(Parser::parse_symbol("", &PREFIXES), Err(""));
    }
}
