use num_traits::Inv;
use pest::{iterators::Pairs, pratt_parser::PrattParser};

use super::{Parse, Rule, Term, TryParse};

#[derive(Debug, PartialEq)]
pub(in crate::parser) enum MainTerm<'i> {
    SlashTerm(Term<'i>),
    Term(Term<'i>),
}

impl<'i> TryParse<'i> for MainTerm<'i> {
    fn try_parse(
        pairs: Pairs<'i, Rule>,
        pratt: &PrattParser<Rule>,
    ) -> Result<Self, crate::parser::Error> {
        pratt
            .map_primary(|primary| match primary.as_rule() {
                Rule::term => Ok(MainTerm::Term(Term::parse(primary.into_inner(), pratt))),
                rule => unreachable!("expected term, found {rule:?}"),
            })
            .map_prefix(|op, rhs| match op.as_rule() {
                Rule::leading_slash => match rhs {
                    Ok(MainTerm::Term(term)) => Ok(MainTerm::SlashTerm(term)),
                    Ok(MainTerm::SlashTerm(_)) | Err(_) => unreachable!(),
                },
                rule => unreachable!("expected leading_slash, found {rule:?}"),
            })
            .map_postfix(|main_term, op| match op.as_rule() {
                Rule::EOI => main_term,
                rule => Err(crate::parser::Error::UnknownUnitString(format!(
                    "expected EOI, found {rule:?}"
                ))),
            })
            .parse(pairs)
    }
}

impl TryFrom<MainTerm<'_>> for crate::Unit {
    type Error = crate::parser::Error;

    fn try_from(main_term: MainTerm<'_>) -> Result<Self, Self::Error> {
        match main_term {
            MainTerm::SlashTerm(term) => {
                let unit = Self::try_from(term)?;
                Ok(unit.inv())
            }
            MainTerm::Term(term) => Self::try_from(term),
        }
    }
}
