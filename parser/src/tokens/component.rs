use super::{Annotatable, Annotation, Term};

#[derive(Debug, PartialEq, Clone)]
pub enum Component<'input> {
    AnnotatedAnnotatable(Annotatable, Annotation<'input>),
    Annotatable(Annotatable),
    Annotation(Annotation<'input>),
    Factor(u32),
    NestedTerm(Term<'input>),
}
