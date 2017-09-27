use atom::Atom;
use pest::inputs::Input;
use pest::iterators::{Pair, Pairs};
use prefix::Prefix;
use term::Term;
use unit::Unit;
use unit_parser::Rule;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret<I: Input>(&mut self, pairs: Pairs<Rule, I>) -> Unit {
        let mut terms: Vec<Term> = vec![];

        self.visit_with_pairs(pairs, &mut terms);

        Unit { terms: terms }
    }

    fn visit_with_pairs<I: Input>(&mut self, pairs: Pairs<Rule, I>, terms: &mut Vec<Term>) {
        for pair in pairs {
            match pair.as_rule() {
                Rule::main_term => self.visit_main_term(pair, terms),
                _ => {
                    println!("visit_with_pairs: unreachable rule: {:?}", pair);
                    unreachable!()
                }
            };
        }
    }

    fn visit_atom_symbol<I: Input>(&mut self, pair: Pair<Rule, I>, term: &mut Term) {
        let atom = match pair.into_span().as_str() {
            // Base units first.
            "1"                       => Atom::TheUnity,
            "Coulomb" | "C"           => Atom::Coulomb,
            "Kelvin"  | "K"           => Atom::Kelvin,
            "candela" | "cd"  | "CD"  => Atom::Candela,
            "gram"    | "g"           => Atom::Gram,
            "meter"   | "m"   | "M"   => Atom::Meter,
            "radian"  | "rad" | "RAD" => Atom::Radian,
            "second"  | "s"   | "S"   => Atom::Second,

            // Derived units last.
            "%"                        | "percent"                           => Atom::Percent,
            "10*"                                                            => Atom::TheNumberTenForArbitraryPowersStar,
            "10^"                                                            => Atom::TheNumberTenForArbitraryPowersCaret,
            "Queen Anne's wine gallon" | "[gal_us]"   | "[GAL_US]"           => Atom::QueenAnnesWineGallon,
            "acre"                     | "[acr_us]"   | "[ACR_US]"           => Atom::AcreUS,
            "are"                      | "ar"         | "AR"       | "a"     => Atom::Are,
            "degree Celsius"           | "Cel"        | "CEL"      | "°C"    => Atom::DegreeCelsius,
            "degree Fahrenheit"        | "[degF]"     | "[DEGF]"   | "°F"    => Atom::DegreeFahrenheit,
            "degree Réaumur"           | "[degRe]"    | "°Ré"                => Atom::DegreeReaumur,
            "degree"                   | "deg"        | "DEG"      | "°"     => Atom::Degree,
            "fluid ounce"              | "[foz_us]"   | "[FOZ_US]" | "oz fl" => Atom::FluidOunceUS,
            "foot"                     | "[ft_i]"     | "[FT_I]"   | "ft"    => Atom::FootInternational,
            "ft(us)"                   | "[ft_us]"    | "[FT_US]"            => Atom::FootUS,
            "gill"                     | "[gil_us]"   | "[GIL_US]"           => Atom::GillUS,
            "inch"                     | "[in_i]"     | "[IN_I]"   | "in"    => Atom::InchInternational,
            "liter"                    | "l"          | "L"                  => Atom::Liter,
            "mole"                     | "mol"        | "MOL"                => Atom::Mole,
            "pH"                       | "[pH]"       | "[PH]"               => Atom::PH,
            "parts per billion"        | "[ppb]"      | "[PPB]"    | "ppb"   => Atom::PartsPerBillion,
            "parts per million"        | "[ppm]"      | "[PPM]"    | "ppm"   => Atom::PartsPerMillion,
            "parts per thousand"       | "[ppth]"     | "[PPTH]"   | "ppt"   => Atom::PartsPerThousand,
            "pint"                     | "[pt_us]"    | "[PT_US]"            => Atom::PintUS,
            "prism diopter"            | "[p'diop]"   | "[P'DIOP]" | "PD"    => Atom::PrismDiopter,
            "quart"                    | "[qt_us]"    | "[QT_US]"            => Atom::QuartUS,
            "rod"                      | "[rd_us]"    | "[RD_US]"            => Atom::RodUS,
            "the number pi"            | "[pi]"       | "[PI]"     | "π"     => Atom::TheNumberPi,

            _ => unreachable!(),
        };

        term.atom = Some(atom);
    }

    fn visit_prefix_symbol<I: Input>(&mut self, pair: Pair<Rule, I>, term: &mut Term) {
        let prefix = match pair.into_span().as_str() {
            "atto"  | "a"   | "A"         => Prefix::Atto,
            "centi" | "c"   | "C"         => Prefix::Centi,
            "deci"  | "d"   | "D"         => Prefix::Deci,
            "deka"  | "da"  | "DA"        => Prefix::Deka,
            "exa"   | "E"   | "EX"        => Prefix::Exa,
            "femto" | "f"   | "F"         => Prefix::Femto,
            "gibi"  | "Gi"  | "GIB"       => Prefix::Gibi,
            "giga"  | "G"   | "GA"        => Prefix::Giga,
            "hecto" | "h"   | "H"         => Prefix::Hecto,
            "kilo"  | "k"   | "K"         => Prefix::Kilo,
            "mebi"  | "Mi"  | "MIB"       => Prefix::Mebi,
            "mega"  | "M"   | "MA"        => Prefix::Mega,
            "micro" | "u"   | "U"   | "µ" => Prefix::Micro,
            "milli" | "m"                 => Prefix::Milli,
            "nano"  | "n"   | "N"         => Prefix::Nano,
            "peta"  | "P"   | "PT"        => Prefix::Peta,
            "tebi"  | "Ti"  | "TIB"       => Prefix::Tebi,
            "tera"  | "T"   | "TR"        => Prefix::Tera,
            "yocto" | "y"   | "YO"        => Prefix::Yocto,
            "yotta" | "Y"   | "YA"        => Prefix::Yotta,
            "zepto" | "z"   | "ZO"        => Prefix::Zepto,
            "zetta" | "Z"   | "ZA"        => Prefix::Zetta,
            _                             => unreachable!(),
        };

        term.prefix = Some(prefix);
    }

    fn visit_prefixed_atom<I: Input>(
        &mut self,
        pair: Pair<Rule, I>,
        term: &mut Term
    ) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::prefix_symbol => {
                    self.visit_prefix_symbol(inner_pair, term);
                }
                Rule::atom_symbol => {
                    self.visit_atom_symbol(inner_pair, term);
                }
                _ => unreachable!(),
            }
        }
    }

    fn visit_digits<I: Input>(&mut self, pair: Pair<Rule, I>) -> i32 {
        let span = pair.into_span();
        let string = span.as_str();

        string.parse::<i32>().unwrap_or_else(|e| panic!("{}", e))
    }

    fn visit_exponent<I: Input>(&mut self, pair: Pair<Rule, I>, term: &mut Term) {
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
                        _ => unreachable!(),
                    }
                }
                Rule::digits => {
                    e *= self.visit_digits(inner_pair);
                }
                _ => unreachable!(),
            }
        }

        term.exponent = e;
    }

    fn visit_simple_unit<I: Input>(
        &mut self,
        pair: Pair<Rule, I>,
        term: &mut Term
    ) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::prefixed_atom => {
                    self.visit_prefixed_atom(inner_pair, term);
                }
                Rule::atom_symbol => {
                    self.visit_atom_symbol(inner_pair, term);
                }
                _ => unreachable!(),
            }
        }
    }

    fn visit_simple_unit_with_exponent<I: Input>(
        &mut self,
        pair: Pair<Rule, I>,
        term: &mut Term
    ) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::simple_unit => {
                    self.visit_simple_unit(inner_pair, term);
                }
                Rule::exponent => {
                    self.visit_exponent(inner_pair, term);
                }
                _ => unreachable!(),
            }
        }
    }

    // TODO
    // fn visit_special_unit(&mut self, su: &SpecialUnit) -> Term {
    //     // Term::new(None, None)
    //     unimplemented!()
    // }

    fn visit_annotatable<I: Input>(
        &mut self,
        pair: Pair<Rule, I>,
        term: &mut Term
    ) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::simple_unit_with_exponent => {
                    self.visit_simple_unit_with_exponent(inner_pair, term);
                }
                Rule::simple_unit => {
                    self.visit_simple_unit(inner_pair, term);
                }
                // Rule::special_unit => {}
                _ => unreachable!(),
            }
        }
    }

    fn visit_annotation_string<I: Input>(&mut self, pair: Pair<Rule, I>, term: &mut Term) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::annotation_string => {
                    term.annotation = Some(inner_pair.into_span().as_str().to_string())
                }
                _ => unreachable!(),
            }
        }
    }

    fn visit_annotation<I: Input>(&mut self, pair: Pair<Rule, I>, term: &mut Term) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::annotation_string => {
                    self.visit_annotation_string(inner_pair, term);
                }
                _ => unreachable!(),
            }
        }
    }

    fn visit_annotated_annotatable<I: Input>(
        &mut self,
        pair: Pair<Rule, I>,
        term: &mut Term
    ) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::annotatable => {
                    self.visit_annotatable(inner_pair, term);
                }
                Rule::annotation => {
                    self.visit_annotation(inner_pair, term);
                }
                _ => unreachable!(),
            }
        }
    }

    fn visit_factor<I: Input>(&mut self, pair: Pair<Rule, I>) -> u32 {
        let mut factor = 0;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::digits => {
                    factor = self.visit_digits(inner_pair);
                }
                _ => unreachable!(),
            }
        }

        factor as u32
    }

    fn visit_basic_component<I: Input>(&mut self, pair: Pair<Rule, I>, terms: &mut Vec<Term>) {
        let mut term = Term::new(None, None);
        let mut is_term = false;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::annotated_annotatable => {
                    self.visit_annotated_annotatable(inner_pair, &mut term);
                }
                Rule::annotatable => {
                    self.visit_annotatable(inner_pair, &mut term);
                }
                Rule::annotation => {
                    term.annotation = Some(inner_pair.into_span().as_str().to_string());
                }
                Rule::factor => {
                    term.factor = self.visit_factor(inner_pair);
                }
                Rule::term => {
                    is_term = true;
                    self.visit_term(inner_pair, terms);
                }
                _ => unreachable!(),
            }
        }

        if !is_term {
            terms.push(term);
        }
    }

    fn visit_component_with_factor<I: Input>(
        &mut self,
        pair: Pair<Rule, I>,
        terms: &mut Vec<Term>,
    ) {
        let mut factor = 1;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::factor => {
                    factor = self.visit_factor(inner_pair);
                }
                Rule::basic_component => {
                    self.visit_basic_component(inner_pair, terms);
                }
                _ => unreachable!(),
            };
        }

        if let Some(first_term) = terms.first_mut() {
            first_term.factor = factor;
        }
    }

    fn visit_component<I: Input>(&mut self, pair: Pair<Rule, I>, mut terms: &mut Vec<Term>) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::component_with_factor => {
                    self.visit_component_with_factor(inner_pair, &mut terms)
                }
                Rule::basic_component => self.visit_basic_component(inner_pair, &mut terms),
                _ => unreachable!(),
            }
        }
    }

    fn visit_basic_term<I: Input>(&mut self, pair: Pair<Rule, I>, terms: &mut Vec<Term>) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::component => {
                    self.visit_component(inner_pair, terms);
                }
                _ => unreachable!(),
            }
        }
    }

    fn visit_slash_term<I: Input>(&mut self, pair: Pair<Rule, I>, terms: &mut Vec<Term>) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::component => {
                    self.visit_component(inner_pair, terms);
                }
                Rule::term => {
                    let mut new_terms: Vec<Term> = vec![];
                    self.visit_term(inner_pair, &mut new_terms);

                    for new_term in &mut new_terms {
                        new_term.exponent = -new_term.exponent;
                    }

                    terms.append(&mut new_terms);
                }
                _ => unreachable!(),
            }
        }
    }

    fn visit_dot_term<I: Input>(&mut self, pair: Pair<Rule, I>, mut terms: &mut Vec<Term>) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::component => {
                    self.visit_component(inner_pair, &mut terms);
                }
                Rule::term => {
                    let mut new_terms: Vec<Term> = vec![];

                    self.visit_term(inner_pair, &mut new_terms);
                    terms.append(&mut new_terms);
                }
                _ => unreachable!(),
            }
        }
    }

    fn visit_term<I: Input>(&mut self, pair: Pair<Rule, I>, mut terms: &mut Vec<Term>) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::dot_term => self.visit_dot_term(inner_pair, &mut terms),
                Rule::slash_term => self.visit_slash_term(inner_pair, &mut terms),
                Rule::basic_term => self.visit_basic_term(inner_pair, &mut terms),
                _ => {
                    println!("visit_term: unreachable rule: {:?}", inner_pair);
                    unreachable!()
                }
            }
        }
    }

    fn visit_slash_main_term<I: Input>(&mut self, pair: Pair<Rule, I>, terms: &mut Vec<Term>) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::term => {
                    let mut new_terms: Vec<Term> = vec![];
                    self.visit_term(inner_pair, &mut new_terms);

                    for new_term in &mut new_terms {
                        new_term.exponent = -new_term.exponent;
                    }

                    terms.append(&mut new_terms);
                }
                _ => unreachable!(),
            }
        }
    }

    fn visit_main_term<I: Input>(&mut self, pair: Pair<Rule, I>, terms: &mut Vec<Term>) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::slash_main_term => {
                    self.visit_slash_main_term(inner_pair, terms);
                },
                Rule::term => {
                    self.visit_term(inner_pair, terms);
                }
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atom::Atom;
    use pest::Parser;
    use unit::Unit;
    use unit_parser::{Rule, UnitParser};

    #[test]
    fn validate_exponent() {
        let pairs = UnitParser::parse_str(Rule::main_term, "m-3").unwrap();
        let mut i = Interpreter;
        let actual = i.interpret(pairs);

        let mut expected_term = Term::new(Some(Atom::Meter), None);
        expected_term.exponent = -3;

        let expected = Unit {
            terms: vec![expected_term],
        };

        assert_eq!(actual, expected);

        let pairs = UnitParser::parse_str(Rule::main_term, "km2/m-3").unwrap();
        let actual = i.interpret(pairs);

        let mut term1 = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        term1.exponent = 2;

        let mut term2 = Term::new(Some(Atom::Meter), None);
        term2.exponent = 3;

        let expected = Unit {
            terms: vec![term1, term2],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_basic_term() {
        let pairs = UnitParser::parse_str(Rule::main_term, "m").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let expected = Unit {
            terms: vec![Term::new(Some(Atom::Meter), None)],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_basic_term_with_exponent() {
        let pairs = UnitParser::parse_str(Rule::main_term, "m2").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let mut expected_term = Term::new(Some(Atom::Meter), None);
        expected_term.exponent = 2;

        let expected = Unit {
            terms: vec![expected_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_basic_term_with_prefix() {
        let pairs = UnitParser::parse_str(Rule::main_term, "km").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);

        let expected = Unit {
            terms: vec![Term::new(Some(Atom::Meter), Some(Prefix::Kilo))],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_basic_term_with_factor() {
        let pairs = UnitParser::parse_str(Rule::main_term, "2m").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);

        let mut expected_term = Term::new(Some(Atom::Meter), None);
        expected_term.factor = 2;

        let expected = Unit {
            terms: vec![expected_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term() {
        let pairs = UnitParser::parse_str(Rule::main_term, "m/s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;


        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_numerator_prefix() {
        let pairs = UnitParser::parse_str(Rule::main_term, "km/s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let expected_numerator_term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        let mut expected_denominator_term = Term::new(Some(Atom::Second), None);
        expected_denominator_term.exponent = -1;

        let expected = Unit {
            terms: vec![expected_numerator_term, expected_denominator_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_denominator_prefix() {
        let pairs = UnitParser::parse_str(Rule::main_term, "m/ks").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let expected_numerator_term = Term::new(Some(Atom::Meter), None);
        let mut expected_denominator_term = Term::new(Some(Atom::Second), Some(Prefix::Kilo));
        expected_denominator_term.exponent = -1;

        let expected = Unit {
            terms: vec![expected_numerator_term, expected_denominator_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_numerator_exponent() {
        let pairs = UnitParser::parse_str(Rule::main_term, "m2/s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_denominator_exponent() {
        let pairs = UnitParser::parse_str(Rule::main_term, "m/s2").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -2;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_numerator_and_denominator_exponent() {
        let pairs = UnitParser::parse_str(Rule::main_term, "m2/s2").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -2;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_factor_in_numerator() {
        let pairs = UnitParser::parse_str(Rule::main_term, "2m/s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.factor = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_factor_in_denominator() {
        let pairs = UnitParser::parse_str(Rule::main_term, "m/2s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;
        second_term.factor = 2;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term() {
        let pairs = UnitParser::parse_str(Rule::main_term, "m.s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let meter_term = Term::new(Some(Atom::Meter), None);
        let second_term = Term::new(Some(Atom::Second), None);

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_left_side_exponent() {
        let pairs = UnitParser::parse_str(Rule::main_term, "m2.s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let second_term = Term::new(Some(Atom::Second), None);

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_right_side_exponent() {
        let pairs = UnitParser::parse_str(Rule::main_term, "m.s2").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = 2;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_left_and_right_side_exponent() {
        let pairs = UnitParser::parse_str(Rule::main_term, "m2.s2").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = 2;


        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_factor_in_left_side() {
        let pairs = UnitParser::parse_str(Rule::main_term, "2m.s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.factor = 2;
        let second_term = Term::new(Some(Atom::Second), None);

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_factor_in_right_side() {
        let pairs = UnitParser::parse_str(Rule::main_term, "m.2s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.factor = 2;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_term_with_dot_term_then_slash_component() {
        let pairs = UnitParser::parse_str(Rule::main_term, "[acr_us].[in_i]/[acr_us]").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs);
        let acre_term = Term::new(Some(Atom::AcreUS), None);
        let inch_term = Term::new(Some(Atom::InchInternational), None);
        let mut acre2_term = Term::new(Some(Atom::AcreUS), None);
        acre2_term.exponent = -1;

        let expected = Unit {
            terms: vec![acre_term, inch_term, acre2_term],
        };

        assert_eq!(actual, expected);
    }
}
