pub mod decomposable;
pub(super) mod simple;

pub use self::decomposable::Decomposable;
pub(super) use self::simple::Decomposer as SimpleDecomposer;

fn build_string(acc: Option<String>, term_string: String) -> Option<String> {
    match acc {
        Some(mut a) => {
            let new_string = format!(".{}", term_string);

            a.push_str(&new_string);
            a.shrink_to_fit();

            Some(a)
        }
        None => Some(term_string),
    }
}
