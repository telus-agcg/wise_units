// Internal structs for mapping parser Rule data to an intermediate
// representation of a Unit.
pub(self) mod annotatable;
pub(self) mod ast_term;
pub(self) mod basic_component;
pub(self) mod component;
pub(self) mod main_term;
pub(self) mod simple_unit;

use self::annotatable::Annotatable;
use self::ast_term::AstTerm;
use self::basic_component::BasicComponent;
use self::component::Component;
use self::main_term::MainTerm;
use self::simple_unit::SimpleUnit;
use error::Error;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use symbols::mapper;
use symbols::symbol_parser::Rule as SymbolRule;
use symbols::symbol_parser::SymbolParser;
use term::Term;
use terms::term_parser::Rule;

pub fn map(mut pairs: Pairs<Rule>) -> Result<Vec<Term>, Error> {
    fn visit_pairs(pair: Pair<Rule>) -> Result<Vec<Term>, Error> {
        let main_term = match pair.as_rule() {
            Rule::main_term => visit_main_term(pair)?,
            _ => {
                let e = Error::ParsingError {
                    expression: pair.as_str().to_string(),
                };
                return Err(e);
            }
        };

        Ok(main_term.into())
    }

    match pairs.next() {
        Some(pair) => {
            let terms = visit_pairs(pair)?;
            Ok(terms)
        }
        None => Ok(vec![]),
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
                        let error = Error::ParsingError {
                            expression: string.to_string(),
                        };
                        return Err(error);
                    }
                }
            }
            Rule::digits => {
                e *= visit_digits(inner_pair)?;
            }
            _ => {
                let error = Error::ParsingError {
                    expression: inner_pair.as_str().to_string(),
                };
                return Err(error);
            }
        }
    }

    Ok(e)
}

fn visit_simple_unit(pair: Pair<Rule>) -> Result<SimpleUnit, Error> {
    let mut simple_unit = SimpleUnit::new();
    let span = pair.into_span();
    let string = span.as_str();

    if string == "1" {
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
            // if let Some(atom) =  CUSTOM_UNITS.get(string) {
            //     let a = *atom;
            //     return a.clone()
            // }

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
                let error = Error::ParsingError {
                    expression: inner_pair.as_str().to_string(),
                };
                return Err(error);
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
                let error = Error::ParsingError {
                    expression: inner_pair.as_str().to_string(),
                };
                return Err(error);
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
                let error = Error::ParsingError {
                    expression: inner_pair.as_str().to_string(),
                };
                return Err(error);
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
                let error = Error::ParsingError {
                    expression: inner_pair.as_str().to_string(),
                };
                return Err(error);
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
                let error = Error::ParsingError {
                    expression: inner_pair.as_str().to_string(),
                };
                return Err(error);
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
    use prefix::Prefix;
    use term::Term;
    use terms::term_parser::{Rule, TermParser};

    macro_rules! validate_interpret {
        ($test_name:ident, $input:expr, $($terms:expr),+) => {
            #[test]
            fn $test_name() {
                let pairs = TermParser::parse(Rule::main_term, $input).unwrap();
                let actual = map(pairs).unwrap();
                let expected = vec![$($terms),+];

                assert_eq!(actual, expected);
            }
        };
    }

    #[test]
    fn validate_exponent() {
        let pairs = TermParser::parse(Rule::main_term, "m-3").unwrap();
        let actual = map(pairs).unwrap();

        let expected_term = term!(Meter, exponent: -3);
        let expected = vec![expected_term];

        assert_eq!(actual, expected);

        let pairs = TermParser::parse(Rule::main_term, "km2/m-3").unwrap();
        let actual = map(pairs).unwrap();

        let term1 = term!(Kilo, Meter, exponent: 2);
        let term2 = term!(Meter, exponent: 3);
        let expected = vec![term1, term2];

        assert_eq!(actual, expected);
    }

    validate_interpret!(validate_interpret_meter, "m", term!(Meter));
    validate_interpret!(
        validate_interpret_meter_positive_exponent,
        "m2",
        term!(Meter, exponent: 2)
    );
    validate_interpret!(
        validate_interpret_meter_negative_exponent,
        "m-2",
        term!(Meter, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_meter_factor,
        "2m",
        term!(Meter, factor: 2)
    );
    validate_interpret!(validate_interpret_kilometer, "km", term!(Kilo, Meter));

    // Slash terms
    validate_interpret!(
        validate_interpret_meter_per_second,
        "m/s",
        term!(Meter),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_kilometer_per_second,
        "km/s",
        term!(Kilo, Meter),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter_per_kilosecond,
        "m/ks",
        term!(Meter),
        term!(Kilo, Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter2_per_second,
        "m2/s",
        term!(Meter, exponent: 2),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter_per_second2,
        "m/s2",
        term!(Meter),
        term!(Second, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_meter2_per_second2,
        "m2/s2",
        term!(Meter, exponent: 2),
        term!(Second, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_2meter_per_second,
        "2m/s",
        term!(Meter, factor: 2),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter_per_2second,
        "m/2s",
        term!(Meter),
        term!(Second, exponent: -1, factor: 2)
    );
    validate_interpret!(
        validate_interpret_2meter2_per_2second2,
        "2m2/2s2",
        term!(Meter, factor: 2, exponent: 2),
        term!(Second, factor: 2, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_foot_per_factor,
        "[ft_i]/12",
        term!(FootInternational),
        term!(factor: 12, exponent: -1)
    );

    // Dot terms
    validate_interpret!(
        validate_interpret_meter_second,
        "m.s",
        term!(Meter),
        term!(Second)
    );
    validate_interpret!(
        validate_interpret_meter2_second,
        "m2.s",
        term!(Meter, exponent: 2),
        term!(Second)
    );
    validate_interpret!(
        validate_interpret_meter_second2,
        "m.s2",
        term!(Meter),
        term!(Second, exponent: 2)
    );
    validate_interpret!(
        validate_interpret_meter2_second2,
        "m2.s2",
        term!(Meter, exponent: 2),
        term!(Second, exponent: 2)
    );
    validate_interpret!(
        validate_interpret_2meter_second,
        "2m.s",
        term!(Meter, factor: 2),
        term!(Second)
    );
    validate_interpret!(
        validate_interpret_meter_2second,
        "m.2s",
        term!(Meter),
        term!(Second, factor: 2)
    );
    validate_interpret!(
        validate_interpret_2meter_2second,
        "2m.2s",
        term!(Meter, factor: 2),
        term!(Second, factor: 2)
    );
    validate_interpret!(
        validate_interpret_2meter2_2second2,
        "2m2.2s2",
        term!(Meter, factor: 2, exponent: 2),
        term!(Second, factor: 2, exponent: 2)
    );

    // Dot and slash combined terms
    validate_interpret!(
        validate_interpret_acre_inch_per_acre,
        "[acr_us].[in_i]/[acr_us]",
        term!(AcreUS),
        term!(InchInternational),
        term!(AcreUS, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_2acre3_3inch4_per_4acre5,
        "2[acr_us]3.3[in_i]4/4[acr_us]5",
        term!(AcreUS, factor: 2, exponent: 3),
        term!(InchInternational, factor: 3, exponent: 4),
        term!(AcreUS, factor: 4, exponent: -5)
    );

    #[test]
    #[ignore]
    fn validate_custom_atom() {
        let pairs = TermParser::parse(Rule::main_term, "[meow]").unwrap();

        let actual = map(pairs).unwrap();
        let acre_term = term!(AcreUS);
        let inch_term = term!(InchInternational);
        let acre2_term = term!(AcreUS, exponent: -1);

        let expected = vec![acre_term, inch_term, acre2_term];

        assert_eq!(actual, expected);
    }
}
