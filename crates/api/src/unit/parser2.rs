#![allow(dead_code)]

#[cfg(test)]
#[cfg_attr(test, macro_export)]
macro_rules! validate_parse {
    (unit_str: $expression:expr, parser: $parser_fn:ident, expected: $expected:expr) => {
        eprintln!("Validating: {}", $expression);

        let (tail, output) = $parser_fn($expression.as_bytes()).unwrap();

        assert!(
            tail.is_empty(),
            "tail was: {}",
            std::str::from_utf8(tail).unwrap()
        );

        pretty_assertions::assert_eq!(output, $expected);
    };
}

mod annotatable;
mod component;
mod main_term;
mod simple_unit;
mod term;

use nom::{combinator::all_consuming, IResult};

pub(super) fn parse_unit(input: &str) -> IResult<&[u8], crate::Unit> {
    let _main_term = all_consuming(main_term::parse)(input.as_bytes())?;
    todo!();
}
