#![deny(unused_extern_crates)]
#![warn(
    future_incompatible,
    missing_copy_implementations,
    // missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications,
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::perf,
    clippy::nursery,
    clippy::style
)]

pub mod tokens;

mod annotatable;
mod annotation;
mod atom_symbol;
mod component;
mod digits;
mod exponent;
mod factor;
mod main_term;
mod prefix_symbol;
mod sign;
mod simple_unit;
mod term;

#[cfg(test)]
mod test_helper;

type ParseResult<'a, O> = Result<(O, &'a str), &'a str>;

#[derive(Debug, PartialEq)]
pub enum Error {
    Unparsable(String),
    PartialMatch { matching: String, remaining: String },
}

#[inline]
pub fn parse(input: &'_ str) -> Result<crate::tokens::MainTerm<'_>, Error> {
    match crate::main_term::parse(input) {
        Ok((mt, tail)) => {
            if !tail.is_empty() {
                let matching_index = input.find(tail).unwrap();
                let matching = &input[..matching_index];

                let e = Error::PartialMatch {
                    matching: matching.to_string(),
                    remaining: tail.to_string(),
                };

                return Err(e);
            }

            Ok(mt)
        }
        Err(_) => Err(Error::Unparsable(input.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use crate::tokens::*;

    #[test]
    fn test_parse_valid() {
        let expected = MainTerm {
            term: Term::Basic(Box::new(Component::Factor(0))),
            leading_slash: true,
        };
        assert_eq!(super::parse("/0"), Ok(expected));
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(
            super::parse("!@#$"),
            Err(Error::Unparsable("!@#$".to_string()))
        );

        let e = Error::PartialMatch {
            matching: "2km2".to_string(),
            remaining: "!@#$".to_string(),
        };
        assert_eq!(super::parse("2km2!@#$"), Err(e));
    }
}
