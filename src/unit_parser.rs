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
        let pairs = UnitParser::parse(Rule::digit, "0");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::digit, "a");
        assert!(pairs.is_err());
    }

    #[test]
    fn validate_digits() {
        let pairs = UnitParser::parse(Rule::digits, "0");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::digits, "01");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::digits, "123450");
        assert!(pairs.is_ok());

        // Looks like it stops parsing at the !, but doesn't error.
        parses_to! {
            parser: UnitParser,
            input: "123456!@#",
            rule: Rule::digits,
            tokens: [digits(0, 6)]
        }

        let pairs = UnitParser::parse(Rule::digits, "!@#123450");
        assert!(pairs.is_err());
    }

    #[test]
    fn validate_factor() {
        let pairs = UnitParser::parse(Rule::factor, "0");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::factor, "01");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::factor, "123450");
        assert!(pairs.is_ok());

        parses_to! {
            parser: UnitParser,
            input: "123456!@#",
            rule: Rule::factor,
            tokens: [factor(0, 6, [digits(0, 6)])]
        }
    }

    #[test]
    fn validate_exponent() {
        let pairs = UnitParser::parse(Rule::exponent, "+0");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::exponent, "-0");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::exponent, "123");
        assert!(pairs.is_ok());

        parses_to! {
            parser: UnitParser,
            input: "-123",
            rule: Rule::exponent,
            tokens: [exponent(0, 4, [sign(0, 1), digits(1, 4)])]
        }
    }

    #[test]
    fn validate_prefix_symbol() {
        let pairs = UnitParser::parse(Rule::prefix_symbol, "k");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::prefix_symbol, "K");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::prefix_symbol, "kilo");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::prefix_symbol, "i");
        assert!(pairs.is_err());

        parses_to! {
            parser: UnitParser,
            input: "a",
            rule: Rule::prefix_symbol,
            tokens: [prefix_symbol(0, 1)]
        }
    }

    #[test]
    fn validate_atom_symbol() {
        let pairs = UnitParser::parse(Rule::atom_symbol, "m");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::atom_symbol, "M");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::atom_symbol, "meter");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::atom_symbol, "k");
        assert!(pairs.is_err());

        parses_to! {
            parser: UnitParser,
            input: "K",
            rule: Rule::atom_symbol,
            tokens: [atom_symbol(0, 1)]
        }

        parses_to! {
            parser: UnitParser,
            input: "10*",
            rule: Rule::atom_symbol,
            tokens: [atom_symbol(0, 3)]
        }
    }

    #[test]
    fn validate_prefixed_atom() {
        let pairs = UnitParser::parse(Rule::prefixed_atom, "km");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::prefixed_atom, "k");
        assert!(pairs.is_err());

        let pairs = UnitParser::parse(Rule::prefixed_atom, "m");
        assert!(pairs.is_err());

        parses_to! {
            parser: UnitParser,
            input: "km",
            rule: Rule::prefixed_atom,
            tokens: [prefixed_atom(0, 2, [
                                   prefix_symbol(0, 1),
                                   atom_symbol(1, 2)
            ])]
        }

        parses_to! {
            parser: UnitParser,
            input: "kilometer",
            rule: Rule::prefixed_atom,
            tokens: [
                prefixed_atom(0, 9, [
                   prefix_symbol(0, 4),
                   atom_symbol(4, 9)
            ])]
        }
    }

    #[test]
    fn validate_simple_unit() {
        let pairs = UnitParser::parse(Rule::simple_unit, "km");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::simple_unit, "m");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::simple_unit, "k");
        assert!(pairs.is_err());

        parses_to! {
            parser: UnitParser,
            input: "km",
            rule: Rule::simple_unit,
            tokens: [
                simple_unit(0, 2, [
                            prefixed_atom(0, 2, [
                                          prefix_symbol(0, 1),
                                          atom_symbol(1, 2)
                            ])]
                           )
            ]
        }
    }

    #[test]
    fn validate_simple_unit_with_exponent() {
        let pairs = UnitParser::parse(Rule::simple_unit, "km2");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::simple_unit, "m-1");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::simple_unit, "k4");
        assert!(pairs.is_err());

        parses_to! {
            parser: UnitParser,
            input: "km2",
            rule: Rule::simple_unit_with_exponent,
            tokens: [
                simple_unit_with_exponent(0, 3, [
                                          simple_unit(0, 2, [
                                                      prefixed_atom(0, 2, [
                                                                    prefix_symbol(0, 1),
                                                                    atom_symbol(1, 2)
                                                      ])
                                          ]),
                                          exponent(2, 3, [digits(2, 3)])
                ])
            ]
        }
    }

    #[test]
    fn validate_annotatable() {
        let pairs = UnitParser::parse(Rule::annotatable, "km2");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::annotatable, "km-2");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::annotatable, "km+2");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::annotatable, "km");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::annotatable, "m");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::annotatable, "k");
        assert!(pairs.is_err());
    }

    #[test]
    fn validate_annotation_string() {
        let pairs = UnitParser::parse(Rule::annotation_string, "k");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::annotation_string, "{d'io}");
        assert!(pairs.is_ok());

        parses_to! {
            parser: UnitParser,
            input: "tot'nit",
            rule: Rule::annotation_string,
            tokens: [annotation_string(0, 7)]
        };
    }

    #[test]
    fn validate_annotation() {
        let pairs = UnitParser::parse(Rule::annotation, "{d'io}");
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

        let pairs = UnitParser::parse(Rule::annotation, "{tot'nit}");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::annotation, "k");
        assert!(pairs.is_err());
    }

    #[test]
    fn validate_basic_component() {
        let pairs = UnitParser::parse(Rule::basic_component, "km{stuff}");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::basic_component, "km");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::basic_component, "{stuff}");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::basic_component, "234");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::basic_component, "(m.s)");
        assert!(pairs.is_ok());
    }

    #[test]
    fn validate_component_with_factor() {
        let pairs = UnitParser::parse(Rule::component_with_factor, "100km");
        assert!(pairs.is_ok());

        parses_to! {
            parser: UnitParser,
            input: "2km",
            rule: Rule::component_with_factor,
            tokens: [
                component_with_factor(0, 3, [
                      factor(0, 1, [digits(0, 1)]),
                      basic_component(1, 3, [
                              annotatable(1, 3, [
                                      simple_unit(1, 3, [
                                              prefixed_atom(1, 3, [
                                                        prefix_symbol(1, 2),
                                                        atom_symbol(2, 3)
                                              ])
                                      ])
                              ])
                      ])
                ])
            ]
        };

        parses_to! {
            parser: UnitParser,
            input: "2km-2{meow}",
            rule: Rule::component_with_factor,
            tokens: [
                component_with_factor(0, 11, [
                    factor(0, 1, [digits(0, 1)]),
                    basic_component(1, 11, [
                        annotated_annotatable(1, 11, [
                            annotatable(1, 5, [
                                simple_unit_with_exponent(1, 5, [
                                    simple_unit(1, 3, [
                                        prefixed_atom(1, 3, [
                                            prefix_symbol(1, 2),
                                            atom_symbol(2, 3)
                                        ])
                                    ]),
                                    exponent(3, 5, [
                                        sign(3, 4),
                                        digits(4, 5)
                                    ])
                                ])
                            ]),
                            annotation(5, 11, [
                                annotation_string(6, 10)
                            ])
                        ])
                   ])
               ])
            ]
        };
    }

    #[test]
    fn validate_basic_term() {
        parses_to! {
            parser: UnitParser,
            input: "2km-2{meow}",
            rule: Rule::basic_term,
            tokens: [
                basic_term(0, 11, [
                    component(0, 11, [
                        component_with_factor(0, 11, [
                            factor(0, 1, [digits(0, 1)]),
                            basic_component(1, 11, [
                                annotated_annotatable(1, 11, [
                                    annotatable(1, 5, [
                                        simple_unit_with_exponent(1, 5, [
                                            simple_unit(1, 3, [
                                                prefixed_atom(1, 3, [
                                                    prefix_symbol(1, 2),
                                                    atom_symbol(2, 3)
                                                ])
                                            ]),
                                            exponent(3, 5, [
                                                sign(3, 4),
                                                digits(4, 5)
                                            ])
                                        ])
                                    ]),
                                    annotation(5, 11, [
                                        annotation_string(6, 10)
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
    fn validate_slash_term() {
        let pairs = UnitParser::parse(Rule::slash_term, "km/s");
        assert!(pairs.is_ok());

        parses_to! {
            parser: UnitParser,
            input: "2km-2{meow}/[acr_us].[in_i]",
            rule: Rule::slash_term,
            tokens: [
                slash_term(0, 27, [
                    component(0, 11, [
                        component_with_factor(0, 11, [
                            factor(0, 1, [digits(0, 1)]),
                            basic_component(1, 11, [
                                annotated_annotatable(1, 11, [
                                    annotatable(1, 5, [
                                        simple_unit_with_exponent(1, 5, [
                                            simple_unit(1, 3, [
                                                prefixed_atom(1, 3, [
                                                    prefix_symbol(1, 2),
                                                    atom_symbol(2, 3)
                                                ])
                                            ]),
                                            exponent(3, 5, [
                                                sign(3, 4),
                                                digits(4, 5)
                                            ])
                                        ])
                                    ]),
                                    annotation(5, 11, [
                                        annotation_string(6, 10)
                                    ])
                                ])
                           ])
                       ])
                    ]),
                    term(12, 27, [
                         dot_term(12, 27, [
                            component(12, 20, [
                                basic_component(12, 20, [
                                    annotatable(12, 20, [
                                        simple_unit(12, 20, [
                                            atom_symbol(12, 20)
                                        ])
                                    ])
                                ])
                            ]),
                            term(21, 27, [
                                basic_term(21, 27, [
                                    component(21, 27, [
                                        basic_component(21, 27, [
                                            annotatable(21, 27, [
                                                simple_unit(21, 27, [
                                                    atom_symbol(21, 27)
                                                ])
                                            ])
                                        ])
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
            parser: UnitParser,
            input: "[acr_us].[in_i]/[acr_us]",
            rule: Rule::term,
            tokens: [
                term(0, 24, [
                    dot_term(0, 24, [
                        component(0, 8, [
                            basic_component(0, 8, [
                                annotatable(0, 8, [
                                    simple_unit(0, 8, [
                                        atom_symbol(0, 8)
                                    ])
                               ])
                            ])
                        ]),
                        term(9, 24, [
                             slash_term(9, 24, [
                                component(9, 15, [
                                    basic_component(9, 15, [
                                        annotatable(9, 15, [
                                            simple_unit(9, 15, [
                                                atom_symbol(9, 15)
                                            ])
                                        ])
                                    ])
                                ]),
                                term(16, 24, [
                                    basic_term(16, 24, [
                                        component(16, 24, [
                                            basic_component(16, 24, [
                                                annotatable(16, 24, [
                                                    simple_unit(16, 24, [
                                                        atom_symbol(16, 24)
                                                    ])
                                                ])
                                            ])
                                        ])
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
        let pairs = UnitParser::parse(Rule::main_term, "km/s");
        assert!(pairs.is_ok());

        let pairs = UnitParser::parse(Rule::main_term, "/km.s");
        assert!(pairs.is_ok());
    }
}
