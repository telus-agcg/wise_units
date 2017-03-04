use atom::Dimension;
use parser_terms::{Annotatable, Annotation, Factor, Term};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Component<'a> {
    AnnotatedAnnotatable(Annotatable, Annotation<'a>),
    Annotatable(Annotatable),
    Annotation(Annotation<'a>),
    Factor(Factor),
    Term(Box<Term<'a>>),
}

impl<'a> Component<'a> {
    // pub fn factor(&self) -> u32 {
    //     match *self {
    //         Component::Factor(ref factor) => { factor.0 as u32 },
    //         Component::Term(ref box_term) => {
    //             let ref term = *box_term;
    //             term.factor() as u32
    //         },
    //         _ => 1
    //     }
    // }

    pub fn composition(&self) -> BTreeMap<Dimension, i32> {
        match *self {
            Component::Annotatable(ref annotatable) => {
                annotatable.composition()
            },
            Component::Term(ref box_term) => {
                let ref term = *box_term;
                term.composition()
            },
            _ => { BTreeMap::new() }
        }
    }

    pub fn is_special(&self) -> bool {
        match *self {
            Component::AnnotatedAnnotatable(ref annotatable, ref __annotation) => {
                annotatable.is_special()
            },
            Component::Annotatable(ref annotatable) => {
                annotatable.is_special()
            },
            _ => false
        }
    }

    pub fn prefix_scalar(&self) -> f64 {
        match *self {
            Component::AnnotatedAnnotatable(ref annotatable, ref __annotation) => {
                annotatable.prefix_scalar()
            },
            Component::Annotatable(ref annotatable) => {
                annotatable.prefix_scalar()
            },
            Component::Term(ref box_term) => {
                let ref term = *box_term;
                term.prefix_scalar()
            },
            _ => 1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Component;
    use atom::ATOMS;
    use parser::parse_Component;
    use parser_terms::{Annotatable, Annotation, Exponent, Factor, SimpleUnit, Term, UnitSign};
    use prefix::PREFIXES;

    #[test]
    fn validate_component_with_annotations() {
        let simple_unit = SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), ATOMS[0].clone());
        let negative_exp = Exponent(UnitSign::Negative, 10);
        let annotatable = Annotatable::UnitWithPower(simple_unit, negative_exp);
        let annotation = Annotation("%vol");

        assert_eq!(
            parse_Component("km-10{%vol}").unwrap(),
            Component::AnnotatedAnnotatable(annotatable.clone(), annotation.clone())
            );

        assert_eq!(
            parse_Component("km-10").unwrap(),
            Component::Annotatable(annotatable.clone())
            );

        let annotation = Annotation("wet'tis.");

        assert_eq!(
            parse_Component("{wet'tis.}").unwrap(),
            Component::Annotation(annotation.clone())
            );
    }

    #[test]
    fn validate_component_with_factor() {
        assert_eq!(
            parse_Component("123").unwrap(),
            Component::Factor(Factor(123))
            );
    }

    #[test]
    fn validate_component_with_term() {
        assert_eq!(
            parse_Component("(123)").unwrap(),
            Component::Term(Box::new(Term::Basic(Component::Factor(Factor(123)))))
            );
    }

    #[test]
    fn validate_prefix_scalar_with_annotated_annotatable() {
        let component = parse_Component("km-10{%vol}").unwrap();
        assert_eq!(component.prefix_scalar(), 1000.0);

        let component = parse_Component("m-10{%vol}").unwrap();
        assert_eq!(component.prefix_scalar(), 1.0);
    }

    #[test]
    fn validate_prefix_scalar_with_annotatable() {
        let component = parse_Component("km-10").unwrap();
        assert_eq!(component.prefix_scalar(), 1000.0);

        let component = parse_Component("m-10").unwrap();
        assert_eq!(component.prefix_scalar(), 1.0);
    }

    #[test]
    fn validate_prefix_scalar_with_term() {
        let component = parse_Component("(123)").unwrap();
        assert_eq!(component.prefix_scalar(), 1.0);
    }
}
