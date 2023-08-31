use crate::{
    parser::definition::Definition, v2::type_traits::Prefix as TPrefix, Classification, Prefix,
    UcumSymbol,
};

impl<'a> TPrefix<'a, f64> for Prefix {
    type String = &'static str;
    type Names = Vec<&'static str>;
    type Class = Classification;
    type Definition = Definition;

    fn primary_code(&self) -> Self::String {
        // Just delegate to existing implementation.
        UcumSymbol::primary_code(self)
    }

    fn secondary_code(&self) -> Option<Self::String> {
        // Just delegate to existing implementation.
        UcumSymbol::secondary_code(self)
    }

    fn print_symbol(&self) -> Option<Self::String> {
        // Just delegate to existing implementation.
        UcumSymbol::print_symbol(self)
    }

    fn names(&self) -> Self::Names {
        // Just delegate to existing implementation.
        UcumSymbol::names(self)
    }

    fn class(&self) -> Self::Class {
        // Just delegate to existing implementation.
        UcumSymbol::classification(self)
    }

    fn definition(&self) -> Self::Definition {
        Definition::new_non_dimensional(UcumSymbol::definition_value(self))
    }
}
