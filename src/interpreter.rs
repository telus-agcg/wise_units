use atom::Atom;
use pest::Parser;
use pest::inputs::Input;
use pest::iterators::{Pair, Pairs};
use prefix::Prefix;
use unit_parser::{Rule, UnitParser};
use term::Term;
use unit::Unit;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret<'a>(&mut self, expression: &'a str) -> Unit {
        let pairs = UnitParser::parse_str(Rule::term, expression)
            .unwrap_or_else(|e| {
                println!("Parsing error: {}", e);
                panic!("Unable to parse \"{}\"", expression);
            });
        let terms = self.visit_with_pairs(pairs);

        Unit {
            expression: expression.to_string(),
            terms: terms
        }
    }

    fn visit_with_pairs<I: Input>(&mut self, pairs: Pairs<Rule, I>) -> Vec<Term> {
        let mut terms: Vec<Term> = vec![];

        for pair in pairs {
            for inner_pair in pair.into_inner() {
                let mut other_terms = match inner_pair.as_rule() {
                    Rule::term => self.visit_with_pairs(inner_pair.into_inner()),
                    Rule::dot_term => self.visit_term(inner_pair),
                    Rule::slash_term => self.visit_term(inner_pair),
                    Rule::basic_term => self.visit_term(inner_pair),
                    Rule::component => self.visit_component(inner_pair),
                    Rule::annotatable => {
                        let (prefix, atom, exponent) = self.visit_annotatable(inner_pair);
                        let mut term = Term::new(atom, prefix);
                        term.exponent = exponent;
                        vec![term]
                    },
                    _ => {
                        println!("visit_with_pairs: unreachable rule: {:?}", inner_pair);
                        unreachable!()
                    },
                };

                terms.append(&mut other_terms);
            }
        }

        terms
    }

    fn visit_atom_symbol<I: Input>(&mut self, pair: Pair<Rule, I>) -> Atom {
        let mut atom = Atom::Meter;

        for inner_pair in pair.into_inner() {
            atom = match inner_pair.as_rule() {
                // Base units first.
                Rule::candela => Atom::Candela,
                Rule::coulomb => Atom::Coulomb,
                Rule::gram => Atom::Gram,
                Rule::kelvin => Atom::Kelvin,
                Rule::meter => Atom::Meter,
                Rule::radian => Atom::Radian,
                Rule::second => Atom::Second,

                // Derived units last.
                Rule::acre_us => Atom::AcreUS,
                Rule::are => Atom::Are,
                Rule::degree_celsius => Atom::DegreeCelsius,
                Rule::degree_fahrenheit => Atom::DegreeFahrenheit,
                Rule::degree_reaumur => Atom::DegreeReaumur,
                Rule::degree => Atom::Degree,
                Rule::fluid_ounce_us => Atom::FluidOunceUS,
                Rule::foot_international => Atom::FootInternational,
                Rule::foot_us => Atom::FootUS,
                Rule::gill_us => Atom::GillUS,
                Rule::inch_international => Atom::InchInternational,
                Rule::liter => Atom::Liter,
                Rule::mole => Atom::Mole,
                Rule::parts_per_billion => Atom::PartsPerBillion,
                Rule::parts_per_million => Atom::PartsPerMillion,
                Rule::parts_per_thousand => Atom::PartsPerThousand,
                Rule::percent => Atom::Percent,
                Rule::ph => Atom::PH,
                Rule::pint_us => Atom::PintUS,
                Rule::prism_diopter => Atom::PrismDiopter,
                Rule::quart_us => Atom::QuartUS,
                Rule::queen_annes_wine_gallon => Atom::QueenAnnesWineGallon,
                Rule::rod_us => Atom::RodUS,
                Rule::the_number_pi => Atom::TheNumberPi,
                Rule::the_number_ten_for_arbitrary_powers_caret => Atom::TheNumberTenForArbitraryPowersCaret,
                Rule::the_number_ten_for_arbitrary_powers_star => Atom::TheNumberTenForArbitraryPowersStar,

                _ => {
                    println!("visit_atom_symbol: unreachable rule: {:?}", inner_pair);
                    unreachable!()
                },
            };
        }

        atom
    }

    fn visit_prefix_symbol<I: Input>(&mut self, pair: Pair<Rule, I>) -> Prefix {
        let mut prefix = Prefix::Mega;

        for inner_pair in pair.into_inner() {
            prefix = match inner_pair.as_rule() {
                Rule::atto => Prefix::Atto,
                Rule::centi => Prefix::Centi,
                Rule::deci => Prefix::Deci,
                Rule::deka => Prefix::Deka,
                Rule::exa => Prefix::Exa,
                Rule::femto => Prefix::Femto,
                Rule::gibi => Prefix::Gibi,
                Rule::giga => Prefix::Giga,
                Rule::hecto => Prefix::Hecto,
                Rule::kilo => Prefix::Kilo,
                Rule::mebi => Prefix::Mebi,
                Rule::mega => Prefix::Mega,
                Rule::micro => Prefix::Micro,
                Rule::milli => Prefix::Milli,
                Rule::nano => Prefix::Nano,
                Rule::peta => Prefix::Peta,
                Rule::tebi => Prefix::Tebi,
                Rule::tera => Prefix::Tera,
                Rule::yocto => Prefix::Yocto,
                Rule::yotta => Prefix::Yotta,
                Rule::zepto => Prefix::Zepto,
                Rule::zetta => Prefix::Zetta,
                _ => unreachable!(),
            };
        }

        prefix
    }

    fn visit_prefixed_atom<I: Input>(&mut self, pair: Pair<Rule, I>) -> (Option<Prefix>, Option<Atom>) {
        let mut prefixed_atom = (None, None);

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::prefix_symbol => {
                    prefixed_atom.0 = Some(self.visit_prefix_symbol(inner_pair));
                },
                Rule::atom_symbol => {
                    prefixed_atom.1 = Some(self.visit_atom_symbol(inner_pair));
                },
                _ => unreachable!()
            }
        }

        prefixed_atom
    }

    fn visit_digits<I: Input>(&mut self, pair: Pair<Rule, I>) -> i32 {
        let span = pair.into_span();
        let string = span.as_str();

        string.parse::<i32>().unwrap_or_else(|e| panic!("{}", e))
    }

    fn visit_exponent<I: Input>(&mut self, pair: Pair<Rule, I>) -> i32 {
        let mut e = 1;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::sign => {
                    let span = inner_pair.into_span();
                    let string = span.as_str();

                    match string {
                        "+" => {},
                        "-" => { e = -e; },
                        _ => unreachable!()
                    }
                },
                Rule::digits => {
                    e *= self.visit_digits(inner_pair);
                },
                _ => unreachable!()
            }
        }

        e
    }

    fn visit_simple_unit<I: Input>(&mut self, pair: Pair<Rule, I>) -> (Option<Prefix>, Option<Atom>) {
        let mut simple_unit = (None, None);

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::prefixed_atom => {
                    simple_unit = self.visit_prefixed_atom(inner_pair);
                },
                Rule::atom_symbol => {
                    simple_unit.1 = Some(self.visit_atom_symbol(inner_pair));
                },
                _ => unreachable!(),
            }
        }

        simple_unit
    }

    fn visit_simple_unit_with_exponent<I: Input>(&mut self, pair: Pair<Rule, I>) -> (Option<Prefix>, Option<Atom>, i32) {
        let mut simple_unit_with_exponent = (None, None, 1);

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::simple_unit => {
                    let (prefix, atom) = self.visit_simple_unit(inner_pair);
                    simple_unit_with_exponent.0 = prefix;
                    simple_unit_with_exponent.1 = atom;
                },
                Rule::exponent => {
                    simple_unit_with_exponent.2 = self.visit_exponent(inner_pair);
                },
                _ => unreachable!()
            }
        }

        simple_unit_with_exponent
    }

    // TODO
    // fn visit_special_unit(&mut self, su: &SpecialUnit) -> Term {
    //     // Term::new(None, None)
    //     unimplemented!()
    // }

    fn visit_annotatable<I: Input>(&mut self, pair: Pair<Rule, I>) -> (Option<Prefix>, Option<Atom>, i32) {
        let mut annotatable = (None, None, 1);

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::simple_unit_with_exponent => {
                    let (prefix, atom, exponent) = self.visit_simple_unit_with_exponent(inner_pair);
                    annotatable.0 = prefix;
                    annotatable.1 = atom;
                    annotatable.2 = exponent;
                },
                Rule::simple_unit => {
                    let (prefix, atom) = self.visit_simple_unit(inner_pair);
                    annotatable.0 = prefix;
                    annotatable.1 = atom;
                },
                // Rule::special_unit => {}
                _ => unreachable!(),
            }
        }

        annotatable
    }

    fn visit_annotation_string<I: Input>(&mut self, pair: Pair<Rule, I>) -> (Option<String>) {
        let mut annotation_string = None;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::annotation_string => {
                    annotation_string = Some(inner_pair.into_span().as_str().to_string())
                },
                _ => unreachable!(),
            }
        }

        annotation_string
    }

    fn visit_annotation<I: Input>(&mut self, pair: Pair<Rule, I>) -> (Option<String>) {
        let mut annotation = None;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::annotation_string => {
                    annotation = self.visit_annotation_string(inner_pair);
                },
                _ => unreachable!(),
            }
        }

        annotation
    }

    fn visit_annotated_annotatable<I: Input>(&mut self, pair: Pair<Rule, I>) -> (Option<Prefix>, Option<Atom>, i32, Option<String>) {
        let mut annotated_annotatable = (None, None, 1, None);

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::annotatable => {
                    let (prefix, atom, exponent) = self.visit_annotatable(inner_pair);
                    annotated_annotatable.0 = prefix;
                    annotated_annotatable.1 = atom;
                    annotated_annotatable.2 = exponent;
                },
                Rule::annotation => {
                    annotated_annotatable.3 = self.visit_annotation(inner_pair);
                },
                _ => unreachable!(),
            }
        }

        annotated_annotatable
    }

    fn visit_factor<I: Input>(&mut self, pair: Pair<Rule, I>) -> u32 {
        let mut factor = 0;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::digits => {
                    factor = self.visit_digits(inner_pair);
                },
                _ => unreachable!()
            }
        }

        factor as u32
    }

    fn visit_basic_component<I: Input>(&mut self, pair: Pair<Rule, I>) -> Vec<Term> {
        let mut terms: Vec<Term> = vec![];
        let mut term = Term::new(None, None);
        let mut is_term = false;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::annotated_annotatable => {
                    let (prefix, atom, exponent, annotation) = self.visit_annotated_annotatable(inner_pair);
                    term.prefix = prefix;
                    term.atom = atom;
                    term.exponent = exponent;
                    term.annotation = annotation;
                },
                Rule::annotatable => {
                    let (prefix, atom, exponent) = self.visit_annotatable(inner_pair);
                    term.prefix = prefix;
                    term.atom = atom;
                    term.exponent = exponent;
                },
                Rule::annotation => {
                    term.annotation = Some(inner_pair.into_span().as_str().to_string());
                },
                Rule::factor => {
                    term.factor = self.visit_factor(inner_pair);
                },
                Rule::term => {
                    is_term = true;
                    let new_terms = self.visit_term(inner_pair);
                    terms = new_terms;
                },
                _ => unreachable!()
            }
        }

        if !is_term { terms.push(term); }

        terms
    }

    fn visit_component_with_factor<I: Input>(&mut self, pair: Pair<Rule, I>) -> Vec<Term> {
        let mut terms: Vec<Term> = vec![];
        let mut factor = 1;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::factor => {
                    factor = self.visit_factor(inner_pair);
                },
                Rule::basic_component => {
                    let mut new_terms = self.visit_basic_component(inner_pair);
                    terms.append(&mut new_terms);
                },
                _ => unreachable!(),
            };
        }

        if let Some(first_term) = terms.first_mut() {
            first_term.factor = factor;
        }

        terms
    }

    fn visit_component<I: Input>(&mut self, pair: Pair<Rule, I>) -> Vec<Term> {
        let mut terms: Vec<Term> = vec![];

        for inner_pair in pair.into_inner() {
            let mut new_terms = match inner_pair.as_rule() {
                Rule::component_with_factor => self.visit_component_with_factor(inner_pair),
                Rule::basic_component => self.visit_basic_component(inner_pair),
                _ => unreachable!()
            };

            terms.append(&mut new_terms);
        }

        terms
    }

    fn visit_basic_term<I: Input>(&mut self, pair: Pair<Rule, I>) -> Vec<Term> {
        let mut terms: Vec<Term> = vec![];

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::component => {
                    let mut term = self.visit_component(inner_pair);
                    terms.append(&mut term);
                },
                _ => unreachable!()
            }
        }

        terms
    }

    fn visit_slash_term<I: Input>(&mut self, pair: Pair<Rule, I>) -> Vec<Term> {
        let mut terms: Vec<Term> = vec![];

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::component => {
                    let mut new_terms = self.visit_component(inner_pair);
                    terms.append(&mut new_terms);
                },
                Rule::term => {
                  let mut new_terms = self.visit_with_pairs(inner_pair.into_inner());

                  for new_term in &mut new_terms {
                    new_term.exponent = -new_term.exponent;
                    println!("visit_slash_term/term: {}", new_term.exponent);
                  }

                  terms.append(&mut new_terms);
                },
                _ => unreachable!()
            }
        }

        terms
    }

    fn visit_dot_term<I: Input>(&mut self, pair: Pair<Rule, I>) -> Vec<Term> {
        let mut terms: Vec<Term> = vec![];

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::component => {
                    let mut new_terms = self.visit_component(inner_pair);
                    terms.append(&mut new_terms);
                },
                Rule::term => {
                    let mut new_terms = self.visit_with_pairs(inner_pair.into_inner());
                    terms.append(&mut new_terms);
                },
                _ => unreachable!()
            }
        }

        terms
    }


    fn visit_term<I: Input>(&mut self, pair: Pair<Rule, I>) -> Vec<Term> {
        match pair.as_rule() {
            Rule::dot_term => self.visit_dot_term(pair),
            Rule::slash_term => self.visit_slash_term(pair),
            Rule::basic_term => self.visit_basic_term(pair),
            _ => {
                println!("visit_term: unreachable rule: {:?}", pair);
                unreachable!()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atom::Atom;
    use unit::Unit;

    #[test]
    fn validate_exponent() {
        let mut i = Interpreter;
        let actual = i.interpret("m-3");

        let mut expected_term = Term::new(Some(Atom::Meter), None);
        expected_term.exponent = -3;

        let expected = Unit {
            expression: "m-3".to_string(),
            terms: vec![expected_term]
        };

        assert_eq!(actual, expected);

        let actual = i.interpret("km2/m-3");

        let mut term1 = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        term1.exponent = 2;

        let mut term2 = Term::new(Some(Atom::Meter), None);
        term2.exponent = 3;

        let expected = Unit {
            expression: "km2/m-3".to_string(),
            terms: vec![term1, term2]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_basic_term() {
        let mut i = Interpreter;
        let actual = i.interpret("m");
        let expected = Unit {
            expression: "m".to_string(),
            terms: vec![Term::new(Some(Atom::Meter), None)]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_basic_term_with_exponent() {
        let mut i = Interpreter;
        let actual = i.interpret("m2");
        let mut expected_term = Term::new(Some(Atom::Meter), None);
        expected_term.exponent = 2;

        let expected = Unit {
            expression: "m2".to_string(),
            terms: vec![expected_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_basic_term_with_prefix() {
        let mut i = Interpreter;
        let actual = i.interpret("km");

        let expected = Unit {
            expression: "km".to_string(),
            terms: vec![Term::new(Some(Atom::Meter), Some(Prefix::Kilo))]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_basic_term_with_factor() {
        let mut i = Interpreter;
        let actual = i.interpret("2m");

        let mut expected_term = Term::new(Some(Atom::Meter), None);
        expected_term.factor = 2;

        let expected = Unit {
            expression: "2m".to_string(),
            terms: vec![expected_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term() {
        let mut i = Interpreter;
        let actual = i.interpret("m/s");
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;


        let expected = Unit {
            expression: "m/s".to_string(),
            terms: vec![meter_term, second_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_numerator_prefix() {
        let mut i = Interpreter;
        let actual = i.interpret("km/s");
        let expected_numerator_term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        let mut expected_denominator_term = Term::new(Some(Atom::Second), None);
        expected_denominator_term.exponent = -1;

        let expected = Unit {
            expression: "km/s".to_string(),
            terms: vec![expected_numerator_term, expected_denominator_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_denominator_prefix() {
        let mut i = Interpreter;
        let actual = i.interpret("m/ks");
        let expected_numerator_term = Term::new(Some(Atom::Meter), None);
        let mut expected_denominator_term = Term::new(Some(Atom::Second), Some(Prefix::Kilo));
        expected_denominator_term.exponent = -1;

        let expected = Unit {
            expression: "m/ks".to_string(),
            terms: vec![expected_numerator_term, expected_denominator_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_numerator_exponent() {
        let mut i = Interpreter;
        let actual = i.interpret("m2/s");
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;

        let expected = Unit {
            expression: "m2/s".to_string(),
            terms: vec![meter_term, second_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_denominator_exponent() {
        let mut i = Interpreter;
        let actual = i.interpret("m/s2");
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -2;

        let expected = Unit {
            expression: "m/s2".to_string(),
            terms: vec![meter_term, second_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_numerator_and_denominator_exponent() {
        let mut i = Interpreter;
        let actual = i.interpret("m2/s2");
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -2;

        let expected = Unit {
            expression: "m2/s2".to_string(),
            terms: vec![meter_term, second_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_factor_in_numerator() {
        let mut i = Interpreter;
        let actual = i.interpret("2m/s");
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.factor = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;

        let expected = Unit {
            expression: "2m/s".to_string(),
            terms: vec![meter_term, second_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_factor_in_denominator() {
        let mut i = Interpreter;
        let actual = i.interpret("m/2s");
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;
        second_term.factor = 2;

        let expected = Unit {
            expression: "m/2s".to_string(),
            terms: vec![meter_term, second_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term() {
        let mut i = Interpreter;
        let actual = i.interpret("m.s");
        let meter_term = Term::new(Some(Atom::Meter), None);
        let second_term = Term::new(Some(Atom::Second), None);

        let expected = Unit {
            expression: "m.s".to_string(),
            terms: vec![meter_term, second_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_left_side_exponent() {
        let mut i = Interpreter;
        let actual = i.interpret("m2.s");
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let second_term = Term::new(Some(Atom::Second), None);

        let expected = Unit {
            expression: "m2.s".to_string(),
            terms: vec![meter_term, second_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_right_side_exponent() {
        let mut i = Interpreter;
        let actual = i.interpret("m.s2");
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = 2;

        let expected = Unit {
            expression: "m.s2".to_string(),
            terms: vec![meter_term, second_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_left_and_right_side_exponent() {
        let mut i = Interpreter;
        let actual = i.interpret("m2.s2");
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = 2;


        let expected = Unit {
            expression: "m2.s2".to_string(),
            terms: vec![meter_term, second_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_factor_in_left_side() {
        let mut i = Interpreter;
        let actual = i.interpret("2m.s");
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.factor = 2;
        let second_term = Term::new(Some(Atom::Second), None);

        let expected = Unit {
            expression: "2m.s".to_string(),
            terms: vec![meter_term, second_term]
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_factor_in_right_side() {
        let mut i = Interpreter;
        let actual = i.interpret("m.2s");
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.factor = 2;

        let expected = Unit {
            expression: "m.2s".to_string(),
            terms: vec![meter_term, second_term]
        };

        assert_eq!(actual, expected);
    }
}
