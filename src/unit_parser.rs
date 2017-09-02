#[derive(Parser)]
#[grammar = "unit.pest"]
pub struct UnitParser;

#[cfg(test)]
mod tests {
    use super::*;
    use super::UnitParser;
    use pest::Parser;

    #[test]
    fn parse_sign() {
        parses_to! {
            parser: UnitParser,
            input: "+",
            rule: Rule::sign,
            tokens: [
                sign(0, 1)
            ]
        }

        parses_to! {
            parser: UnitParser,
            input: "-",
            rule: Rule::sign,
            tokens: [
                sign(0, 1)
            ]
        }
    }


    #[test]
    fn validate_digit() {
        let parser = UnitParser::parse_str(Rule::digit, "0");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::digit, "a");
        assert!(parser.is_err());
    }

    #[test]
    fn validate_digits() {
        let parser = UnitParser::parse_str(Rule::digits, "0");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::digits, "01");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::digits, "123450");
        assert!(parser.is_ok());

        // Looks like it stops parsing at the !, but doesn't error.
        parses_to! {
            parser: UnitParser,
            input: "123456!@#",
            rule: Rule::digits,
            tokens: [digits(0, 6)]
        }

        let parser = UnitParser::parse_str(Rule::digits, "!@#123450");
        assert!(parser.is_err());
    }

    #[test]
    fn validate_factor() {
        let parser = UnitParser::parse_str(Rule::factor, "0");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::factor, "01");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::factor, "123450");
        assert!(parser.is_ok());

        parses_to! {
            parser: UnitParser,
            input: "123456!@#",
            rule: Rule::factor,
            tokens: [factor(0, 6, [digits(0, 6)])]
        }
    }

    #[test]
    fn validate_exponent() {
        let parser = UnitParser::parse_str(Rule::exponent, "+0");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::exponent, "-0");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::exponent, "123");
        assert!(parser.is_ok());

        parses_to! {
            parser: UnitParser,
            input: "-123",
            rule: Rule::exponent,
            tokens: [exponent(0, 4, [sign(0, 1), digits(1, 4)])]
        }
    }

    #[test]
    fn validate_prefix_symbol() {
        let parser = UnitParser::parse_str(Rule::prefix_symbol, "k");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::prefix_symbol, "K");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::prefix_symbol, "kilo");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::prefix_symbol, "i");
        assert!(parser.is_err());

        parses_to! {
            parser: UnitParser,
            input: "a",
            rule: Rule::prefix_symbol,
            tokens: [prefix_symbol(0, 1, [atto(0, 1)])]
        }
    }

    #[test]
    fn validate_atom_symbol() {
        let parser = UnitParser::parse_str(Rule::atom_symbol, "m");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::atom_symbol, "M");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::atom_symbol, "meter");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::atom_symbol, "k");
        assert!(parser.is_err());

        parses_to! {
            parser: UnitParser,
            input: "K",
            rule: Rule::atom_symbol,
            tokens: [atom_symbol(0, 1, [kelvin(0, 1)])]
        }
    }

    #[test]
    fn validate_prefixed_atom() {
        let parser = UnitParser::parse_str(Rule::prefixed_atom, "km");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::prefixed_atom, "k");
        assert!(parser.is_err());

        let parser = UnitParser::parse_str(Rule::prefixed_atom, "m");
        assert!(parser.is_err());

        parses_to! {
            parser: UnitParser,
            input: "km",
            rule: Rule::prefixed_atom,
            tokens: [prefixed_atom(0, 2, [
                                   prefix_symbol(0, 1, [kilo(0, 1)]),
                                   atom_symbol(1, 2, [meter(1, 2)])
            ])]
        }
    }

    #[test]
    fn validate_simple_unit() {
        let parser = UnitParser::parse_str(Rule::simple_unit, "km");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::simple_unit, "m");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::simple_unit, "k");
        assert!(parser.is_err());

        parses_to! {
            parser: UnitParser,
            input: "km",
            rule: Rule::simple_unit,
            tokens: [
                simple_unit(0, 2, [
                            prefixed_atom(0, 2, [
                                          prefix_symbol(0, 1, [kilo(0, 1)]),
                                          atom_symbol(1, 2, [meter(1, 2)])
                                 ])]
                            )
            ]
        }
    }

    #[test]
    fn validate_simple_unit_with_exponent() {
        let parser = UnitParser::parse_str(Rule::simple_unit, "km2");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::simple_unit, "m-1");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::simple_unit, "k4");
        assert!(parser.is_err());

        parses_to! {
            parser: UnitParser,
            input: "km2",
            rule: Rule::simple_unit_with_exponent,
            tokens: [
                simple_unit_with_exponent(0, 3, [
                            simple_unit(0, 2, [
                                prefixed_atom(0, 2, [
                                              prefix_symbol(0, 1, [kilo(0, 1)]),
                                              atom_symbol(1, 2, [meter(1, 2)])
                                     ])
                            ]),
                            exponent(2, 3, [digits(2, 3)])
                ])
            ]
        }
    }

    #[test]
    fn validate_annotatable() {
        let parser = UnitParser::parse_str(Rule::annotatable, "km2");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::annotatable, "km-2");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::annotatable, "km+2");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::annotatable, "km");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::annotatable, "m");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::annotatable, "k");
        assert!(parser.is_err());
    }

    #[test]
    fn validate_annotation_string() {
        let parser = UnitParser::parse_str(Rule::annotation_string, "k");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::annotation_string, "{d'io}");
        assert!(parser.is_ok());

        parses_to! {
            parser: UnitParser,
            input: "tot'nit",
            rule: Rule::annotation_string,
            tokens: [annotation_string(0, 7)]
        };
    }

    #[test]
    fn validate_annotation() {
        let pairs = UnitParser::parse_str(Rule::annotation, "{d'io}");
        assert!(pairs.is_ok());

        parses_to! {
            parser: UnitParser,
            input: "{tot'nit}",
            rule: Rule::annotation,
            tokens: [
                annotation(0, 9, [
                    annotation_string(1, 8)
                ])
            ]
        };

        let parser = UnitParser::parse_str(Rule::annotation, "{tot'nit}");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::annotation, "k");
        assert!(parser.is_err());
    }

    #[test]
    fn validate_component() {
        let parser = UnitParser::parse_str(Rule::component, "km{stuff}");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::component, "km");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::component, "{stuff}");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::component, "234");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::component, "(m.s)");
        assert!(parser.is_ok());
    }

    #[test]
    fn validate_slash_term() {
        let parser = UnitParser::parse_str(Rule::term, "km/s");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::term, "km.s");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::term, "km{stuff}");
        println!("meow: {:?} 2", parser);
        assert!(parser.is_ok());
    }

    #[test]
    fn validate_main_term() {
        let parser = UnitParser::parse_str(Rule::main_term, "km/s");
        assert!(parser.is_ok());

        let parser = UnitParser::parse_str(Rule::main_term, "/km.s");
        println!("parsaer: {:?}", parser);
        assert!(parser.is_ok());
    }
}
