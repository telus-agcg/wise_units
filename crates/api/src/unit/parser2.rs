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

pub(super) fn parse(input: &str) -> Result<self::main_term::MainTerm<'_>, ()> {
    let (_, main_term) = main_term::parse(input.as_bytes()).unwrap();

    Ok(main_term)
}
