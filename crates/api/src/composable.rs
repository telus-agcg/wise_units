use crate::Composition;

/// A simple trait for getting a type to define how its `Composition` should be built.  Typically
/// `Composition`s are used to determine the compatibility with another type; so long as each type
/// provides a `Composition`, they can be compared for compatibility.
///
pub trait Composable {
    fn composition(&self) -> Composition;
}

/// This trait was extracted from a function in `unit::term_reducing` to allow for specifying how
/// `Term`s within a `Unit` should be combined. Terms that are "composably equal" are terms that,
/// aside from any associated exponents, could be considered the same. Note that "same" is very
/// strict, where it does not mean "equal" in the scalar sense (like "1000m" == "km"), but rather
/// in the sense of the underlying `Term`. This is mainly to handle cases such as where a `Term`'s
/// `factor` can be `1`, in which case just because it has a factor, it should be considered equal
/// to a `Term` that's the same, but minus a factor.
///
/// Some basic examples:
///
/// - `m` and `m3` are `ComposablyEq` because after stripping away exponents, they're both `m`.
/// - `m` and `km` are _not_ `ComposablyEq` because the RHS term has a `Prefix`.
/// - `m` and `1m` are `ComposablyEq` because the `1` in `1m` has no effect on the scalar value of
///   the unit.
/// - `m` and `2m` are _not_ `ComposablyEq` because the `2` in `2m` has effect on the scalar value
///   of the unit.
/// - `m` and `[ft_i]` are _not_ `ComposablyEq` because they're different `Atoms`.
///
/// We could take this further and, say, account for the effect on both factors and exponents of a
/// `Term` (allowing us to treat, say `4m` the same as `2m2`), but currently we do not.
///
/// This becomes trickier when we talk about units with annotations. Because, contrary to the spec,
/// we treat units with annotations differently than the same unit without an annotation, we must
/// also take these in account when determining to remove a term when reducing.
///
/// Some examples:
///
/// - `m{foo}` and `m` are _not_ `ComposablyEq` because the LHS has an annotation and RHS does not.
/// - `m{foo}` and `m2{foo}` are `ComposablyEq` because they only differ by the RHS exponent.
/// - `m{foo}` and `1m{foo}` are `ComposablyEq` because the 1 on the RHS has to scalar impact.
/// - `m{foo}` and `1m2{foo}` are `ComposablyEq` because the 1 on the RHS has to scalar impact and
///   the exponent bears no impact on this operation.
/// - `m2{foo}` and `m2` are _not_ `ComposablyEq`.
/// - `km{foo}` and `km` are _not_ `ComposablyEq`.
///
pub(crate) trait ComposablyEq<T> {
    fn composably_eq(&self, rhs: &T) -> Option<crate::term::Exponent>;
}
