use pest::{iterators::Pairs, pratt_parser::PrattParser};

use crate::term::Exponent;

use super::{Annotatable, Parse, Rule};

#[derive(Debug, PartialEq)]
pub(in crate::unit) enum Component<'i> {
    Annotatable {
        factor: Option<&'i str>,
        annotatable: Annotatable<'i>,
        annotation: Option<&'i str>,
    },
    Factor {
        factor: &'i str,
        annotation: Option<&'i str>,
    },
    Annotation(&'i str),
    // ParenTerm(Box<Term<'i>>),
}

impl<'a> Parse<'a> for Component<'a> {
    fn parse(pairs: Pairs<'a, Rule>, pratt: &PrattParser<Rule>) -> Self {
        fn parse_annotation(mut pairs: Pairs<'_, Rule>) -> &str {
            pairs
                .next()
                .map_or_else(|| unreachable!(), |pair| pair.as_str())
        }

        pratt
            .map_primary(|primary| match primary.as_rule() {
                Rule::annotatable => Component::Annotatable {
                    factor: None,
                    annotatable: Annotatable::parse(primary.into_inner(), pratt),
                    annotation: None,
                },
                Rule::annotation_component => {
                    Component::Annotation(parse_annotation(primary.into_inner()))
                }
                Rule::factor_component => Component::Factor {
                    factor: primary.as_str(),
                    annotation: None,
                },
                rule => unreachable!("expected inner component, found {rule:?}"),
            })
            .map_prefix(|op, rhs| match op.as_rule() {
                Rule::factor => match rhs {
                    Component::Annotatable {
                        annotatable,
                        annotation,
                        ..
                    } => Component::Annotatable {
                        factor: Some(op.as_str()),
                        annotatable,
                        annotation,
                    },
                    _ => unreachable!(),
                },
                rule => unreachable!("expected factor, found {rule:?}"),
            })
            .map_postfix(|annotatable, op| match op.as_rule() {
                Rule::annotation_string => match annotatable {
                    Component::Annotatable {
                        factor,
                        annotatable,
                        ..
                    } => Component::Annotatable {
                        factor,
                        annotatable,
                        annotation: Some(op.as_str()),
                    },
                    Component::Factor { factor, .. } => Component::Factor {
                        factor,
                        annotation: Some(op.as_str()),
                    },
                    Component::Annotation(_) => unreachable!(),
                },
                rule => unreachable!("expected annotation_string, found {rule:?}"),
            })
            .parse(pairs)
    }
}

impl TryFrom<Component<'_>> for crate::Term {
    type Error = crate::parser::Error;

    fn try_from(component: Component<'_>) -> Result<Self, Self::Error> {
        match component {
            Component::Annotatable {
                factor,
                annotatable,
                annotation,
            } => {
                let maybe_factor = match factor {
                    Some(string_value) => {
                        let integer = string_value.parse()?;
                        Some(integer)
                    }
                    None => None,
                };

                let (prefix, atom, exponent) = {
                    match annotatable {
                        Annotatable::SimpleUnitExponent {
                            simple_unit,
                            exponent,
                        } => {
                            let (prefix, atom) = simple_unit.into();
                            (prefix, atom, Some(Exponent::try_from(exponent)?))
                        }
                        Annotatable::SimpleUnit(simple_unit) => {
                            let (prefix, atom) = simple_unit.into();
                            (prefix, atom, None)
                        }
                    }
                };

                Ok(Self {
                    factor: maybe_factor,
                    prefix,
                    atom,
                    exponent,
                    // TODO: Remove .to_string() with PLCC-291
                    annotation: annotation.map(ToString::to_string),
                })
            }
            Component::Factor { factor, annotation } => Ok(Self {
                factor: Some(factor.parse()?),
                prefix: None,
                atom: None,
                exponent: None,
                // TODO: Remove .to_string() with PLCC-291
                annotation: annotation.map(ToString::to_string),
            }),
            Component::Annotation(annotation) => Ok(Self {
                factor: None,
                prefix: None,
                atom: None,
                exponent: None,
                // TODO: Remove .to_string() with PLCC-291
                annotation: Some(annotation.to_string()),
            }),
        }
    }
}
