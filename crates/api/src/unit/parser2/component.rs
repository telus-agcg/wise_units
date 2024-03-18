use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::char,
    combinator::opt,
    sequence::{delimited, pair, tuple},
    IResult,
};

use crate::term::Factor;

use super::{annotatable::Annotatable, term::Term};

#[derive(Debug, PartialEq)]
pub(in crate::unit) enum Component<'i> {
    Annotatable {
        factor: Option<Factor>,
        annotatable: Annotatable,
        annotation: Option<&'i [u8]>,
    },
    Factor {
        factor: Factor,
        annotation: Option<&'i [u8]>,
    },
    Annotation(&'i [u8]),
    Term(Box<Term<'i>>),
}

impl TryFrom<Component<'_>> for crate::Unit {
    type Error = ();

    fn try_from(value: Component<'_>) -> Result<Self, Self::Error> {
        match value {
            Component::Annotatable {
                factor,
                annotatable,
                annotation,
            } => {
                let s = annotation.map(|a| std::str::from_utf8(a).unwrap().to_string());

                let (prefix, atom, exponent) = annotatable.extract_for_term();

                Ok(Self::new(vec![crate::Term {
                    factor,
                    prefix,
                    atom: Some(atom),
                    exponent,
                    annotation: s,
                }]))
            }
            Component::Factor { factor, annotation } => {
                let s = annotation.map(|a| std::str::from_utf8(a).unwrap().to_string());

                Ok(Self::new(vec![crate::Term {
                    factor: Some(factor),
                    prefix: None,
                    atom: None,
                    exponent: None,
                    annotation: s,
                }]))
            }
            Component::Annotation(annotation) => {
                let s = std::str::from_utf8(annotation).unwrap();
                Ok(Self::new(vec![term!(annotation: s.to_string())]))
            }
            Component::Term(t) => Self::try_from(*t),
        }
    }
}

pub(super) fn parse(input: &[u8]) -> IResult<&[u8], Component<'_>> {
    alt((
        parse_annotatable_component,
        parse_factor_component,
        parse_annotation_component,
        parse_nested_term,
    ))(input)
}

fn parse_annotatable_component(input: &[u8]) -> IResult<&[u8], Component<'_>> {
    if let Ok((tail, (factor, annotatable, annotation))) = tuple((
        opt(parse_factor),
        super::annotatable::parse,
        opt(parse_annotation),
    ))(input)
    {
        Ok((
            tail,
            Component::Annotatable {
                factor,
                annotatable,
                annotation,
            },
        ))
    // Since parsing atoms like `10*` can greedily match the `10` part as a factor, we retry
    // parsing here without trying the factor on error.
    //
    } else {
        let (tail, (annotatable, annotation)) =
            tuple((super::annotatable::parse, opt(parse_annotation)))(input)?;

        Ok((
            tail,
            Component::Annotatable {
                factor: None,
                annotatable,
                annotation,
            },
        ))
    }
}

fn parse_factor_component(input: &[u8]) -> IResult<&[u8], Component<'_>> {
    let (tail, (factor, annotation)) = pair(parse_factor, opt(parse_annotation))(input)?;

    Ok((tail, Component::Factor { factor, annotation }))
}

fn parse_factor(input: &[u8]) -> IResult<&[u8], Factor> {
    nom::character::complete::u32(input)
}

fn parse_annotation_component(input: &[u8]) -> IResult<&[u8], Component<'_>> {
    let (tail, annotation) = parse_annotation(input)?;

    Ok((tail, Component::Annotation(annotation)))
}

fn parse_annotation(input: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(char('{'), is_not("}"), char('}'))(input)
}

fn parse_nested_term(input: &[u8]) -> IResult<&[u8], Component<'_>> {
    let (tail, term) = delimited(char('{'), super::term::parse, char('}'))(input)?;

    Ok((tail, Component::Term(Box::new(term))))
}
