pub(super) mod decomposable;
pub(super) mod reduction;
pub(super) mod simple;

pub(super) use self::decomposable::Decomposable;
pub(super) use self::reduction::Decomposer as ReductionDecomposer;
pub(super) use self::simple::Decomposer as SimpleDecomposer;

fn build_string(mut acc: String, term_string: String) -> String {
    let new_string = if acc.is_empty() {
        term_string
    } else {
        format!(".{}", term_string)
    };
    acc.push_str(&new_string);

    acc
}
