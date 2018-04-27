// Internal structs for mapping parser Rule data to an intermediate
// representation of a Unit.
pub(self) mod annotatable;
pub(self) mod ast_term;
pub(self) mod basic_component;
pub(self) mod component;
pub(self) mod main_term;
pub(self) mod simple_unit;

use error::Error;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use self::annotatable::Annotatable;
use self::ast_term::AstTerm;
use self::basic_component::BasicComponent;
use self::component::Component;
use self::main_term::MainTerm;
use self::simple_unit::SimpleUnit;
use symbols::symbol_parser::SymbolParser;
use symbols::symbol_parser::Rule as SymbolRule;
use symbols::mapper;
use term::Term;
use terms::term_parser::Rule;

pub fn map(mut pairs: Pairs<Rule>) -> Result<Vec<Term>, Error> {
    fn visit_pairs(pair: Pair<Rule>) -> Result<Vec<Term>, Error> {
        let main_term = match pair.as_rule() {
            Rule::main_term => visit_main_term(pair)?,
            _ => {
                let e = Error::ParsingError { expression: pair.as_str().to_string() };
                return Err(e)
            }
        };

        Ok(main_term.into())
    }

    match pairs.next() {
        Some(pair) => {
            let terms = visit_pairs(pair)?;
            Ok(terms)
        }
        None => Ok(vec![])
    }
}

fn visit_digits(pair: Pair<Rule>) -> Result<i32, Error> {
    let span = pair.into_span();
    let string = span.as_str();

    string.parse::<i32>().map_err(|e| Error::ParsingError {
        expression: e.to_string(),
    })
}

fn visit_exponent(pair: Pair<Rule>) -> Result<i32, Error> {
    let mut e = 1;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::sign => {
                let span = inner_pair.into_span();
                let string = span.as_str();

                match string {
                    "+" => {}
                    "-" => {
                        e = -e;
                    }
                    _ => {
                        let error = Error::ParsingError { expression: string.to_string() };
                        return Err(error)
                    }
                }
            }
            Rule::digits => {
                e *= visit_digits(inner_pair)?;
            }
            _ => {
                let error = Error::ParsingError { expression: inner_pair.as_str().to_string() };
                return Err(error)
            }
        }
    }

    Ok(e)
}

fn visit_simple_unit(pair: Pair<Rule>) -> Result<SimpleUnit, Error> {
    let mut simple_unit = SimpleUnit::new();
    let span = pair.into_span();
    let string = span.as_str();

    if string.is_empty() {
        return Ok(simple_unit);
    }

    match SymbolParser::parse(SymbolRule::symbol, string) {
        Ok(mut symbol_pairs) => {
            let symbol = mapper::map(symbol_pairs.next().unwrap())?;
            simple_unit.atom = symbol.atom;
            simple_unit.prefix = symbol.prefix;
        }
        Err(e) => {
            // Try 3rd party lookup
            println!("MEOW: {:#?}", &e);
            return Err(Error::ParsingError {
                expression: string.to_string(),
            });
        }
    };

    Ok(simple_unit)
}

fn visit_annotatable(pair: Pair<Rule>) -> Result<Annotatable, Error> {
    let mut annotatable = Annotatable::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::simple_unit => {
                let simple_unit = visit_simple_unit(inner_pair)?;

                annotatable.atom = simple_unit.atom;
                annotatable.prefix = simple_unit.prefix;
            }
            Rule::exponent => {
                annotatable.exponent = visit_exponent(inner_pair)?;
            }
            _ => {
                let error = Error::ParsingError { expression: inner_pair.as_str().to_string() };
                return Err(error)
            }
        }
    }

    Ok(annotatable)
}

fn visit_annotation(pair: Pair<Rule>) -> Result<String, Error> {
    let string = pair.into_span().as_str().to_string();

    Ok(string)
}

fn visit_factor(pair: Pair<Rule>) -> Result<u32, Error> {
    let span = pair.into_span();
    let string = span.as_str();

    string.parse::<u32>().map_err(|e| Error::ParsingError {
        expression: e.to_string(),
    })
}

fn visit_basic_component(pair: Pair<Rule>) -> Result<BasicComponent, Error> {
    let mut bc = BasicComponent::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::annotatable => {
                let annotatable = visit_annotatable(inner_pair)?;

                bc.prefix = annotatable.prefix;
                bc.atom = annotatable.atom;
                bc.exponent = annotatable.exponent;
            }
            Rule::annotation => {
                let annotation = visit_annotation(inner_pair)?;

                bc.annotation = Some(annotation);
            }
            Rule::factor => {
                bc.factor = visit_factor(inner_pair)?;
            }
            Rule::term => {
                let ast_term = visit_term(inner_pair)?;

                bc.terms.append(&mut ast_term.into());
            }
            _ => {
                let error = Error::ParsingError { expression: inner_pair.as_str().to_string() };
                return Err(error)
            }
        }
    }

    Ok(bc)
}

fn visit_component(pair: Pair<Rule>) -> Result<Component, Error> {
    let mut component = Component::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::factor => {
                component.factor = Some(visit_factor(inner_pair)?);
            }
            Rule::basic_component => {
                let bc = visit_basic_component(inner_pair)?;

                component.terms.append(&mut bc.into());
            }
            _ => {
                let error = Error::ParsingError { expression: inner_pair.as_str().to_string() };
                return Err(error)
            }
        }
    }

    Ok(component)
}

fn visit_term(pair: Pair<Rule>) -> Result<AstTerm, Error> {
    let mut has_slash = false;
    let mut ast_term = AstTerm::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::term => {
                let child_ast_term = visit_term(inner_pair)?;
                let mut new_terms: Vec<Term> = child_ast_term.into();

                if has_slash {
                    for new_term in &mut new_terms {
                        new_term.exponent = -new_term.exponent;
                    }
                    has_slash = false;
                }

                ast_term.terms.append(&mut new_terms);
            }
            Rule::slash => {
                has_slash = true;
            }
            Rule::component => {
                let component = visit_component(inner_pair)?;

                ast_term.component = Some(component);
            }
            _ => {
                let error = Error::ParsingError { expression: inner_pair.as_str().to_string() };
                return Err(error)
            }
        }
    }

    Ok(ast_term)
}

fn visit_main_term(pair: Pair<Rule>) -> Result<MainTerm, Error> {
    let mut main_term = MainTerm::new();
    let mut has_slash = false;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::slash => {
                has_slash = true;
            }
            Rule::term => {
                let mut ast_term = visit_term(inner_pair)?;
                let mut new_terms: Vec<Term> = ast_term.into();

                if has_slash {
                    for new_term in &mut new_terms {
                        new_term.exponent = -new_term.exponent;
                    }
                    has_slash = false;
                }

                main_term.terms.append(&mut new_terms);
            }
            _ => {
                let error = Error::ParsingError { expression: inner_pair.as_str().to_string() };
                return Err(error)
            }
        }
    }

    Ok(main_term)
}

#[cfg(test)]
mod tests {
    use super::*;
    use atom::Atom;
    use pest::Parser;
    use term::Term;
    use terms::term_parser::{Rule, TermParser};
    use prefix::Prefix;

    #[test]
    fn validate_exponent() {
        let pairs = TermParser::parse(Rule::main_term, "m-3").unwrap();
        let actual = map(pairs).unwrap();

        let mut expected_term = Term::new(Some(Atom::Meter), None);
        expected_term.exponent = -3;

        let expected = vec![expected_term];

        assert_eq!(actual, expected);

        let pairs = TermParser::parse(Rule::main_term, "km2/m-3").unwrap();
        let actual = map(pairs).unwrap();

        let mut term1 = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        term1.exponent = 2;

        let mut term2 = Term::new(Some(Atom::Meter), None);
        term2.exponent = 3;

        let expected = vec![term1, term2];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_component_with_exponent() {
        let pairs = TermParser::parse(Rule::main_term, "m2").unwrap();

        let actual = map(pairs).unwrap();
        let mut expected_term = Term::new(Some(Atom::Meter), None);
        expected_term.exponent = 2;

        let expected = vec![expected_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_component_with_prefix() {
        let pairs = TermParser::parse(Rule::main_term, "km").unwrap();

        let actual = map(pairs).unwrap();

        let expected = vec![Term::new(Some(Atom::Meter), Some(Prefix::Kilo))];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_component_with_factor() {
        let pairs = TermParser::parse(Rule::main_term, "2m").unwrap();

        let actual = map(pairs).unwrap();

        let mut expected_term = Term::new(Some(Atom::Meter), None);
        expected_term.factor = 2;

        let expected = vec![expected_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term() {
        let pairs = TermParser::parse(Rule::main_term, "m/s").unwrap();

        let actual = map(pairs).unwrap();
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;

        let expected = vec![meter_term, second_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_numerator_prefix() {
        let pairs = TermParser::parse(Rule::main_term, "km/s").unwrap();

        let actual = map(pairs).unwrap();
        let expected_numerator_term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        let mut expected_denominator_term = Term::new(Some(Atom::Second), None);
        expected_denominator_term.exponent = -1;

        let expected = vec![expected_numerator_term, expected_denominator_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_denominator_prefix() {
        let pairs = TermParser::parse(Rule::main_term, "m/ks").unwrap();

        let actual = map(pairs).unwrap();
        let expected_numerator_term = Term::new(Some(Atom::Meter), None);
        let mut expected_denominator_term = Term::new(Some(Atom::Second), Some(Prefix::Kilo));
        expected_denominator_term.exponent = -1;

        let expected = vec![expected_numerator_term, expected_denominator_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_numerator_exponent() {
        let pairs = TermParser::parse(Rule::main_term, "m2/s").unwrap();

        let actual = map(pairs).unwrap();
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;

        let expected = vec![meter_term, second_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_denominator_exponent() {
        let pairs = TermParser::parse(Rule::main_term, "m/s2").unwrap();

        let actual = map(pairs).unwrap();
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -2;

        let expected = vec![meter_term, second_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_numerator_and_denominator_exponent() {
        let pairs = TermParser::parse(Rule::main_term, "m2/s2").unwrap();

        let actual = map(pairs).unwrap();
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -2;

        let expected = vec![meter_term, second_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_factor_in_numerator() {
        let pairs = TermParser::parse(Rule::main_term, "2m/s").unwrap();

        let actual = map(pairs).unwrap();
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.factor = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;

        let expected = vec![meter_term, second_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_factor_in_denominator() {
        let pairs = TermParser::parse(Rule::main_term, "m/2s").unwrap();

        let actual = map(pairs).unwrap();
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;
        second_term.factor = 2;

        let expected = vec![meter_term, second_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_only_factor_in_denominator() {
        let pairs = TermParser::parse(Rule::main_term, "[ft_i]/12").unwrap();

        let actual = map(pairs).unwrap();
        let meter_term = Term::new(Some(Atom::FootInternational), None);
        let mut second_term = Term::new(None, None);
        second_term.exponent = -1;
        second_term.factor = 12;

        let expected = vec![meter_term, second_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term() {
        let pairs = TermParser::parse(Rule::main_term, "m.s").unwrap();

        let actual = map(pairs).unwrap();
        let meter_term = Term::new(Some(Atom::Meter), None);
        let second_term = Term::new(Some(Atom::Second), None);

        let expected = vec![meter_term, second_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_left_side_exponent() {
        let pairs = TermParser::parse(Rule::main_term, "m2.s").unwrap();

        let actual = map(pairs).unwrap();
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let second_term = Term::new(Some(Atom::Second), None);

        let expected = vec![meter_term, second_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_right_side_exponent() {
        let pairs = TermParser::parse(Rule::main_term, "m.s2").unwrap();

        let actual = map(pairs).unwrap();
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = 2;

        let expected = vec![meter_term, second_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_left_and_right_side_exponent() {
        let pairs = TermParser::parse(Rule::main_term, "m2.s2").unwrap();

        let actual = map(pairs).unwrap();
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = 2;

        let expected = vec![meter_term, second_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_factor_in_left_side() {
        let pairs = TermParser::parse(Rule::main_term, "2m.s").unwrap();

        let actual = map(pairs).unwrap();
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.factor = 2;
        let second_term = Term::new(Some(Atom::Second), None);

        let expected = vec![meter_term, second_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_factor_in_right_side() {
        let pairs = TermParser::parse(Rule::main_term, "m.2s").unwrap();

        let actual = map(pairs).unwrap();
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.factor = 2;

        let expected = vec![meter_term, second_term];

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_term_with_dot_term_then_slash_component() {
        let pairs = TermParser::parse(Rule::main_term, "[acr_us].[in_i]/[acr_us]").unwrap();

        let actual = map(pairs).unwrap();
        let acre_term = Term::new(Some(Atom::AcreUS), None);
        let inch_term = Term::new(Some(Atom::InchInternational), None);
        let mut acre2_term = Term::new(Some(Atom::AcreUS), None);
        acre2_term.exponent = -1;

        let expected = vec![acre_term, inch_term, acre2_term];

        assert_eq!(actual, expected);
    }
}
