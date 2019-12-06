#[derive(Debug, PartialEq, Clone)]
pub struct MainTerm<'input> {
    pub leading_slash: bool,
    pub term: Term<'input>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Term<'input> {
    Combined(Box<Component<'input>>, TermSeparator, Box<Term<'input>>),
    Basic(Box<Component<'input>>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TermSeparator {
    Dot,
    Slash,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Component<'input> {
    AnnotatedAnnotatable(Annotatable<'input>, Annotation<'input>),
    Annotatable(Annotatable<'input>),
    Annotation(Annotation<'input>),
    Factor(u32),
    NestedTerm(Term<'input>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Annotatable<'input>(pub SimpleUnit<'input>, pub Option<i32>);

#[derive(Debug, PartialEq, Clone)]
pub struct Annotation<'input>(pub &'input str);

// Section 2.2
//
// §8 integer numbers
//
// ■2 Only a pure string of decimal digits (‘0’–‘9’) is interpreted as a number.
// If after one or more digits there is any non-digit character found that is
// valid for unit atoms, all the characters (including the digits) will be
// interpreted as a simple unit symbol.
//
// For example, the string “123” is a positive integer number while “12a” is a symbol.
//
#[derive(Debug, PartialEq, Clone)]
pub struct SimpleUnit<'input>(
    pub Option<u32>,
    pub Option<PrefixSymbol<'input>>,
    pub AtomSymbol<'input>,
);

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixSymbol<'input>(pub &'input str);

#[derive(Debug, PartialEq, Clone)]
pub struct AtomSymbol<'input>(pub &'input str);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Sign {
    Positive,
    Negative,
}
