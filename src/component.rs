use atom::Dimension;
use parser_terms::{Annotatable, Annotation, Factor};
use std::collections::BTreeMap;
use term::Term;

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
}

// #[cfg(test)]
// mod tests {
//     use super::Component;
// }
