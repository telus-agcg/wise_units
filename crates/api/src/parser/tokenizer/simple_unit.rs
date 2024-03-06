use pest::{iterators::Pairs, pratt_parser::PrattParser};

use crate::{Atom, Prefix};

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
            .map_prefix(|prefix, atom| match prefix.as_rule() {
                Rule::prefix_symbol => match atom {
                    SimpleUnit::Atom(atom_symbol) => SimpleUnit::PrefixAtom {
                        prefix_symbol: prefix.as_str(),
                        atom_symbol,
                    },
                    SimpleUnit::PrefixAtom { .. } => unreachable!(),
                },
                rule => unreachable!("expected factor, found {rule:?}"),
            })
            .parse(pairs)
    }
}

impl<'i> From<SimpleUnit<'i>> for (Option<Prefix>, Option<Atom>) {
    fn from(value: SimpleUnit<'i>) -> Self {
        match value {
            SimpleUnit::PrefixAtom {
                prefix_symbol,
                atom_symbol,
            } => (
                Some(prefix_symbol_to_prefix(prefix_symbol)),
                Some(atom_symbol_to_atom(atom_symbol)),
            ),
            SimpleUnit::Atom(atom_symbol) => (None, Some(atom_symbol_to_atom(atom_symbol))),
        }
    }
}

// We can skip error handling here because the `prefix_symbol` was picked up by the parser, and all
// of those strings must match a `crate::Prefix`'s `primary_symbol`.
//
fn prefix_symbol_to_prefix(prefix_symbol: &str) -> Prefix {
    match prefix_symbol {
        "k" => Prefix::Kilo,
        symbol => unreachable!("unknown prefix symbol: {symbol:#?}"),
    }
}

// We can skip error handling here because the `atom_symbol` was picked up by the parser, and all
// of those strings must match a `crate::Atom`'s `primary_symbol`.
//
fn atom_symbol_to_atom(atom_symbol: &str) -> Atom {
    match atom_symbol {
        "m" => Atom::Meter,
        symbol => unreachable!("unknown atom symbol: {symbol:#?}"),
    }
}
