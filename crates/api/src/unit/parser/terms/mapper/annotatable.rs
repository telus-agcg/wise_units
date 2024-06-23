use pest::iterators::Pair;

use crate::{
    term::UNITY,
    unit::parser::{terms::term_parser::Rule as TermRule, Error, Visit},
    Term,
};

use super::{Exponent, SimpleUnit};

pub(super) struct Annotatable(pub(super) Term);

impl Visit<'_, TermRule> for Annotatable {
    fn visit(pair: Pair<'_, TermRule>) -> Result<Self, Error> {
        let mut pairs = pair.into_inner();

        let mut simple_unit = match pairs.next() {
            Some(first) => match first.as_rule() {
                TermRule::simple_unit => SimpleUnit::visit(first)?,
                _ => unreachable!(),
            },
            None => unreachable!(),
        };

        if simple_unit.0 == UNITY {
            Ok(Self(simple_unit.0))
        } else {
            match pairs.next() {
                Some(second) => match second.as_rule() {
                    TermRule::exponent => {
                        let _ = simple_unit.0.set_exponent(Exponent::visit(second)?.0);
                        Ok(Self(simple_unit.0))
                    }
                    _ => unreachable!(),
                },
                None => Ok(Self(simple_unit.0)),
            }
        }
    }
}
