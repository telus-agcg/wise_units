use nom::{
    branch::alt, bytes::complete::is_not, character::complete::char, sequence::delimited, IResult,
};

use crate::term::Factor;

use super::{annotatable::Annotatable, term::Term};

#[derive(Debug, PartialEq)]
pub(super) enum Component<'i> {
    AnnotatableAnnotation((Annotatable, &'i [u8])),
    Annotatable(Annotatable),
    Factor(Factor),
    Annotation(&'i [u8]),
    Term(Box<Term<'i>>),
}

pub(super) fn parse(input: &[u8]) -> IResult<&[u8], Component<'_>> {
    alt((
        parse_annotatable_annotation,
        parse_annotatable_component,
        parse_annotation_component,
        parse_factor_component,
        parse_nested_term,
    ))(input)
}

fn parse_annotatable_annotation(input: &[u8]) -> IResult<&[u8], Component<'_>> {
    let (tail, annotatable) = super::annotatable::parse(input)?;
    let (tail, annotation) = parse_annotation(tail)?;

    Ok((
        tail,
        Component::AnnotatableAnnotation((annotatable, annotation)),
    ))
}

fn parse_annotatable_component(input: &[u8]) -> IResult<&[u8], Component<'_>> {
    let (tail, annotatable) = super::annotatable::parse(input)?;

    Ok((tail, Component::Annotatable(annotatable)))
}

fn parse_annotation_component(input: &[u8]) -> IResult<&[u8], Component<'_>> {
    let (tail, annotation) = parse_annotation(input)?;

    Ok((tail, Component::Annotation(annotation)))
}

fn parse_annotation(input: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(char('{'), is_not("}"), char('}'))(input)
}

fn parse_factor_component(input: &[u8]) -> IResult<&[u8], Component<'_>> {
    let (tail, factor) = parse_factor(input)?;

    Ok((tail, Component::Factor(factor)))
}

fn parse_factor(input: &[u8]) -> IResult<&[u8], Factor> {
    nom::character::complete::u32(input)
}

fn parse_nested_term(input: &[u8]) -> IResult<&[u8], Component<'_>> {
    let (tail, term) = delimited(char('{'), super::term::parse, char('}'))(input)?;

    Ok((tail, Component::Term(Box::new(term))))
}
