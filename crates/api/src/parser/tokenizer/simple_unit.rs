use pest::{iterators::Pairs, pratt_parser::PrattParser};

use super::{Parse, Rule};

#[derive(Debug, PartialEq)]
pub(in crate::parser) enum SimpleUnit<'i> {
    PrefixAtom {
        prefix_symbol: &'i str,
        atom_symbol: &'i str,
    },
    Atom(&'i str),
}

impl<'i> Parse<'i> for SimpleUnit<'i> {
    fn parse(pairs: Pairs<'i, Rule>, pratt: &PrattParser<Rule>) -> Self {
        pratt
            .map_primary(|primary| match primary.as_rule() {
                Rule::metric_atom_symbol | Rule::non_metric_atom_symbol => {
                    SimpleUnit::Atom(primary.as_str())
                }
                rule => unreachable!("expected *atom_symbol, found {rule:?}"),
            })
            .map_prefix(|op, rhs| match op.as_rule() {
                Rule::prefix_symbol => match rhs {
                    SimpleUnit::Atom(atom_symbol) => SimpleUnit::PrefixAtom {
                        prefix_symbol: op.as_str(),
                        atom_symbol,
                    },
                    SimpleUnit::PrefixAtom { .. } => unreachable!(),
                },
                rule => unreachable!("expected factor, found {rule:?}"),
            })
            .parse(pairs)
    }
}
