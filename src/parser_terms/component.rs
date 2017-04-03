use atom::Dimension;
use parser_terms::{Annotatable, Annotation, Factor, Term};
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, PartialEq)]
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

    pub fn scalar(&self, magnitude: f64) -> f64 {
        match *self {
            Component::Term(ref box_term) => {
                let ref term = *box_term;
                term.scalar(magnitude)
            },
            Component::Factor(_) => 1.0,
            Component::Annotation(_) => 1.0,
            Component::Annotatable(ref annotatable) => {
                annotatable.scalar(magnitude)
            },
            Component::AnnotatedAnnotatable(ref annotatable, ref _annotation) => {
                annotatable.scalar(magnitude)
            }
        }
    }

    pub fn scalar_default(&self) -> f64 {
        self.scalar(1.0)
    }

    pub fn magnitude(&self, scalar: f64) -> f64 {
        match *self {
            Component::Term(ref box_term) => {
                let ref term = *box_term;
                term.magnitude(scalar)
            },
            Component::Factor(_) => 1.0,
            Component::Annotation(_) => 1.0,
            Component::Annotatable(ref annotatable) => {
                annotatable.magnitude(scalar)
            },
            Component::AnnotatedAnnotatable(ref annotatable, ref _annotation) => {
                annotatable.magnitude(scalar)
            }
        }
    }

    pub fn magnitude_default(&self) -> f64 {
        self.magnitude(1.0)
    }
}

impl<'a> fmt::Display for Component<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Component::AnnotatedAnnotatable(ref annotatable, ref annotation) => {
                write!(f, "{}{{{}}}", annotatable, annotation)
            },
            Component::Annotatable(ref annotatable) => { write!(f, "{}", annotatable) },
            Component::Annotation(ref annotation) => { write!(f, "{}", annotation) },
            Component::Factor(ref factor) => { write!(f, "{}", factor) },
            Component::Term(ref box_term) => {
                let ref term = *box_term;
                write!(f, "{}", term)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Component;
    use atom::base::Meter;
    use parser::parse_Component;
    use parser_terms::{Annotatable, Annotation, Exponent, Factor, SimpleUnit, Term, UnitSign};
    use prefix::PREFIXES;

    #[test]
    fn validate_component_with_annotations() {
        let annotation = Annotation("%vol");

        assert_eq!(
            parse_Component("km-10{%vol}").unwrap(),
            Component::AnnotatedAnnotatable(make_annotatable(), annotation)
            );

        assert_eq!(
            parse_Component("km-10").unwrap(),
            Component::Annotatable(make_annotatable())
            );

        let annotation = Annotation("wet'tis.");

        assert_eq!(
            parse_Component("{wet'tis.}").unwrap(),
            Component::Annotation(annotation)
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

    fn make_annotatable() -> Annotatable {
        let simple_unit = SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), Box::new(Meter));
        let negative_exp = Exponent(UnitSign::Negative, 10);

        Annotatable::UnitWithPower(simple_unit, negative_exp)
    }
}
