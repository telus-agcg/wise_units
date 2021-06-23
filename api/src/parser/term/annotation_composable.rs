use crate::parser::annotation_composition::{AnnotationComposable, AnnotationComposition};
use crate::parser::Term;

/// Similar to `Composable`, this is only to allow for checking compatibility on `Unit`s whose
/// `Term`s have annotations. For those cases, we want to be able to ensure that, for example,
/// `m{foo}` is not comparable to `m{bar}`. This implementation treats each `Term`s `annotation`
/// as its own `Dimension` of sorts, allowing `m2{foo}/m{foo}` to be comparable to `m{foo}`, since
/// they have equivalent `AnnotationComposable`s.
///
impl<'a> AnnotationComposable for &'a [Term] {
    fn annotation_composition(self) -> Option<AnnotationComposition> {
        let mut map = self
            .iter()
            .filter_map(|term| {
                term.annotation
                    .as_ref()
                    .map(|annotation| (annotation.clone(), term.exponent.unwrap_or(1)))
            })
            .fold(AnnotationComposition::new(), |mut map, (key, exponent)| {
                let _ = map
                    .entry(key)
                    .and_modify(|entry| *entry += exponent)
                    .or_insert(exponent);

                map
            });

        // Filter out things that have no values
        map.retain(|_annotation, exponent| *exponent != 0);

        if map.is_empty() {
            None
        } else {
            Some(map)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::annotation_composition::{AnnotationComposable, AnnotationComposition};
    use crate::parser::Atom;
    use crate::parser::Term;

    mod without_annotations {
        use super::*;

        #[test]
        fn validate_no_atom() {
            let terms = [term!()];
            assert!(terms.annotation_composition().is_none());
        }

        #[test]
        fn validate_with_atom() {
            let terms = [term!(Meter)];
            assert!(terms.annotation_composition().is_none());
        }

        #[test]
        fn validate_with_atom_and_exponent() {
            let terms = [term!(Meter, exponent: 2), term!(Second, exponent: -1)];
            assert!(terms.annotation_composition().is_none());
        }
    }

    mod with_annotations {
        use super::*;

        #[test]
        fn validate_no_atom() {
            let terms = [term!(annotation: "foo".to_string())];

            let mut annotation_composition = AnnotationComposition::new();
            let _ = annotation_composition.insert("foo".to_string(), 1);

            assert_eq!(terms.annotation_composition(), Some(annotation_composition));
        }

        #[test]
        fn validate_with_atom() {
            let terms = [term!(Meter, annotation: "foo".to_string())];

            let mut annotation_composition = AnnotationComposition::new();
            let _ = annotation_composition.insert("foo".to_string(), 1);

            assert_eq!(terms.annotation_composition(), Some(annotation_composition));
        }

        #[test]
        fn validate_with_atom_and_negative_exponent() {
            let terms = [term!(Meter, exponent: -2, annotation: "foo".to_string())];

            let mut annotation_composition = AnnotationComposition::new();
            let _ = annotation_composition.insert("foo".to_string(), -2);

            assert_eq!(terms.annotation_composition(), Some(annotation_composition));
        }

        #[test]
        fn validate_with_atoms_and_positive_and_negative_exponents() {
            let terms = [
                term!(Gram, exponent: 3, annotation: "bar".to_string()),
                term!(Meter, exponent: -2, annotation: "foo".to_string()),
            ];
            let mut annotation_composition = AnnotationComposition::new();
            let _ = annotation_composition.insert("bar".to_string(), 3);
            let _ = annotation_composition.insert("foo".to_string(), -2);
            assert_eq!(terms.annotation_composition(), Some(annotation_composition));
        }

        #[test]
        fn validate_with_atoms_cancelling_exponents() {
            let terms = [
                term!(Meter, exponent: 2, annotation: "foo".to_string()),
                term!(Meter, exponent: -2, annotation: "foo".to_string()),
            ];
            assert!(terms.annotation_composition().is_none());
        }
    }
}
