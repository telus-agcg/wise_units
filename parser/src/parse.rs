use crate::{
    atom_collection::AtomCollection, prefix_collection::PrefixCollection,
    symbol_collection::SymbolCollection,
};

// A = value in the key/value pair for things that we'll want to fetch on successful parse. In
// practice, this is the atom.
//
// AC = the collection that contains the key/value pairs that we'll use to look up the value for
// on a successful parse. In practice, this is a collection that has keys that are unit strings
// (ex. "m" for meter, etc) and values that are the types that represent the unit.
//
// T = the type of token returned containing its parse tree.
//
pub trait Parse<'input, 'a, T> {
    fn parse(
        input: &'input str,
        prefixes: &'a PrefixCollection,
        atoms: &'a AtomCollection,
    ) -> ParseResult<'input, T>;
}

/// Trait for parsing specifically either an Atom or a Prefix.
///
#[allow(clippy::module_name_repetitions)]
pub trait ParseSymbol<'input, 'a, C, V>
where
    C: SymbolCollection<'a, V>,
{
    fn parse_symbol(input: &'input str, collection: &'a C) -> ParseResult<'input, &'a V>;
}

#[allow(clippy::module_name_repetitions)]
pub type ParseResult<'input, O> = Result<(O, &'input str), &'input str>;
