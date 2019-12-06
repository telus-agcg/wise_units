use crate::{unit::UnitDefinition, Error, Prefix};
use std::{borrow::Borrow, convert::TryFrom};
use wise_units_parser::tokens::{Component, MainTerm, Term, TermSeparator};

pub struct RuntimeUnitBuilder {
    factor: Option<u32>,
    prefix: Option<Prefix>,
    unit: Option<UnitDefinition>,
    exponent: i32,
    annotation: Option<String>,
    rhs: Option<Box<RuntimeUnit>>,
}

impl RuntimeUnitBuilder {
    pub fn factor(mut self, factor: u32) -> Self {
        self.factor = Some(factor);
        self
    }

    pub fn prefix(mut self, prefix: Prefix) -> Self {
        self.prefix = Some(prefix);
        self
    }

    pub fn unit(mut self, unit: UnitDefinition) -> Self {
        self.unit = Some(unit);
        self
    }

    pub const fn exponent(mut self, exponent: i32) -> Self {
        self.exponent = exponent;
        self
    }

    pub fn annotation(mut self, annotation: String) -> Self {
        self.annotation = Some(annotation);
        self
    }

    pub fn rhs(mut self, rhs: RuntimeUnit) -> Self {
        self.rhs = Some(Box::new(rhs));
        self
    }

    pub fn build(self) -> RuntimeUnit {
        RuntimeUnit {
            factor: self.factor,
            prefix: self.prefix,
            unit: self.unit,
            annotation: self.annotation,
            exponent: self.exponent,
            rhs: self.rhs,
        }
    }
}

impl Default for RuntimeUnitBuilder {
    fn default() -> Self {
        Self {
            factor: None,
            prefix: None,
            unit: None,
            exponent: 1,
            annotation: None,
            rhs: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RuntimeUnit {
    factor: Option<u32>,
    prefix: Option<Prefix>,
    unit: Option<UnitDefinition>,
    exponent: i32,
    annotation: Option<String>,
    rhs: Option<Box<RuntimeUnit>>,
}

impl RuntimeUnit {
    pub const fn factor(&self) -> Option<u32> {
        self.factor
    }

    pub fn prefix(&self) -> Option<&Prefix> {
        self.prefix.as_ref()
    }

    pub fn unit(&self) -> Option<&UnitDefinition> {
        self.unit.as_ref()
    }

    pub fn annotation(&self) -> Option<&str> {
        self.annotation.as_ref().map(|a| &*a.as_str())
    }

    pub const fn exponent(&self) -> i32 {
        self.exponent
    }

    pub fn rhs(&self) -> Option<&RuntimeUnit> {
        self.rhs.as_ref().map(|r| r.borrow())
    }

    pub fn invert(&mut self) {
        self.exponent *= -1;

        if let Some(ref mut rhs) = self.rhs {
            rhs.invert();
        }
    }
}

impl TryFrom<MainTerm<'_>> for RuntimeUnit {
    type Error = Error;

    fn try_from(main_term: MainTerm<'_>) -> Result<Self, Self::Error> {
        let mut unit = RuntimeUnit::try_from(main_term.term)?;

        if main_term.leading_slash {
            unit.invert();
        }

        Ok(unit)
    }
}

impl TryFrom<Term<'_>> for RuntimeUnit {
    type Error = Error;

    fn try_from(term: Term<'_>) -> Result<Self, Self::Error> {
        match term {
            Term::Combined(lhs, separator, rhs) => {
                let mut lhs_unit = RuntimeUnit::try_from(*lhs)?;
                assert!(lhs_unit.rhs.is_none());

                match separator {
                    TermSeparator::Dot => {
                        lhs_unit.rhs = Some(Box::new(RuntimeUnit::try_from(*rhs)?));
                    }
                    TermSeparator::Slash => {
                        let mut rhs = RuntimeUnit::try_from(*rhs)?;
                        rhs.invert();

                        lhs_unit.rhs = Some(Box::new(rhs));
                    }
                }

                Ok(lhs_unit)
            }
            Term::Basic(component) => RuntimeUnit::try_from(*component),
        }
    }
}

impl TryFrom<Component<'_>> for RuntimeUnit {
    type Error = Error;

    fn try_from(component: Component<'_>) -> Result<Self, Self::Error> {
        match component {
            Component::AnnotatedAnnotatable(annotatable, annotation) => {
                let prefix = match (annotatable.0).1 {
                    Some(prefix_symbol) => Some(Prefix::try_from(prefix_symbol)?),
                    None => None,
                };

                Ok(build_runtime_unit!(
                    factor: (annotatable.0).0,
                    prefix: prefix,
                    unit: Some(UnitDefinition::try_from((annotatable.0).2)?),
                    annotation: Some(annotation.0.to_owned()),
                    exponent: annotatable.1.unwrap_or(1),
                ))
            }
            Component::Annotatable(annotatable) => {
                let prefix = match (annotatable.0).1 {
                    Some(prefix_symbol) => Some(Prefix::try_from(prefix_symbol)?),
                    None => None,
                };

                Ok(build_runtime_unit!(
                    factor: (annotatable.0).0,
                    prefix: prefix,
                    unit: Some(UnitDefinition::try_from((annotatable.0).2)?),
                    exponent: annotatable.1.unwrap_or(1),
                ))
            }
            Component::Annotation(annotation) => {
                Ok(build_runtime_unit!(annotation: Some(annotation.0.to_owned())))
            }
            Component::Factor(factor) => Ok(build_runtime_unit!(factor: Some(factor))),
            Component::NestedTerm(term) => {
                let rhs = RuntimeUnit::try_from(term)?;

                Ok(build_runtime_unit!(rhs: Some(Box::new(rhs))))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RuntimeUnit;
    use crate::{
        prefixes::KILO,
        units::{GRAM, METER},
        Error,
    };
    use std::convert::TryFrom;
    use wise_units_parser::tokens::{
        Annotatable, Annotation, AtomSymbol, Component, PrefixSymbol, SimpleUnit, Term,
        TermSeparator,
    };

    mod try_from_term {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn test_basic_ok() {
            let term = Term::Basic(Box::new(Component::Annotatable(Annotatable(
                SimpleUnit(None, None, AtomSymbol("m")),
                None,
            ))));

            let expected_unit = build_runtime_unit!(unit: Some(METER));

            assert_eq!(RuntimeUnit::try_from(term).unwrap(), expected_unit);
        }

        #[test]
        fn test_basic_bad_unit() {
            let term = Term::Basic(Box::new(Component::Annotatable(Annotatable(
                SimpleUnit(None, None, AtomSymbol("!@#$")),
                None,
            ))));

            let expected_error = Error::UnknownAtomSymbol("!@#$".to_string());

            assert_eq!(RuntimeUnit::try_from(term).unwrap_err(), expected_error);
        }

        mod combined {
            use super::*;
            use pretty_assertions::assert_eq;

            // Ex. "m.g.m"
            #[test]
            fn test_dot_separator_lhs_and_rhs_valid() {
                let term = {
                    let lhs = Component::Annotatable(Annotatable(
                        SimpleUnit(None, None, AtomSymbol("m")),
                        None,
                    ));

                    let rhs = {
                        let lhs = Component::Annotatable(Annotatable(
                            SimpleUnit(None, None, AtomSymbol("g")),
                            None,
                        ));

                        let rhs = Term::Basic(Box::new(Component::Annotatable(Annotatable(
                            SimpleUnit(None, None, AtomSymbol("m")),
                            None,
                        ))));

                        Term::Combined(Box::new(lhs), TermSeparator::Dot, Box::new(rhs))
                    };

                    Term::Combined(Box::new(lhs), TermSeparator::Dot, Box::new(rhs))
                };

                let expected_rhs = {
                    let rhs = build_runtime_unit!(unit: Some(METER));
                    build_runtime_unit!(unit: Some(GRAM), rhs: Some(Box::new(rhs)))
                };

                let expected_unit =
                    build_runtime_unit!(unit: Some(METER), rhs: Some(Box::new(expected_rhs)));

                assert_eq!(RuntimeUnit::try_from(term).unwrap(), expected_unit);
            }

            // Ex. "m.g.!@#$"
            #[test]
            fn test_dot_separator_lhs_valid_but_rhs_invalid() {
                let term = {
                    let lhs = Component::Annotatable(Annotatable(
                        SimpleUnit(None, None, AtomSymbol("m")),
                        None,
                    ));

                    let rhs = {
                        let lhs = Component::Annotatable(Annotatable(
                            SimpleUnit(None, None, AtomSymbol("g")),
                            None,
                        ));

                        let rhs = Term::Basic(Box::new(Component::Annotatable(Annotatable(
                            SimpleUnit(None, None, AtomSymbol("!@#$")),
                            None,
                        ))));

                        Term::Combined(Box::new(lhs), TermSeparator::Dot, Box::new(rhs))
                    };

                    Term::Combined(Box::new(lhs), TermSeparator::Dot, Box::new(rhs))
                };

                let expected_error = Error::UnknownAtomSymbol("!@#$".to_string());

                assert_eq!(RuntimeUnit::try_from(term).unwrap_err(), expected_error);
            }

            // Ex. "!@#$.g.m"
            #[test]
            fn test_dot_separator_lhs_invalid_but_rhs_valid() {
                let term = {
                    let lhs = Component::Annotatable(Annotatable(
                        SimpleUnit(None, None, AtomSymbol("!@#$")),
                        None,
                    ));

                    let rhs = {
                        let lhs = Component::Annotatable(Annotatable(
                            SimpleUnit(None, None, AtomSymbol("g")),
                            None,
                        ));

                        let rhs = Term::Basic(Box::new(Component::Annotatable(Annotatable(
                            SimpleUnit(None, None, AtomSymbol("m")),
                            None,
                        ))));

                        Term::Combined(Box::new(lhs), TermSeparator::Dot, Box::new(rhs))
                    };

                    Term::Combined(Box::new(lhs), TermSeparator::Dot, Box::new(rhs))
                };

                let expected_error = Error::UnknownAtomSymbol("!@#$".to_string());

                assert_eq!(RuntimeUnit::try_from(term).unwrap_err(), expected_error);
            }

            // Ex. "m/g"
            #[test]
            fn test_slash_separator_lhs_and_basic_rhs_valid() {
                let term = {
                    let lhs = Component::Annotatable(Annotatable(
                        SimpleUnit(None, None, AtomSymbol("m")),
                        None,
                    ));

                    let rhs = Term::Basic(Box::new(Component::Annotatable(Annotatable(
                        SimpleUnit(None, None, AtomSymbol("g")),
                        None,
                    ))));

                    Term::Combined(Box::new(lhs), TermSeparator::Slash, Box::new(rhs))
                };

                let expected_rhs = build_runtime_unit!(unit: Some(GRAM), exponent: -1);

                let expected_unit =
                    build_runtime_unit!(unit: Some(METER), rhs: Some(Box::new(expected_rhs)));

                assert_eq!(RuntimeUnit::try_from(term).unwrap(), expected_unit);
            }

            // Ex. "m/g.m"
            #[test]
            fn test_slash_separator_lhs_and_dot_combined_rhs_valid() {
                let term = {
                    let lhs = Component::Annotatable(Annotatable(
                        SimpleUnit(None, None, AtomSymbol("m")),
                        None,
                    ));

                    let rhs = {
                        let lhs = Component::Annotatable(Annotatable(
                            SimpleUnit(None, None, AtomSymbol("g")),
                            None,
                        ));

                        let rhs = Term::Basic(Box::new(Component::Annotatable(Annotatable(
                            SimpleUnit(None, None, AtomSymbol("m")),
                            None,
                        ))));

                        Term::Combined(Box::new(lhs), TermSeparator::Dot, Box::new(rhs))
                    };

                    Term::Combined(Box::new(lhs), TermSeparator::Slash, Box::new(rhs))
                };

                let expected_rhs = {
                    let rhs = build_runtime_unit!(unit: Some(METER), exponent: -1);
                    build_runtime_unit!(unit: Some(GRAM), exponent: -1, rhs: Some(Box::new(rhs)))
                };

                let expected_unit =
                    build_runtime_unit!(unit: Some(METER), rhs: Some(Box::new(expected_rhs)));

                assert_eq!(RuntimeUnit::try_from(term).unwrap(), expected_unit);
            }

            // Ex. "m/g/m"
            #[test]
            fn test_slash_separator_lhs_and_slash_combined_rhs_valid() {
                let term = {
                    let lhs = Component::Annotatable(Annotatable(
                        SimpleUnit(None, None, AtomSymbol("m")),
                        None,
                    ));

                    let rhs = {
                        let lhs = Component::Annotatable(Annotatable(
                            SimpleUnit(None, None, AtomSymbol("g")),
                            None,
                        ));

                        let rhs = Term::Basic(Box::new(Component::Annotatable(Annotatable(
                            SimpleUnit(None, None, AtomSymbol("m")),
                            None,
                        ))));

                        Term::Combined(Box::new(lhs), TermSeparator::Slash, Box::new(rhs))
                    };

                    Term::Combined(Box::new(lhs), TermSeparator::Slash, Box::new(rhs))
                };

                let expected_rhs = {
                    let rhs = build_runtime_unit!(unit: Some(METER), exponent: 1);
                    build_runtime_unit!(unit: Some(GRAM), exponent: -1, rhs: Some(Box::new(rhs)))
                };

                let expected_unit =
                    build_runtime_unit!(unit: Some(METER), rhs: Some(Box::new(expected_rhs)));

                assert_eq!(RuntimeUnit::try_from(term).unwrap(), expected_unit);
            }
        }
    }

    mod try_from_component {
        use super::*;

        #[test]
        fn test_factor() {
            let component = Component::Factor(123);
            let expected_unit = build_runtime_unit!(factor: Some(123));

            assert_eq!(RuntimeUnit::try_from(component).unwrap(), expected_unit);
        }

        #[test]
        fn test_annotation() {
            let component = Component::Annotation(Annotation("stuff"));
            let expected_unit = build_runtime_unit!(annotation: Some("stuff".to_string()));

            assert_eq!(RuntimeUnit::try_from(component).unwrap(), expected_unit);
        }

        #[test]
        fn test_annotated_annotatable() {
            let component = Component::AnnotatedAnnotatable(
                Annotatable(
                    SimpleUnit(Some(123), Some(PrefixSymbol("k")), AtomSymbol("m")),
                    None,
                ),
                Annotation("things"),
            );

            let expected_unit = build_runtime_unit!(
                prefix: Some(KILO),
                unit: Some(METER),
                factor: Some(123),
                annotation: Some("things".to_string())
            );

            assert_eq!(RuntimeUnit::try_from(component).unwrap(), expected_unit);
        }

        #[test]
        fn test_nested_term_ok() {
            let component = Component::NestedTerm(Term::Basic(Box::new(Component::Factor(123))));

            let expected_unit =
                build_runtime_unit!(rhs: Some(Box::new(build_runtime_unit!(factor: Some(123)))),);

            assert_eq!(RuntimeUnit::try_from(component).unwrap(), expected_unit);
        }

        #[test]
        fn test_nested_term_error() {
            let component = Component::NestedTerm(Term::Basic(Box::new(Component::Annotatable(
                Annotatable(SimpleUnit(None, None, AtomSymbol("!@#$")), None),
            ))));

            let expected_error = Error::UnknownAtomSymbol("!@#$".to_string());

            assert_eq!(
                RuntimeUnit::try_from(component).unwrap_err(),
                expected_error
            );
        }

        mod annotatable {
            use super::*;

            #[test]
            fn test_no_factor_valid_atom() {
                let component = Component::Annotatable(Annotatable(
                    SimpleUnit(None, None, AtomSymbol("m")),
                    None,
                ));

                let expected_unit = build_runtime_unit!(unit: Some(METER));

                assert_eq!(RuntimeUnit::try_from(component).unwrap(), expected_unit);
            }

            #[test]
            fn test_no_factor_invalid_atom() {
                let component = Component::Annotatable(Annotatable(
                    SimpleUnit(None, None, AtomSymbol("!@#$")),
                    None,
                ));

                let expected_error = Error::UnknownAtomSymbol("!@#$".to_string());

                assert_eq!(
                    RuntimeUnit::try_from(component).unwrap_err(),
                    expected_error
                );
            }

            #[test]
            fn test_no_factor_valid_prefix_and_atom() {
                let component = Component::Annotatable(Annotatable(
                    SimpleUnit(None, Some(PrefixSymbol("k")), AtomSymbol("m")),
                    None,
                ));

                let expected_unit = build_runtime_unit!(prefix: Some(KILO), unit: Some(METER));

                assert_eq!(RuntimeUnit::try_from(component).unwrap(), expected_unit);
            }

            #[test]
            fn test_no_factor_valid_prefix_but_invalid_atom() {
                let component = Component::Annotatable(Annotatable(
                    SimpleUnit(None, Some(PrefixSymbol("k")), AtomSymbol("!@#$")),
                    None,
                ));

                let expected_error = Error::UnknownAtomSymbol("!@#$".to_string());

                assert_eq!(
                    RuntimeUnit::try_from(component).unwrap_err(),
                    expected_error
                );
            }

            #[test]
            fn test_no_factor_invalid_prefix_but_valid_atom() {
                let component = Component::Annotatable(Annotatable(
                    SimpleUnit(None, Some(PrefixSymbol("!@#$")), AtomSymbol("m")),
                    None,
                ));

                let expected_error = Error::UnknownPrefixSymbol("!@#$".to_string());

                assert_eq!(
                    RuntimeUnit::try_from(component).unwrap_err(),
                    expected_error
                );
            }

            #[test]
            fn test_with_factor_valid_prefix_and_atom() {
                let component = Component::Annotatable(Annotatable(
                    SimpleUnit(Some(123), Some(PrefixSymbol("k")), AtomSymbol("m")),
                    None,
                ));

                let expected_unit =
                    build_runtime_unit!(prefix: Some(KILO), unit: Some(METER), factor: Some(123));

                assert_eq!(RuntimeUnit::try_from(component).unwrap(), expected_unit);
            }
        }
    }
}
