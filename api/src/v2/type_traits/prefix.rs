pub trait Prefix {
    type String;
    type Names;
    type Class;
    type Definition;

    fn primary_code(&self) -> Self::String;
    fn secondary_code(&self) -> Option<Self::String>;
    fn print_symbol(&self) -> Option<Self::String>;
    fn names(&self) -> Self::Names;

    fn class(&self) -> Self::Class;

    fn definition(&self) -> Self::Definition;
}
