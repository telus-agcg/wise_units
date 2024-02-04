pub trait SymbolCollection<'a, V> {
    // Return: if Some, returns the type that was found, and the length of its str representation.
    // (returning the latter saves us from having call atom.primary_code().len() later)
    fn find_match(&'a self, input: &str) -> Option<(&'a V, usize)>;
}
