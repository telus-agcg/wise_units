use super::Component;

#[derive(Debug, PartialEq, Clone)]
pub enum Term<'input> {
    Combined(Box<Component<'input>>, Separator, Box<Term<'input>>),
    Basic(Box<Component<'input>>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Separator {
    Dot,
    Slash,
}
