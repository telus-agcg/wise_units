#[derive(Parser)]
#[grammar = "terms/term.pest"]
pub struct TermParser;

#[cfg(test)]
mod tests {
    use super::*;
    use super::TermParser;
    use pest::Parser;

    #[test]
    fn parse_sign() {
        parses_to! {
            parser: TermParser,
            input: "+",
            rule: Rule::sign,
            tokens: [
                sign(0, 1)
            ]
        }

        parses_to! {
            parser: TermParser,
            input: "-",
            rule: Rule::sign,
            tokens: [
                sign(0, 1)
            ]
        }
    }

    #[test]
    fn validate_digits() {
        let pairs = TermParser::parse(Rule::digits, "0");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::digits, "01");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::digits, "123450");
        assert!(pairs.is_ok());

        // Looks like it stops parsing at the !, but doesn't error.
        parses_to! {
            parser: TermParser,
            input: "123456!@#",
            rule: Rule::digits,
            tokens: [digits(0, 6)]
        }

        let pairs = TermParser::parse(Rule::digits, "!@#123450");
        assert!(pairs.is_err());
    }

    #[test]
    fn validate_factor() {
        let pairs = TermParser::parse(Rule::factor, "0");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::factor, "01");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::factor, "123450");
        assert!(pairs.is_ok());

        parses_to! {
            parser: TermParser,
            input: "123456!@#",
            rule: Rule::factor,
            tokens: [factor(0, 6)]
        }
    }

    #[test]
    fn validate_exponent() {
        let pairs = TermParser::parse(Rule::exponent, "+0");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::exponent, "-0");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::exponent, "123");
        assert!(pairs.is_ok());

        parses_to! {
            parser: TermParser,
            input: "-123",
            rule: Rule::exponent,
            tokens: [exponent(0, 4, [sign(0, 1), digits(1, 4)])]
        }
    }

    #[test]
    fn validate_simple_unit() {
        let pairs = TermParser::parse(Rule::simple_unit, "km");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::simple_unit, "m");
        assert!(pairs.is_ok());

        parses_to! {
            parser: TermParser,
            input: "km",
            rule: Rule::simple_unit,
            tokens: [
                simple_unit(0, 2)
            ]
        }
    }

    #[test]
    fn validate_simple_unit_with_exponent() {
        let pairs = TermParser::parse(Rule::simple_unit, "km2");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::simple_unit, "m-1");
        assert!(pairs.is_ok());

        parses_to! {
            parser: TermParser,
            input: "km2",
            rule: Rule::annotatable,
            tokens: [
                annotatable(0, 3, [
                      simple_unit(0, 2),
                      exponent(2, 3, [digits(2, 3)])
                ])
            ]
        }
    }

    #[test]
    fn validate_annotatable() {
        let pairs = TermParser::parse(Rule::annotatable, "km2");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::annotatable, "km-2");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::annotatable, "km+2");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::annotatable, "km");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::annotatable, "m");
        assert!(pairs.is_ok());
    }

    #[test]
    fn validate_annotation() {
        let pairs = TermParser::parse(Rule::annotation, "{d'io}");
        assert!(pairs.is_ok());

        parses_to! {
            parser: TermParser,
            input: "{tot'nit}",
            rule: Rule::annotation,
            tokens: [
                annotation(0, 9)
            ]
        };

        let pairs = TermParser::parse(Rule::annotation, "{tot'nit}");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::annotation, "k");
        assert!(pairs.is_err());
    }

    #[test]
    fn validate_basic_component() {
        let pairs = TermParser::parse(Rule::basic_component, "km{stuff}");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::basic_component, "km");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::basic_component, "{stuff}");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::basic_component, "234");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::basic_component, "(m.s)");
        assert!(pairs.is_ok());
    }

    #[test]
    fn validate_component_with_factor() {
        let pairs = TermParser::parse(Rule::component, "100km");
        assert!(pairs.is_ok());

        parses_to! {
            parser: TermParser,
            input: "2km",
            rule: Rule::component,
            tokens: [
                component(0, 3, [
                      factor(0, 1),
                      basic_component(1, 3, [
                              annotatable(1, 3, [
                                      simple_unit(1, 3)
                              ])
                      ])
                ])
            ]
        };

        parses_to! {
            parser: TermParser,
            input: "2km-2{meow}",
            rule: Rule::component,
            tokens: [
                component(0, 11, [
                    factor(0, 1),
                    basic_component(1, 11, [
                        annotatable(1, 5, [
                            simple_unit(1, 3),
                            exponent(3, 5, [
                                sign(3, 4),
                                digits(4, 5)
                            ])
                        ]),
                        annotation(5, 11)
                   ])
               ])
            ]
        };
    }

    #[test]
    fn validate_component_with_annotation() {
        parses_to! {
            parser: TermParser,
            input: "2km-2{meow}",
            rule: Rule::component,
            tokens: [
                component(0, 11, [
                    factor(0, 1),
                    basic_component(1, 11, [
                        annotatable(1, 5, [
                            simple_unit(1, 3),
                            exponent(3, 5, [
                                sign(3, 4),
                                digits(4, 5)
                            ])
                        ]),
                        annotation(5, 11)
                   ])
               ])
            ]
        };
    }

    #[test]
    fn validate_slash_term() {
        let pairs = TermParser::parse(Rule::term, "km/s");
        assert!(pairs.is_ok());

        parses_to! {
            parser: TermParser,
            input: "2km-2{meow}/[acr_us].[in_i]",
            rule: Rule::term,
            tokens: [
                term(0, 27, [
                    component(0, 11, [
                        factor(0, 1),
                        basic_component(1, 11, [
                            annotatable(1, 5, [
                                simple_unit(1, 3),
                                exponent(3, 5, [
                                    sign(3, 4),
                                    digits(4, 5)
                                ])
                            ]),
                            annotation(5, 11)
                       ])
                    ]),
                    slash(11, 12),
                    term(12, 27, [
                        component(12, 20, [
                            basic_component(12, 20, [
                                annotatable(12, 20, [
                                    simple_unit(12, 20)
                                ])
                            ])
                        ]),
                        term(21, 27, [
                            component(21, 27, [
                                basic_component(21, 27, [
                                    annotatable(21, 27, [
                                        simple_unit(21, 27)
                                    ])
                                ])
                            ])
                        ])
                    ])
               ])
            ]
        };
    }

    #[test]
    fn validate_interpret_term_with_dot_term_then_slash_component() {
        parses_to! {
            parser: TermParser,
            input: "[acr_us].[in_i]/[acr_us]",
            rule: Rule::term,
            tokens: [
                term(0, 24, [
                    component(0, 8, [
                        basic_component(0, 8, [
                            annotatable(0, 8, [
                                simple_unit(0, 8)
                           ])
                        ])
                    ]),
                    term(9, 24, [
                        component(9, 15, [
                            basic_component(9, 15, [
                                annotatable(9, 15, [
                                    simple_unit(9, 15)
                                ])
                            ])
                        ]),
                        slash(15, 16),
                        term(16, 24, [
                            component(16, 24, [
                                basic_component(16, 24, [
                                    annotatable(16, 24, [
                                        simple_unit(16, 24)
                                    ])
                                ])
                            ])
                        ])
                    ])
                ])
            ]
        };
    }

    #[test]
    fn validate_main_term() {
        let pairs = TermParser::parse(Rule::main_term, "km/s");
        assert!(pairs.is_ok());

        let pairs = TermParser::parse(Rule::main_term, "/km.s");
        assert!(pairs.is_ok());

        parses_to! {
            parser: TermParser,
            input: "/2m",
            rule: Rule::main_term,
            tokens: [
                main_term(0, 3, [
                    slash(0, 1),
                    term(1, 3, [
                        component(1, 3, [
                            factor(1, 2),
                            basic_component(2, 3, [
                                annotatable(2, 3, [
                                    simple_unit(2, 3)
                               ])
                            ])
                        ])
                    ])
                ])
            ]
        };

        parses_to! {
            parser: TermParser,
            input: "/1",
            rule: Rule::main_term,
            tokens: [
                main_term(0, 2, [
                    slash(0, 1),
                    term(1, 2, [
                        component(1, 2, [
                            basic_component(1, 2, [
                                annotatable(1, 2, [
                                    simple_unit(1, 2)
                               ])
                            ])
                        ])
                    ])
                ])
            ]
        }
    }
}
