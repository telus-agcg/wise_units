use crate::{
    atom_collection::AtomCollection,
    parse::{ParseResult, ParseSymbol},
    symbol_collection::SymbolCollection,
    tokens::AtomSymbol,
};

pub(super) struct Parser;

impl<'input> ParseSymbol<'input, AtomCollection, AtomSymbol> for Parser {
    fn parse_symbol(
        input: &'input str,
        atoms: &'static AtomCollection,
    ) -> ParseResult<'input, &'static AtomSymbol> {
        atoms
            .find_match(input)
            .map(|(atom_symbol, atom_str_len)| (atom_symbol, &input[atom_str_len..]))
            .ok_or(input)
    }
}

#[cfg(test)]
mod tests {
    use super::{ParseSymbol, Parser};
    use crate::units::UNITS;

    macro_rules! assert_parse {
        ($input:expr, $expected_const:expr, $expected_remaining:expr) => {
            let (symbol, tail) = Parser::parse_symbol($input, &UNITS).unwrap();
            assert_eq!(&symbol.primary_code(), $expected_const);
            assert_eq!(tail, $expected_remaining);
        };
    }

    #[test]
    fn test_all_the_atoms_tail_shift1234() {
        for primary_code in crate::test_helper::ATOMS.iter() {
            let tail = "!@#$";
            let primary_input = format!("{primary_code}{tail}");
            assert_parse!(&primary_input, primary_code, tail);
        }
    }

    #[test]
    fn test_all_the_atoms_tail_1234() {
        for primary_code in crate::test_helper::ATOMS.iter() {
            let tail = "1234";
            let primary_input = format!("{primary_code}{tail}");
            assert_parse!(&primary_input, primary_code, tail);
        }
    }

    #[test]
    fn test_all_the_atoms_bracket_tail() {
        for primary_code in crate::test_helper::ATOMS.iter() {
            let tail = "]";
            let primary_input = format!("{primary_code}{tail}");
            assert_parse!(&primary_input, primary_code, tail);
        }
    }

    #[test]
    fn test_parse_errors() {
        assert_eq!(Parser::parse_symbol("0", &UNITS), Err("0"));
        assert_eq!(Parser::parse_symbol("", &UNITS), Err(""));
        assert_eq!(Parser::parse_symbol("[asdf", &UNITS), Err("[asdf"));
    }
}
