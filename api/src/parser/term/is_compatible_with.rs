use parser::{Composable, IsCompatibleWith, Term};

/// In order to enforce compatibility on "non-units" (ex. `{each}`, `{total}`, `{heartbeats}`),
/// `Term`s need to compare their annotations along with their `Composition`s. In practice, and
/// taking from [UCUM's Examples for some
/// non-units](http://unitsofmeasure.org/ucum.html#section-Examples-for-some-Non-Units.), if the
/// following are measurements we want to say :
///
/// * "100 grams of creatine" are compatible with "200 grams of creatine"
/// * "100 grams of creatine" are _not_ compatible with "200 grams of wet tissue"
/// * "100 grams" are _not_ compatible with "200 grams of creatine"
///
/// ...or when used without a Unit Atom...
///
/// * "100 {tree}" is compatible with "200 {tree}"
/// * "100 {tree}" is _not_ compatible with "200 {cell}"
/// * "100" is _not_ compatible with "200 {cell}"
///
/// These distinctions are important as unit-compatibility is what determines if/when/how
/// measurements can be:
///
/// * converted to another unit
/// * added to, subtracted from, multiplied, divided
/// * checked for equality
/// * sorted
///
/// ...and thus the annotation plays an important role in that.
///
/// More info at nih.gov, [here](https://ucum.nlm.nih.gov/ucum-service.html) (look under the
/// "Annotations" section for starters).
///
impl<'a, 'b> IsCompatibleWith<&'b Term> for &'a Term {
    fn is_compatible_with(self, rhs: &'b Term) -> bool {
        self.composition() == rhs.composition() && self.annotation == rhs.annotation
    }
}

#[cfg(test)]
mod tests {
    use parser::{Atom, IsCompatibleWith, Prefix, Term};

    #[test]
    fn validate_no_annotations() {
        let lhs = term!(Meter);
        let rhs = term!(Kilo, Meter);
        assert!(lhs.is_compatible_with(&rhs));

        let rhs = term!(Kilo, Meter, factor: 20);
        assert!(lhs.is_compatible_with(&rhs));

        let rhs = term!(Kilo, Meter, factor: 20, exponent: 2);
        assert!(!lhs.is_compatible_with(&rhs));
    }

    #[test]
    fn validate_with_annotations() {
        let lhs = term!(Meter, annotation: "stuff".to_string());
        let rhs = term!(Kilo, Meter, annotation: "stuff".to_string());
        assert!(lhs.is_compatible_with(&rhs));

        // Different annotation
        let rhs = term!(Kilo, Meter, annotation: "pants".to_string());
        assert!(!lhs.is_compatible_with(&rhs));

        // No annotation
        let rhs = term!(Kilo, Meter);
        assert!(!lhs.is_compatible_with(&rhs));
    }
}
