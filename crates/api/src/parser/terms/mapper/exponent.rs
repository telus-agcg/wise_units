use pest::iterators::Pair;

use crate::parser::{term, terms::term_parser::Rule as TermRule};

use super::{Digits, Error, Visit};

pub(super) struct Exponent(pub(super) term::Exponent);

impl Visit<TermRule> for Exponent {
    fn visit(pair: Pair<'_, TermRule>) -> Result<Self, Error> {
        let mut pairs = pair.into_inner();

        let first_token = match pairs.next() {
            Some(first) => match first.as_rule() {
                TermRule::sign => {
                    let string = first.as_span().as_str();

                    match string {
                        "+" => FirstToken::PositiveSign,
                        "-" => FirstToken::NegativeSign,
                        _ => unreachable!(),
                    }
                }
                TermRule::digits => FirstToken::Exponent(Digits::visit(first)?),
                _ => unreachable!(),
            },
            None => unreachable!(),
        };

        let exponent = match first_token {
            FirstToken::PositiveSign => parse_second_token(pairs.next())?,
            FirstToken::NegativeSign => -parse_second_token(pairs.next())?,
            FirstToken::Exponent(e) => e,
        };

        Ok(Self(exponent))
    }
}

fn parse_second_token(next: Option<Pair<'_, TermRule>>) -> Result<Digits, Error> {
    next.map_or_else(
        || unreachable!(),
        |second| match second.as_rule() {
            TermRule::digits => Digits::visit(second),
            _ => unreachable!(),
        },
    )
}

enum FirstToken {
    PositiveSign,
    NegativeSign,
    Exponent(term::Exponent),
}
