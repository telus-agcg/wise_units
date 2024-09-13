use std::borrow::Cow;

use crate::{Composable, IsCompatibleWith, Term};

use super::annotation_composable::AnnotationComposable;

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
impl IsCompatibleWith for Term {
    fn is_compatible_with(&self, rhs: &Self) -> bool {
        match self {
            Self::Annotation(inner) => inner.is_compatible_with(rhs),
            Self::Atom(inner) => inner.is_compatible_with(rhs),
            Self::AtomAnnotation(inner) => inner.is_compatible_with(rhs),
            Self::AtomExponent(inner) => inner.is_compatible_with(rhs),
            Self::AtomExponentAnnotation(inner) => inner.is_compatible_with(rhs),
            Self::PrefixAtom(inner) => inner.is_compatible_with(rhs),
            Self::PrefixAtomAnnotation(inner) => inner.is_compatible_with(rhs),
            Self::PrefixAtomExponent(inner) => inner.is_compatible_with(rhs),
            Self::PrefixAtomExponentAnnotation(inner) => inner.is_compatible_with(rhs),
            Self::Factor(_) => rhs.composition().is_dimless() && rhs.annotation().is_none(),
            Self::FactorAnnotation(inner) => inner.is_compatible_with(rhs),
            Self::FactorExponent(inner) => inner.is_compatible_with(rhs),
            Self::FactorExponentAnnotation(inner) => inner.is_compatible_with(rhs),
            Self::FactorAtom(inner) => inner.is_compatible_with(rhs),
            Self::FactorAtomAnnotation(inner) => inner.is_compatible_with(rhs),
            Self::FactorAtomExponent(inner) => inner.is_compatible_with(rhs),
            Self::FactorAtomExponentAnnotation(inner) => inner.is_compatible_with(rhs),
            Self::FactorPrefixAtom(inner) => inner.is_compatible_with(rhs),
            Self::FactorPrefixAtomAnnotation(inner) => inner.is_compatible_with(rhs),
            Self::FactorPrefixAtomExponent(inner) => inner.is_compatible_with(rhs),
            Self::FactorPrefixAtomExponentAnnotation(inner) => inner.is_compatible_with(rhs),
        }
    }
}

impl<'a> IsCompatibleWith for Cow<'a, [Term]> {
    fn is_compatible_with(&self, rhs: &Self) -> bool {
        if self.composition() != rhs.composition() {
            return false;
        }

        // TODO: Since we only care about keys when both sides have annotations, we probably don't
        // need to return the exponents map from `annotation_composition()`.
        match (self.annotation_composition(), rhs.annotation_composition()) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(l), Some(r)) => l.keys().eq(r.keys()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::is_compatible_with::IsCompatibleWith;

    mod without_annotations {
        use super::*;

        #[test]
        fn validate_term() {
            let lhs = term!(Meter);
            let rhs = term!(Kilo, Meter);
            assert!(lhs.is_compatible_with(&rhs));
        }

        #[test]
        fn validate_term_with_factor() {
            let lhs = term!(Meter);
            let rhs = term!(Kilo, Meter, factor: 20);
            assert!(lhs.is_compatible_with(&rhs));
        }

        #[test]
        fn validate_term_with_factor_and_exponent() {
            let lhs = term!(Meter);
            let rhs = term!(Kilo, Meter, factor: 20, exponent: 2);
            assert!(!lhs.is_compatible_with(&rhs));
        }

        #[test]
        fn validate_terms() {
            let lhs = terms![term!(Meter)];
            let rhs = terms![term!(Kilo, Meter)];
            assert!(lhs.is_compatible_with(&rhs));
        }

        #[test]
        fn validate_terms_with_factor() {
            let lhs = terms![term!(Meter)];
            let rhs = terms![term!(Kilo, Meter, factor: 20)];
            assert!(lhs.is_compatible_with(&rhs));
        }

        #[test]
        fn validate_terms_with_factor_and_exponent() {
            let lhs = terms![term!(Meter)];
            let rhs = terms![term!(Kilo, Meter, factor: 20, exponent: 2)];
            assert!(!lhs.is_compatible_with(&rhs));
        }
    }

    mod with_annotations {
        use super::*;

        #[test]
        fn validate_term() {
            let m = term!(Meter, annotation: "stuff");
            let km_stuff = term!(Kilo, Meter, annotation: "stuff");
            assert!(m.is_compatible_with(&km_stuff));

            // Different annotation
            let km_pants = term!(Kilo, Meter, annotation: "pants");
            assert!(!m.is_compatible_with(&km_pants));

            // No annotation
            let km_no_annotation = term!(Kilo, Meter);
            assert!(!m.is_compatible_with(&km_no_annotation));
        }

        #[test]
        fn validate_terms() {
            let m = terms![term!(Meter, annotation: "stuff")];
            let km_stuff = terms![term!(Kilo, Meter, annotation: "stuff")];
            assert!(m.is_compatible_with(&km_stuff));

            // Different annotation
            let km_pants = terms![term!(Kilo, Meter, annotation: "pants")];
            assert!(!m.is_compatible_with(&km_pants));

            // No annotation
            let km_no_annotation = terms![term!(Kilo, Meter)];
            assert!(!m.is_compatible_with(&km_no_annotation));
        }

        #[test]
        fn validate_atomless_term() {
            let lhs_tree = term!(annotation: "tree");
            let rhs_tree = term!(annotation: "tree");
            assert!(lhs_tree.is_compatible_with(&rhs_tree));

            // Different annotation
            let rhs_plant = term!(annotation: "plant");
            assert!(!lhs_tree.is_compatible_with(&rhs_plant));
        }
    }
}
