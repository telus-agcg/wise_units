use std::collections::HashMap;

pub type AnnotationComposition = HashMap<String, i32>;

/// Similar to `Composable`, this is only to allow for checking compatibility on `Unit`s that have
/// annotations. For those cases, we want to be able to ensure that, for example, `m{foo}` is not
/// comparable to `m{bar}`.
///
pub(crate) trait AnnotationComposable {
    fn annotation_composition(self) -> Option<AnnotationComposition>;
}
