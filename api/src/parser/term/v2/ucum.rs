use crate::{v2::behavior_traits::dim, Term};

impl dim::Dimensionable for Term {
    type Output = crate::Composition;

    fn dim(&self) -> Self::Output {
        // Just delegate to existing impl for now.
        crate::Composable::composition(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Atom;

    #[test]
    fn dim_test() {
        let term = term!(Meter);

        assert_eq!(
            dim::Dimensionable::dim(&term),
            crate::Composition::new(crate::Dimension::Length, 1)
        );
    }
}
