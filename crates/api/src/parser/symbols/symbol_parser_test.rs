//! These tests are for the dynamically generated `symbol_parser` module.
//!
#![cfg(test)]
#![allow(clippy::cognitive_complexity, non_fmt_panics)]

use crate::parser::symbols::symbol_parser::{Rule, SymbolParser};
use pest::{consumes_to, fails_with, parses_to, Parser};

#[test]
fn validate_prefixes() {
    parses_to! {
        parser: SymbolParser,
        input: "k",
        rule: Rule::pri_prefix,
        tokens: [pri_prefix(0, 1, [pri_kilo(0, 1)])]
    }

    parses_to! {
        parser: SymbolParser,
        input: "K",
        rule: Rule::sec_prefix,
        tokens: [sec_prefix(0, 1, [sec_kilo(0, 1)])]
    }

    fails_with! {
        parser: SymbolParser,
        input: "i",
        rule: Rule::pri_prefix,
        positives: vec![Rule::pri_prefix],
        negatives: vec![],
        pos: 0
    }

    parses_to! {
        parser: SymbolParser,
        input: "a",
        rule: Rule::pri_prefix,
        tokens: [pri_prefix(0, 1, [pri_atto(0, 1)])]
    }
}

#[test]
fn validate_atoms() {
    let pairs = SymbolParser::parse(Rule::pri_atom, "m");
    assert!(pairs.is_ok());

    let pairs = SymbolParser::parse(Rule::sec_atom, "M");
    assert!(pairs.is_ok());

    parses_to! {
        parser: SymbolParser,
        input: "K",
        rule: Rule::pri_atom,
        tokens: [pri_atom(0, 1, [pri_kelvin(0, 1)])]
    }

    parses_to! {
        parser: SymbolParser,
        input: "10*",
        rule: Rule::pri_atom,
        tokens: [pri_atom(0, 3, [pri_the_number_ten_for_arbitrary_powers_star(0, 3)])]
    }
}

#[test]
fn parse_yotta() {
    parses_to! {
        parser: SymbolParser,
        input: "Y",
        rule: Rule::pri_yotta,
        tokens: [
            pri_yotta(0, 1)
        ]
    }

    parses_to! {
        parser: SymbolParser,
        input: "Y",
        rule: Rule::pri_prefix,
        tokens: [
            pri_prefix(0, 1, [pri_yotta(0, 1)])
        ]
    }

    parses_to! {
        parser: SymbolParser,
        input: "YA",
        rule: Rule::sec_yotta,
        tokens: [
            sec_yotta(0, 2)
        ]
    }

    parses_to! {
        parser: SymbolParser,
        input: "YA",
        rule: Rule::sec_prefix,
        tokens: [
            sec_prefix(0, 2, [sec_yotta(0, 2)])
        ]
    }
}

#[test]
fn parse_thermochemical_calorie() {
    parses_to! {
        parser: SymbolParser,
        input: "cal_th",
        rule: Rule::pri_thermochemical_calorie,
        tokens: [
            pri_thermochemical_calorie(0, 6)
        ]
    }

    parses_to! {
        parser: SymbolParser,
        input: "cal_th",
        rule: Rule::pri_atom,
        tokens: [
            pri_atom(0, 6, [pri_thermochemical_calorie(0, 6)])
        ]
    }

    parses_to! {
        parser: SymbolParser,
        input: "CAL_TH",
        rule: Rule::sec_thermochemical_calorie,
        tokens: [
            sec_thermochemical_calorie(0, 6)
        ]
    }

    parses_to! {
        parser: SymbolParser,
        input: "CAL_TH",
        rule: Rule::sec_atom,
        tokens: [
            sec_atom(0, 6, [sec_thermochemical_calorie(0, 6)])
        ]
    }
}

// The Atom "cal" is the primary symbol for "Calorie", but the "ca" in "cal"
// can also match "c" => "centi" and "a" => "year", depending on the
// grammar. This test makes sure "cal" actually matches the "calorie" Atom.
#[test]
fn valid_atom_with_possible_matching_prefix_and_atom() {
    parses_to! {
        parser: SymbolParser,
        input: "cal",
        rule: Rule::symbol,
        tokens: [symbol(0, 3, [pri_atom(0, 3, [pri_calorie(0, 3)]), EOI(3, 3)])]
    }

    parses_to! {
        parser: SymbolParser,
        input: "CAL",
        rule: Rule::symbol,
        tokens: [symbol(0, 3, [sec_atom(0, 3, [sec_calorie(0, 3)]), EOI(3, 3)])]
    }
}

#[test]
fn valid_prefix_and_atom_with_possible_matching_atoms() {
    parses_to! {
        parser: SymbolParser,
        input: "dm",
        rule: Rule::symbol,
        tokens: [
            symbol(0, 2, [
                   pri_prefix(0, 1, [pri_deci(0, 1)]),
                   pri_atom(1, 2, [pri_meter(1, 2)]),
                   EOI(2, 2)
            ])
        ]
    }

    parses_to! {
        parser: SymbolParser,
        input: "DM",
        rule: Rule::symbol,
        tokens: [
            symbol(0, 2, [
                   sec_prefix(0, 1, [sec_deci(0, 1)]),
                   sec_atom(1, 2, [sec_meter(1, 2)]),
                   EOI(2, 2)
            ])
        ]
    }
}
